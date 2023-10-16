# update system
dnf update -y

# install packages
dnf install -y zsh vim git bat exa gcc openssl-devel lsof

# change shell to zsh
usermod -s $(which zsh) root

# install oh-my-zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"
exit

# install zsh-syntax-highlighting
cd /root/.oh-my-zsh/custom/plugins && git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:--/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting

# install zsh-autosuggestions
cd /root/.oh-my-zsh/custom/plugins && git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:--/.oh-my-zsh/custom}/plugins/zsh-autosuggestions

# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# install bottom
dnf copr enable atim/bottom -y
dnf install bottom -y

# install leptos
cargo install cargo-leptos --version 0.2.0