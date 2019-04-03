# envtpl

## Installation

```
# linux
curl -L https://github.com/niquola/envtpl/releases/download/0.0.1/envtpl.linux > envtpl
# macos
curl -L https://github.com/niquola/envtpl/releases/download/0.0.1/envtpl.macos > envtpl

chmod u+x envtpl
```

## Usage

Util to replace env in templates

```
# test.yaml
image: {{IMAGE}}
version: {{VERSION}}

```

```bash

cat test.yaml | envtpl
cat test.yaml | env IMAGE=myimg VERSION=v1 envtpl
```





## Development

```
curl https://sh.rustup.rs -sSf | sh\n
echo 'export PATH="$HOME/.cargo/bin:$PATH"'  >> ~/.zshrc
source ~/.zshrc
rustc --version
cargo install rustfmt
cargo install racer
rustup component add rust-src
cargo build
cargo build --release
cargo run

```




Build for linux

```
docker pull clux/muslrust
docker run -v $PWD:/volume --rm -t clux/muslrust cargo build
ls target/

```
