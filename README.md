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
image: "{{ IMAGE }}:{{ VERSION }}"
extra:
  with_default: "{{ VAR1 || this is default }}"
  inst: "{{ #inst }}"
  base64: "{{ VAR2 | base64 }}"

```

```bash

cat test.yaml | envtpl
> No VAR: IMAGE
> No VAR: VERSION
> No VAR: VAR2
> home: /Users/nicola
> image: "<<NOVAR:IMAGE>>:<<NOVAR:VERSION>>"
> extra:
>   with_default: "this is default"
>   inst: "2019-04-11T07:04:32.886068+03:00"
>   base64: "<<NOVAR:VAR2>>"


cat test.yaml | env IMAGE=myimg VERSION=v1 VAR2=secret envtpl
> home: /Users/nicola
> image: "myimg:v1"
> extra:
>   with_default: "this is default"
>   inst: "2019-04-11T07:06:20.248382+03:00"
>   base64: "c2VjcmV0"
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
