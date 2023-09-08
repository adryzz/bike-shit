#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod physics;
mod values;

use embassy_rp::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::{USB, I2C0, PIN_4, PIN_5};
use embassy_rp::usb::Driver;
use embassy_rp::watchdog::Watchdog;
use embassy_rp::{bind_interrupts, i2c};
use embassy_time::{Duration, Timer, Instant};
use panic_persist::get_panic_message_utf8;
use {defmt_rtt as _, panic_persist as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
    I2C0_IRQ => i2c::InterruptHandler<I2C0>;
});

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {

    // report panic info
    panic_persist::report_panic_info(info);

    // clear watchdog scratch register
    let p = unsafe { Peripherals::steal() };
    let mut watchdog = Watchdog::new(p.WATCHDOG);
    watchdog.set_scratch(0, 0);

    watchdog.trigger_reset();

    // unreachable
    unreachable!()
}

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Start up watchdog
    let mut watchdog = Watchdog::new(p.WATCHDOG);
    watchdog.pause_on_debug(true);
    let watchdog_reset = watchdog.get_scratch(0) == values::WATCHDOG_SCRATCH0_VALUE;
    watchdog.set_scratch(0, values::WATCHDOG_SCRATCH0_VALUE);
    watchdog.start(Duration::from_millis(values::WATCHDOG_TIMER_MS));
    spawner.spawn(watchdog_feeder(watchdog)).unwrap();

    // Set up USB logging
    let driver = Driver::new(p.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // wait before starting up
    Timer::after(Duration::from_secs(2)).await;
    log::info!("[boot] Initialized Watchdog.");
    log::info!("[boot] Initialized USB logger.");

    if watchdog_reset {
        log::warn!("[boot] Watchdog reset!");
    }

    if let Some(msg) = get_panic_message_utf8() {
        log::warn!("[boot] {}", msg);
    }

    log::info!("[boot] Initializing GPS...");
    spawner.spawn(gps_task()).unwrap();

    log::info!("[boot] Initializing IMU...");
    spawner.spawn(imu_task(p.I2C0, p.PIN_5, p.PIN_4)).unwrap();

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
async fn imu_task(i2c: I2C0, scl: PIN_5, sda: PIN_4) {
    let _i2c = i2c::I2c::new_async(i2c, scl, sda, Irqs, i2c::Config::default());
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

/// This makes sure that if a task blocks for more than WATCHDOG_TIMER_MS, the device will reset.
#[embassy_executor::task]
async fn watchdog_feeder(mut watchdog: Watchdog) {
    loop {
        watchdog.feed();
        Timer::after(Duration::from_millis(values::WATCHDOG_TIMER_MS / 2)).await;
    }
}