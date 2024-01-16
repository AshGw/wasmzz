import * as wasm from "wasmzz";

try {
    wasm.canvas(); 
    let fm = null;
    document.getElementById("play")
    .addEventListener("click", () => {
    if (fm === null) {
        fm = new wasm.FmOsc();
        fm.set_note(50);
        fm.set_fm_frequency(0);
        fm.set_fm_amount(0);
        fm.set_gain(0.8);
    } else {
        fm.free();
        fm = null;
    }
    });

    document.getElementById("primary_input")
    .addEventListener("input", event => {
    if (fm) {
        fm.set_note(parseInt(event.target.value));
    }
    });

    document.getElementById("fm_freq")
    .addEventListener("input", event => {
    if (fm) {
        fm.set_fm_frequency(parseFloat(event.target.value));
    }
    });

    document.getElementById("fm_amount")
    .addEventListener("input", event => {
    if (fm) {
        fm.set_fm_amount(parseFloat(event.target.value));
    }
    });

    wasm.run_watermarker();
} catch (error) {
  console.error(error);
}
