use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType, GainNode, OscillatorNode};


#[wasm_bindgen]
pub struct FmOsc {
    ctx: AudioContext,
    /// The primary oscillator.  This will be the fundamental frequency
    primary: OscillatorNode,
    ///  modulates the primary oscillator's frequency
    fm_osc: OscillatorNode,
    /// volume control
    gain: GainNode,
    /// Amount of frequency modulation
    fm_gain: GainNode,
    /// The ratio between the primary frequency and the fm_osc frequency.
    ///
    /// Generally fractional values like 1/2 or 1/4 sound best
    fm_freq_ratio: f32,
    fm_gain_ratio: f32,
}

impl Drop for FmOsc {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
} 

#[wasm_bindgen]
impl FmOsc {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<FmOsc, JsValue> {
        let ctx = web_sys::AudioContext::new()?;

        // Create our web audio objects.
        let primary = ctx.create_oscillator()?;
        let fm_osc = ctx.create_oscillator()?;
        let gain = ctx.create_gain()?;
        let fm_gain = ctx.create_gain()?;

        // Some initial settings:
        primary.set_type(OscillatorType::Sine);
        primary.frequency().set_value(440.0); // A4 note
        gain.gain().set_value(0.0); // starts muted
        fm_gain.gain().set_value(0.0); // no initial frequency modulation

        fm_osc.set_type(OscillatorType::Sine);
        fm_osc.frequency().set_value(0.0);
        primary.connect_with_audio_node(&gain)?;
        gain.connect_with_audio_node(&ctx.destination())?;
        fm_osc.connect_with_audio_node(&fm_gain)?;
        fm_gain.connect_with_audio_param(&primary.frequency())?;

        // Start the oscillators!
        primary.start()?;
        fm_osc.start()?;

        Ok(FmOsc {
            ctx,
            primary,
            gain,
            fm_gain,
            fm_osc,
            fm_freq_ratio: 0.0,
            fm_gain_ratio: 0.0,
        })
    }

    /// Sets the gain for this oscillator, between 0.0 and 1.0.
    #[wasm_bindgen]
    pub fn set_gain(&self, mut gain: f32) {
        if gain > 1.0 {
            gain = 1.0;
        }
        if gain < 0.0 {
            gain = 0.0;
        }
        self.gain.gain().set_value(gain);
    }

    #[wasm_bindgen]
    pub fn set_primary_frequency(&self, freq: f32) {
        self.primary.frequency().set_value(freq);
        self.fm_osc.frequency().set_value(self.fm_freq_ratio * freq);
        self.fm_gain.gain().set_value(self.fm_gain_ratio * freq);
    }

    /// Converts a midi note to frequency
    fn mid_to_f(note: u8) -> f32 {
        27.5 * 2f32.powf((note as f32 - 21.0) / 12.0)
    }

    #[wasm_bindgen]
    pub fn set_note(&self, note: u8) {
        let freq = FmOsc::mid_to_f(note);
        self.set_primary_frequency(freq);
    }


    /// This should be between 0 and 1, though higher values are accepted.
    #[wasm_bindgen]
    pub fn set_fm_amount(&mut self, amt: f32) {
        self.fm_gain_ratio = amt;

        self.fm_gain
            .gain()
            .set_value(self.fm_gain_ratio * self.primary.frequency().value());
    }

    /// This should be between 0 and 1, though higher values are accepted.
    #[wasm_bindgen]
    pub fn set_fm_frequency(&mut self, amt: f32) {
        self.fm_freq_ratio = amt;
        self.fm_osc
            .frequency()
            .set_value(self.fm_freq_ratio * self.primary.frequency().value());
    }
}