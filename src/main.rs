#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _; 

use stm32f1::stm32f103;

#[entry]
fn main() -> ! {
    let peripherals = stm32f103::Peripherals::take().unwrap();
    let rcc = &peripherals.RCC;
    let gpioc = &peripherals.GPIOC;

    rcc.apb2enr.write(|w| w.iopcen().set_bit());

    gpioc.crh.write(|w| w.mode13().output().cnf13().push_pull());

    loop {
        gpioc.bsrr.write(|w| w.bs13().set_bit());
        cortex_m::asm::delay(8_000_000);

        gpioc.brr.write(|w| w.br13().set_bit());
        cortex_m::asm::delay(8_000_000);
    }
}
