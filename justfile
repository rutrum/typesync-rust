test:
    cargo test --all

build:
    cargo build --all

web:
    (cd client; wasm-pack build --target web --out-name package --dev)

watch-web:
    watchexec -w client/src -- just web

serve: web
    (cd client; microserver)

tree:
    tree -I "pkg|target" --dirsfirst
