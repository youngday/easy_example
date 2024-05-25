# rust env 

## ref
https://rustcc.gitbooks.io/rustprimer/content/install/rustup.html

## install

sudo apt install curl build-essential gcc make

curl https://sh.rustup.rs -sSf | sh

source $HOME/.cargo/env

## version check
```sh

rustup update 
rustup show
#rustup default 1.78.0 or stable
rustup default <toolchain>
rustc --version

```

## uninstall

rustup self uninstall

## proxy

create file config.toml in ~/.cargo

```toml
[source.crates-io]
replace-with = 'rsproxy-sparse'
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"
[net]
git-fetch-with-cli = true
```