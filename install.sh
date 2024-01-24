echo "This will build then move the executable to the /usr/bin so it will need a sudo password"
cargo build --release
sudo ln -s $PWD/target/release/goop /usr/local/bin/
mkdir ~/.config/goop
cp names ~/.config/goop/
cp config.toml ~/.config/goop/
echo "Done!"