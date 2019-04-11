rm -r target
cargo build --release
strip target/release/envtpl

mkdir -p bin
cp target/release/envtpl bin/envtpl.macos

docker run -v $PWD:/volume --rm -ti clux/muslrust  bash -c 'cargo build --release && strip target/x86_64-unknown-linux-musl/release/envtpl'
cp target/x86_64-unknown-linux-musl/release/envtpl bin/envtpl.linux

ls -lah bin
