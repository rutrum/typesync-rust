test:
    cargo test --all

build:
    cargo build --all

web:
    (cd client; wasm-pack build --target web --out-name package --dev)

serve: web
    (cd client; microserver)
