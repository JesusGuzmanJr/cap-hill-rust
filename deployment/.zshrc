export ZSH="/root/.oh-my-zsh"
export PATH="/root/.cargo/bin:$PATH"
export CAP_HILL_RUST="/root/cap-hill-rust"
export LEPTOS_WASM_OPT_VERSION="version_116"

ZSH_THEME="pygmalion"

plugins=(z rust git zsh-syntax-highlighting zsh-autosuggestions zbell)
source $ZSH/oh-my-zsh.sh

alias l='exa --long --header --group-directories-first --created --git --modified --all'

config() {
    vim ~/.zshrc
}

ssh_config() {
    vim ~/.ssh/config
}

gsha() {
    git rev-parse --short HEAD
}

ip_address() {
    ip_v4_address
}

ip_v4_address() {
    curl https://api.ipify.org
}

ip_v6_address() {
    curl -6 https://ifconfig.co
}

find_port() {
    lsof -i :$1
}

edit_unit () {
    vim /etc/systemd/system/cap-hill-rust.service
}

logs () {
    journalctl --unit cap-hill-rust --follow "$@"
}

build () {
    pushd $CAP_HILL_RUST && cargo leptos build --release
    popd
}

test () {
    pushd $CAP_HILL_RUST && cargo leptos test
    popd
}

release () {
    test && build && release_unchecked
}

release_unchecked () {
    pushd $CAP_HILL_RUST \
        && VERSION=$(cargo metadata --no-deps --format-version 1 | jq ".packages[0].version" | xargs) \
        && APP=cap_hill_rust_v${VERSION}_$(git rev-parse --short HEAD) \
        && rm -f /usr/local/bin/$APP \
        && cp target/release/cap-hill-rust /usr/local/bin/$APP \
        && ln -sf /usr/local/bin/$APP /usr/local/bin/cap-hill-rust \
        && rm -rf /usr/local/etc/cap-hill-rust/assets \
        && mkdir -p /usr/local/etc/cap-hill-rust/assets \
        && rsync -a /root/cap-hill-rust/target/site/ /usr/local/etc/cap-hill-rust/assets/ \
        && systemctl restart cap-hill-rust
    sleep 3
    systemctl status cap-hill-rust
    popd
}