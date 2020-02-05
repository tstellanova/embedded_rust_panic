#![no_std]
#![no_main]

extern crate panic_itm;
#[macro_use]
extern crate cortex_m_rt;

use cortex_m::{iprint, iprintln, Peripherals};


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

