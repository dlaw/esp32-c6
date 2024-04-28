#![no_std]
#![no_main]

#[rtic::app(device=esp32c6)]
mod app {
    use esp_backtrace as _;
    use esp_hal::prelude::*;
    use esp_println::println;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        button: esp_hal::gpio::Gpio9<esp_hal::gpio::Input<esp_hal::gpio::PullUp>>,
    }

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        println!("RTIC starting");
        let peripherals = esp_hal::peripherals::Peripherals::take();
        let io = esp_hal::gpio::IO::new(peripherals.GPIO, peripherals.IO_MUX);
        let mut button = io.pins.gpio9.into_pull_up_input();
        button.listen(esp_hal::gpio::Event::FallingEdge);
        (Shared {}, Local {button})
    }

    #[task(binds=GPIO, local=[button], priority=3)]
    fn gpio_handler(cx: gpio_handler::Context) {
        println!("Button!");
        cx.local.button.clear_interrupt();
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        let mut i = 0;
        loop {
            if i % 1000000 == 0 {
                println!("Tick {}", i);
            }
            i += 1;
        }
    }
}
