@build:
    cargo install wasm-pack
    wasm-pack build --target bundler
    npm i
    npm run build

@serve:
    npm run serve

@clean:
    rm -rf Cargo.lock pkg target node_modules dist package-lock.json