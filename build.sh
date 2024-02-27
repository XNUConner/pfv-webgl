cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/pfv_webgl.wasm --out-dir generated --target web
/home/conner/Desktop/devserver_build/devserver/target/debug/devserver --path generated/
