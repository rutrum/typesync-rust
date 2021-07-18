test:
    cargo test --all

watch-test:
    watchexec -- just test

build:
    cargo build --all

web:
    (cd client; wasm-pack build --target web --dev)

watch-web:
    watchexec -w client/src -- just web

serve: web
    (cd client; microserver)

tree:
    tree -I "pkg|target" --dirsfirst

api:
    cargo run -p api

watch-api:
    watchexec -w api/src -- just api

fmt:
    cargo fmt

watch-check:
    watchexec -- cargo check --workspace

css:
    (cd client; grass scss/index.scss > index.css)

watch-css:
    watchexec -w client/scss -- just css

build-prod:
    ([ ! -e /tmp/typesync ] && mkdir /tmp/typesync) || true
    ([ ! -e /tmp/typesync/client ] && mkdir /tmp/typesync/client) || true
    cargo build -p api --release
    cp target/release/api /tmp/typesync/
    cp .env_prod /tmp/typesync/.env
    cp Rocket_prod.toml /tmp/typesync/Rocket.toml
    bash -c "source .env_prod; cd client; wasm-pack build --target web --release --no-typescript"
    cp client/record.png /tmp/typesync/client/record.png
    cp -r client/pkg /tmp/typesync/client

    (cd client; grass scss/index.scss > /tmp/typesync/client/index.css)
    cp client/index.html /tmp/typesync/client/index.html

    tar -czf typesync.tar.gz -C /tmp typesync
