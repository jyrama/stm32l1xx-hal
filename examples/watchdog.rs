#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use hal::prelude::*;
use hal::rcc::ClockSrc;
use hal::stm32;
use rt::entry;
use sh::hprintln;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.clock_src(ClockSrc::HSI).freeze();
    let mut delay = cp.SYST.delay(clocks);

    hprintln!("Starting watchdog").unwrap();

    //let mut watchdog = dp.WWDG.watchdog(clocks);
    let mut watchdog = dp.IWDG.watchdog();
    watchdog.start(100.ms());

    delay.delay(60.ms());
    //delay.delay(120.ms());

    cortex_m::asm::bkpt();

    loop {}
}
