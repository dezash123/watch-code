#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::spi::master::{Address, Command, Config, Spi};
use esp_hal::spi::DataMode;
use esp_hal::time::Rate;
use esp_hal::gpio::Flex;

use defmt_rtt as _;
use defmt::info;

use embassy_time::{Timer};
use embassy_executor::Spawner;
use panic_rtt_target as _;

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();
#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);
    // COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    // TODO: Spawn some tasks
    let _ = spawner;
    
    info!("Hello world!");

    let mut spi = Spi::new(
        peripherals.SPI2,
        Config::default()
        .with_frequency(Rate::from_mhz(10))
    ).unwrap()
    .with_sck(peripherals.GPIO12)
    .with_sio0(peripherals.GPIO11)
    // .with_sio1(peripherals.GPIO13)
    // .with_sio2(peripherals.GPIO13)
    // .with_sio3(peripherals.GPIO13)
    .with_cs(peripherals.GPIO10);
    //.into_async();

    let mut rst = Flex::new(peripherals.GPIO1);
    info!("resetting");
    rst.set_low();
    Timer::after_millis(500).await;
    rst.set_high();
    Timer::after_millis(500).await;

    info!("resetting spi mode");
    // reset to sspi mode
    spi.half_duplex_write(
        DataMode::Single,
        Command::_8Bit(0xFF, DataMode::Single),
        Address::None,
        0,
        &[],
    ).unwrap();
    Timer::after_millis(500).await;
    
    info!("reading");
    let mut buf = [0u8; 3];
    spi_read(&mut spi, 0x04, &mut buf).unwrap();
    info!("id: {:?}", buf);
    Timer::after_secs(1).await;

    spi_write(&mut spi, 0xFE, &[0x00]).unwrap();
    spi_write(&mut spi, 0xC4, &[0x80]).unwrap();
    spi_write(&mut spi, 0x3A, &[0x77]).unwrap();
    // spi_write(&mut spi, 0x35, &[0x00]).unwrap();
    spi_write(&mut spi, 0x34, &[0x00]).unwrap(); // TEOFF
    spi_write(&mut spi, 0x53, &[0x20]).unwrap();
    spi_write(&mut spi, 0x51, &[0xFF]).unwrap();
    spi_write(&mut spi, 0x63, &[0xFF]).unwrap();
    spi_write(&mut spi, 0x2A, &[0x00, 240, 0, 255]).unwrap();
    spi_write(&mut spi, 0x2B, &[0x00, 240, 0, 255]).unwrap();
    spi_write(&mut spi, 0x11, &[]).unwrap();
    Timer::after_millis(500).await;
    spi_write(&mut spi, 0x29, &[]).unwrap();
    info!("init done");
    Timer::after_secs(1).await;

    let mut buf = [0u8; 1];
    spi_read(&mut spi, 0x0A, &mut buf).unwrap();
    info!("disply power mode: {:08b}", buf[0]);

    let mut buf = [0u8; 1];
    spi_read(&mut spi, 0x54, &mut buf).unwrap();
    info!("brightness mode: {:08b}", buf[0]);

    let mut buf = [0u8; 1];
    spi_read(&mut spi, 0x0E, &mut buf).unwrap();
    info!("display signal mode: {:08b}", buf[0]);
    loop {
        info!("on");
        spi_write(&mut spi, 0x23, &[]).unwrap();
        Timer::after_secs(2).await;
        info!("off");
        spi_write(&mut spi, 0x22, &[]).unwrap();
        Timer::after_secs(2).await;
    }
}

fn spi_read<Dm: esp_hal::DriverMode>(spi: &mut Spi<Dm>, addr: u8, buf: &mut [u8]) -> Result<(), esp_hal::spi::Error> {
    info!("spi read: {}", addr);
    spi.half_duplex_read(
        DataMode::Single,
        Command::_8Bit(0x03, DataMode::Single),
        Address::_24Bit(u32::from_be_bytes([0x00, 0x00, addr, 0x00]), DataMode::Single),
        0,
        buf,
    )
}

fn spi_write<Dm: esp_hal::DriverMode>(spi: &mut Spi<Dm>, addr: u8, data: &[u8]) -> Result<(), esp_hal::spi::Error> {
    info!("spi write: {}: {:?}", addr, data);
    spi.half_duplex_write(
        DataMode::Single,
        Command::_8Bit(0x02, DataMode::Single),
        Address::_24Bit(u32::from_be_bytes([0x00, 0x00, addr, 0x00]), DataMode::Single),
        0,
        data,
    )
}


// fn to_point(data: [u8; 3]) -> (u16, u16) {
//     let x = u16::from_be_bytes([(data[0] >> 6) & 1, data[1]]);
//     let y = u16::from_be_bytes([(data[0] >> 7) & 1, data[2]]);
//     (x, y)
// }
// 
// async fn touch_stuff(peripherals: Peripherals) {
//     let mut i2c = I2c::new(
//         peripherals.I2C0,
//         I2cConfig::default()
//             .with_frequency(Rate::from_khz(200))
//     ).unwrap()
//     .with_sda(peripherals.GPIO6)
//     .with_scl(peripherals.GPIO5)
//     .into_async();
// 
//     let mut int = Flex::new(peripherals.GPIO7);
//     int.apply_input_config(&InputConfig::default().with_pull(Pull::Up));
// 
//     loop {
//         int.wait_for_low().await;
//         info!("falling edge");
//         let mut buf = [0u8; 3];
//         match i2c.read(0x2E, &mut buf) {
//             Ok(_) => {
//                 info!("point: {:?}", to_point(buf));
//             },
//             Err(e) => {
//                 info!("error: {:?}", e);
//             }
//         }
//         int.wait_for_high().await;
//     }
// }
