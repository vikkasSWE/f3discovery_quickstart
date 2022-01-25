# f3discovery_quickstart

## Run these to get started
```Rust
cargo install itm
rustup component add llvm-tools-preview
cargo install cargo-binutils
```

## Install the gcc toolchain
Chek the box for add to %PATH%
https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads

## Install openocd (unofficial):
Extract in "C:\OpenOCD" and add "bin" to %PATH% \
https://github.com/xpack-dev-tools/openocd-xpack/releases 

## PuTTY
https://www.chiark.greenend.org.uk/~sgtatham/putty/latest.html

## ST-Link USB-driver:
https://www.st.com/en/development-tools/stsw-link009.html#overview&secondary=st-get-software

## Flashing setup:
```Rust
rustup target add thumbv7em-none-eabihf
```
