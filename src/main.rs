#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use {defmt_rtt as _, panic_probe as _};

use assign_resources::assign_resources;
use embassy_stm32::peripherals;

assign_resources! {
    usb: UsbResources {
        dp: PA12,
        dm: PA11,
        usb: USB_OTG_FS,
    }
    leds: LedResources {
        r: PA2,
        g: PA3,
        b: PA4,
        tim2: TIM2,
    }
}
#[embassy_executor::task]
async fn usb_task(r: UsbResources) {
    // use r.dp, r.dm, r.usb
}
#[embassy_executor::task]
async fn led_task(r: LedResources) {
    // use r.r, r.g, r.b, r.tim2
}
#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = embassy_stm32::init(Default::default());
    let r = split_resources!(p);
    spawner.spawn(usb_task(r.usb)).unwrap();
    spawner.spawn(led_task(r.leds)).unwrap();
    // can still use p.PA0, p.PA1, etc
}