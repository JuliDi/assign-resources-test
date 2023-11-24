#![no_std]


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
