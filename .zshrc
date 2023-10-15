export ZSH="/root/.oh-my-zsh"
export PATH="/root/.cargo/bin:$PATH"
export CAP_HILL_RUST="/root/cap-hill-rust"

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
    cd $CAP_HILL_RUST && cargo leptos build --release
}

test () {
    cd $CAP_HILL_RUST && cargo leptos test
}

release () {
    test && build && release_unchecked
}

release_unchecked () {
    cd $CAP_HILL_RUST \
        && VERSION=$(cargo metadata --no-deps --format-version 1 | jq ".packages[0].version" | xargs) \
        && APP=cap_hill_rust_v${VERSION}_$(git rev-parse --short HEAD) \
        && rm -f /usr/local/bin/$APP \
        && cp ../target/server/release/cap-hill-rust /usr/local/bin/$APP \
        && ln -sf /usr/local/bin/$APP /usr/local/bin/cap-hill-rust \
        && rm -rf /usr/local/etc/cap-hill-rust/public \
        && mkdir -p /usr/local/etc/cap-hill-rust/public \
        && rsync -a /root/cap-hill-rust/target/site/ /usr/local/etc/cap-hill-rust/public/ \
        && systemctl restart cap-hill-rust
        sleep 3
        systemctl status cap-hill-rust
}