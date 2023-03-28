echo COMPILING FOR LINUX
cargo build --release #linux
echo LINUX COMPLETE
#echo WINDOWS NOW
#cargo build --release --target x86_64-pc-windows-gnu #windows
#echo WINDOWS DONE
echo WINDOWS MUST BE DONE ON BARE METAL
#echo COPYING
cp target/release/openxgauthenticator-cli ../site/downloads/openxgauthenticator-linux
cp target/x86_64-pc-windows-gnu/release/openxgauthenticator-cli.exe ../site/downloads/openxgauthenticator.exe

rm ../site/*.enc

echo COMPLETE
# see https://wapl.es/rust/2019/02/17/rust-cross-compile-linux-to-macos.html for later