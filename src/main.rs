#![no_std]
#![no_main]

use cortex_m;
use cortex_m::peripheral::NVIC;
use cortex_m_rt;
use hal::gpio::v2::PA08;
use hal::gpio::v2::PinId;
use hal::gpio::v2::{PA02, PA04};
use hal::gpio::{Output, PushPull};
use hal::timer::TimerCounter;
use panic_halt as _;
use xiao_m0 as hal;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::gpio::Pin;
use hal::pac::{interrupt, CorePeripherals, Peripherals, TC3, TC4, TC5};
use hal::prelude::*;
use hal::time::U32Ext;
use hal::{entry, Pins};

struct Ctx1 {
    out: Pin<PA02, Output<PushPull>>,
    tc3: TimerCounter<TC3>,
    counter: u32,
}
struct Ctx2 {
    out: Pin<PA08, Output<PushPull>>,
    tc5: TimerCounter<TC4>,
    counter: u32,
}
static mut CTX1: Option<Ctx1> = None;
static mut CTX2: Option<Ctx2> = None;


static PERIOD: u32 = 3000;
static DELAY1_us: u32 = 200;
static WIDTH1_us: u32 = 500;
static DELAY2_us: u32 = 200;
static WIDTH2_us: u32 = 500;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut led = pins.led2.into_open_drain_output(&mut pins.port);

    let gclk0= clocks.gclk0();

    // // let mut timer_clock = clocks.tcc2_tc3(&gclk5).unwrap();
    let mut tc3 = TimerCounter::tc3_(&clocks.tcc2_tc3(&gclk0).unwrap(), peripherals.TC3, &mut peripherals.PM);
    // let mut tc5 = TimerCounter::tc4_(&clocks.tc4_tc5(&gclk0).unwrap(), peripherals.TC4, &mut peripherals.PM);
    let mut delay = Delay::new(core.SYST, &mut clocks);

    tc3.enable_interrupt();
    // tc5.enable_interrupt();
    // led.toggle();

    tc3.start(500.khz());
    // tc5.start(100.khz());
    // tc5.start(500.khz());

    unsafe {
        NVIC::unmask(interrupt::TC3);
    }
    // unsafe {
    //     NVIC::unmask(interrupt::TC5);
    // }

    unsafe {
        CTX1 = Some(Ctx1{
            out: pins.a0.into_push_pull_output(&mut pins.port),
            tc3,
            counter: 0u32,
        });
        // CTX2 = Some(Ctx2{
        //     out: pins.a4.into_push_pull_output(&mut pins.port),
        //     tc5,
        //     counter: 0u32
        // });
    }
    // a0.set_high();

    loop {
        delay.delay_ms(1u8);
    }
}

#[interrupt]
fn TC3() {
    unsafe {
        let ctx = CTX1.as_mut().unwrap();
        ctx.tc3.wait().unwrap();
        ctx.counter = ctx.counter + 1;
        
        if (ctx.counter >= DELAY1_us+WIDTH1_us) {
            ctx.out.set_low();
        }
        else if (ctx.counter >= DELAY1_us) {
            ctx.out.set_high();
        }
        if (ctx.counter == PERIOD) {
            ctx.counter = 0u32;
        }

    }
}

// #[interrupt]
// fn TC5() {
//     unsafe {
//         let ctx = CTX2.as_mut().unwrap();
//         ctx.tc5.wait().unwrap();
//         ctx.counter = ctx.counter + 1;
        
//         if (ctx.counter >= DELAY2_us+WIDTH2_us) {
//             ctx.out.set_low();
//         }
//         else if (ctx.counter >= DELAY2_us) {
//             ctx.out.set_high();
//         }
//         if (ctx.counter == PERIOD) {
//             ctx.counter = 0u32;
//         }

//     }
// }
