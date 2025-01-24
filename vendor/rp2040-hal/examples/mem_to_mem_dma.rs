//! # Memory to memory DMA transfer Example
//!
//! This application demonstrates how to use DMA to transfer data from memory to memory buffers.
//!
//! See the `Cargo.toml` file for Copyright and licence details.
#![no_std]
#![no_main]

use cortex_m::singleton;
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use hal::dma::{single_buffer, DMAExt};
use hal::pac;
use panic_halt as _;
use rp2040_hal as hal;
use rp2040_hal::clocks::Clock;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Setup clocks and the watchdog.
    let mut watchdog = hal::watchdog::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Setup the pins.
    let sio = hal::sio::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Initialize DMA.
    let dma = pac.DMA.split(&mut pac.RESETS);
    // Configure GPIO25 as an output
    let mut led_pin = pins.gpio25.into_push_pull_output();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Use DMA to transfer some bytes (single buffering).
    let tx_buf = singleton!(: [u8; 16] = [0x42; 16]).unwrap();
    let rx_buf = singleton!(: [u8; 16] = [0; 16]).unwrap();

    // Use a single_buffer to read from tx_buf and write into rx_buf
    let transfer = single_buffer::Config::new(dma.ch0, tx_buf, rx_buf).start();
    // Wait for both DMA channels to finish
    let (_ch, tx_buf, rx_buf) = transfer.wait();

    // Compare buffers to see if the data was transferred correctly
    // Slow blink on success, fast on failure
    let delay_ms = if tx_buf == rx_buf { 1000 } else { 100 };

    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(delay_ms);
        led_pin.set_low().unwrap();
        delay.delay_ms(delay_ms);
    }
}
