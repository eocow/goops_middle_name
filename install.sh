echo "This will build then move the executable to the /usr/bin so it will need a sudo password"
cargo build --release
sudo mv target/release/goop /usr/bin
echo "Done!"