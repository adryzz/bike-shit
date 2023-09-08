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
    log::info!("[boot] Initializing USB logger...");
    let driver = Driver::new(p.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // wait before starting up
    Timer::after(Duration::from_secs(2)).await;
    log::info!("[boot] Initialized USB logger.");

    log::info!("[boot] Initializing GPS...");
    spawner.spawn(gps_task()).unwrap();

    log::info!("[boot] Initializing IMU...");
    spawner.spawn(imu_task()).unwrap();

    log::info!("[boot] Initializing OSD...");
    spawner.spawn(osd_task()).unwrap();

    log::info!("[boot] Boot complete.");

    let mut north_back = Input::new(p.PIN_14, Pull::Down);
    let mut south_back = Input::new(p.PIN_15, Pull::Down);

    let mut last_time_back = Instant::now();
    let mut current_pole_back = false;

    loop {
        current_pole_back = !current_pole_back;
        if current_pole_back {
            north_back.wait_for_rising_edge().await;
        } else {
            south_back.wait_for_rising_edge().await;
        }
        let current_time = Instant::now();
        let difference_time = current_time - last_time_back;
        last_time_back = current_time;

        let rpm = 15000.0 / difference_time.as_millis() as f32;

        log::info!("A quarter of a turn happened! RPM: {}", rpm);
        // a quarter of a turn
    }
}

#[embassy_executor::task]
async fn gps_task() {
    log::info!("[gps] Initialized GPS.");
}

#[embassy_executor::task]
async fn imu_task() {
    log::info!("[imu] Initialized IMU.");
}

#[embassy_executor::task]
async fn osd_task() {
    log::info!("[osd] Initialized OSD.");
    // TODO: show banner
    Timer::after(Duration::from_secs(2)).await;

    log::info!("[osd] OSD set to refresh every {}ms.", values::OSD_REFRESH_MS);
    loop {
        Timer::after(Duration::from_millis(values::OSD_REFRESH_MS)).await;
        //log::debug!("OSD refresh");
    }
}