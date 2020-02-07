# embedded_rust_panic

## What it is
This is a simple embedded rust application for Cortex-M that tests:
- Access to ITM / SWO output. 
- Whether ITM print logging can be read by the debugger
- Whether panic_itm is working properly
If you're using VSCode, see the .vscode/launch.json for an example of `cortex-debug` configuration that will allow you to view ITM log output.

## Building/installing with openocd:

eg on Mac OSX:
`brew install --HEAD openocd`

In one shell run `openocd` from the project directory. 
This should use the provided `openocd.cfg` configuration file.

In another shell run `cargo run` from the project directory with the target development board attached to your host via stlink.  This should utilize the `openocd.gdb` file to install and run the application. 

Note there are many toolchain prerequisites. You need to have rust installed. You also need to have the embedded target installed, ie `rustup target add thumbv7em-none-eabihf`.  You also need to have arm embedded toolchain installed ie `arm-none-eabi-gdb`.

The default openocd configuration is for an [STM32F401 development board](https://www.amazon.com/SongHe-STM32F401-Development-STM32F401CCU6-Learning/dp/B07XBWGF9M/ref=sr_1_1).  In addition I've provided a VSCode launch configuration (`.vscode/launch.json`) with a config for the `cortex-debug` extension that will read ITM/SWP output to a console window. 


## Reading SWO Output

You need to have an stlink or jtag programmer/debugger attached to your Cortex-M development board (or chip breakout) that connects to the SWO signal. In some cases this may mean soldering bridges, or connecting it yourself to eg the `PB3` pin on a compact breakout board. The exact route for this varies from board to board and chip to chip.  Note that the cheap, widely available [stlink programmers](https://www.amazon.com/Aideepen-ST-Link-Programming-Emulator-Downloader/dp/B01J7N3RE6/ref=sr_1_1?keywords=stlink&qid=1581094343&s=electronics&sr=1-1) do not route the SWO signal, though you can modify the hardware yourself to do this.  Almost any stm32 discovery board, eg the inexpensive [stm32f0discovery](https://www.mouser.com/ProductDetail/STMicroelectronics/STM32F0DISCOVERY?qs=%2Fha2pyFadugozBplj2HA6I5SJWdyEV7UjcFqXknA50zpNg0Ehpc7VQ%3D%3D), can be used to act as a standalone stlink programmer with SWO routing. 
