#![no_std]
#![no_main]

extern crate panic_itm;
#[macro_use]
extern crate cortex_m_rt;

use cortex_m::{iprint, iprintln, Peripherals};

///
/// A simple rust embedded application for Cortex-M that tests:
/// - Access to ITM / SWO
/// - Whether ITM print logging can be read by the debugger
/// - Whether panic_itm is working properly
/// If you're using VSCode, see the .vscode/launch.json for an example of
/// `cortex-debug` configuration that will allow you to view ITM log output.
///
#[entry]
fn main() -> ! {

    let mut cp = Peripherals::take().unwrap();
    let stim = &mut cp.ITM.stim[0];

    iprintln!(stim, "stage 01");

    let x = 5 * 25;

    iprintln!(stim, "stage 02: {}", x);

    loop {
        iprint!(stim, ".");
        panic!("FOO");
    }

}

