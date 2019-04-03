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
>> No VAR: IMAGE
>> No VAR: VERSION
>> image: <<NOVAR:IMAGE>>
>> version: <<NOVAR:VERSION>>

cat test.yaml | env IMAGE=myimg VERSION=v1 envtpl

>> image: myimg
>> version: v1
```





## Development

Dev env

```
curl https://sh.rustup.rs -sSf | sh\n
echo 'export PATH="$HOME/.cargo/bin:$PATH"'  >> ~/.zshrc
source ~/.zshrc
rustc --version
cargo install rustfmt
cargo install racer
rustup component add rust-src
cargo build
cargo run

```

Build macos and linux

```
bash build.sh

```


Some hints from here https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html
