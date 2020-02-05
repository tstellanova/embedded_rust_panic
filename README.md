# embedded_rust_panic

## Building/installing with openocd:

eg on Mac OSX:
`brew install --HEAD openocd`

In one shell run `openocd` from the project directory. 
This should use the provided `openocd.cfg` file and essentially run `openocd -f interface/stlink.cfg -f target/stm32h7x.cfg`

In another shell run `cargo run` from the project directory with the NUCLEO-H743ZI2 board attached to your host via stlink.  This should utilize the `openocd.gdb` file to install and run the application. 

Note there are a lot of prerequisites. You need to have rust installed. You also need to have the embedded target installed, ie `rustup target add thumbv7em-none-eabihf`.  You also need to have arm embedded toolchain installed ie `arm-none-eabi-gdb`
