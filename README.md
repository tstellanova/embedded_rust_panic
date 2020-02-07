# embedded_rust_panic

## Building/installing with openocd:

eg on Mac OSX:
`brew install --HEAD openocd`

In one shell run `openocd` from the project directory. 
This should use the provided `openocd.cfg` configuration file.

In another shell run `cargo run` from the project directory with the target development board attached to your host via stlink.  This should utilize the `openocd.gdb` file to install and run the application. 

Note there are many toolchain prerequisites. You need to have rust installed. You also need to have the embedded target installed, ie `rustup target add thumbv7em-none-eabihf`.  You also need to have arm embedded toolchain installed ie `arm-none-eabi-gdb`.

The default openocd configuration is for an [STM32F401 development board](https://www.amazon.com/SongHe-STM32F401-Development-STM32F401CCU6-Learning/dp/B07XBWGF9M/ref=sr_1_1).  In addition I've provided a VSCode launch configuration (`.vscode/launch.json`) with a config for the `cortex-debug` extension that will read ITM/SWP output to a console window. 
