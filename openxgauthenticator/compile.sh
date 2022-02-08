echo COMPILING FOR LINUX
cargo build --release #linux
echo LINUX COMPLETE
echo WINDOWS NOW
cargo build --release --target x86_64-pc-windows-gnu #windows
echo WINDOWS DONE
echo COMPLETE
# see https://wapl.es/rust/2019/02/17/rust-cross-compile-linux-to-macos.html for later
