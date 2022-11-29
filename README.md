# gw2-stats
Collect account info from the GW2 api, save to DB, expose API for timeseries analisys

# Build for ARM
https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/
Make sure to DL the linux libraries.   
-> AArch32 target with hard float (arm-none-linux-gnueabihf)

rustup target add armv7-unknown-linux-gnueabihf

```bash
./build.sh
```
Cross compilation is a bit painful... build on the RPI instead.

# Run
Set some ENV variables (tokens, passwords, see src/main.rs)

