#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod physics;
mod values;

use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_rp::bind_interrupts;
use embassy_time::{Duration, Timer, Instant};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // Set up USB logging
    log::info!("Initializing USB logger...");
    let driver = Driver::new(p.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();
    log::info!("Initialized USB logger.");

    log::info!("Initializing OSD...");
    spawner.spawn(osd_task()).unwrap();

    let mut north = Input::new(p.PIN_14, Pull::Down);
    let mut south = Input::new(p.PIN_15, Pull::Down);

    let mut current_pole = false;
    let mut last_time = Instant::now();
    loop {
        current_pole = !current_pole;
        if current_pole {
            north.wait_for_rising_edge().await;
        } else {
            south.wait_for_rising_edge().await;
        }
        let current_time = Instant::now();
        let difference_time = current_time - last_time;
        last_time = current_time;

        let rpm = 15000.0 / difference_time.as_millis() as f32;

        log::info!("A quarter of a turn happened! RPM: {}", rpm);
        // a quarter of a turn
    }
}

#[embassy_executor::task]
async fn osd_task() {
    log::info!("Initialized OSD.");
    // TODO: show banner
    Timer::after(Duration::from_secs(2)).await;

    log::info!("OSD set to refresh every {}ms.", values::OSD_REFRESH_MS);
    loop {
        Timer::after(Duration::from_millis(values::OSD_REFRESH_MS)).await;
        //log::debug!("OSD refresh");
    }
}
