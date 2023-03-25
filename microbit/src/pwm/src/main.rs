#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{Timer, pac::Peripherals, gpio, pwm, gpio::Level}
};
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Get ownership of peripherals
    let board_peripherals = Peripherals::take().expect("Couldn't initialize board_peripherals.");
    // Configuring output pin
     let gpio = gpio::p0::Parts::new(board_peripherals.P0);
     let pwm_pin_col1 = gpio.p0_28.into_push_pull_output(Level::Low).degrade();
     let pwm_pin_row1 = gpio.p0_21.into_push_pull_output(Level::Low).degrade();
     let pwm_pin_row2 = gpio.p0_22.into_push_pull_output(Level::Low).degrade();
     let pwm_pin_col2 = gpio.p0_31.into_push_pull_output(Level::Low).degrade();
     let pwm_pin_row3 = gpio.p0_24.into_push_pull_output(Level::Low).degrade();
     let pwm_pin_col3 = gpio.p0_30.into_push_pull_output(Level::Low).degrade();

     let pwm1 = pwm::Pwm::new(board_peripherals.PWM0);
     let pwm2 = pwm::Pwm::new(board_peripherals.PWM1);
     let pwm3 = pwm::Pwm::new(board_peripherals.PWM2);

     pwm1.set_output_pin(pwm::Channel::C0, pwm_pin_col1);
     pwm1.set_output_pin(pwm::Channel::C0, pwm_pin_row1);
     pwm2.set_output_pin(pwm::Channel::C1, pwm_pin_row2);
     pwm2.set_output_pin(pwm::Channel::C1, pwm_pin_col2);
     pwm3.set_output_pin(pwm::Channel::C2, pwm_pin_row3);
     pwm3.set_output_pin(pwm::Channel::C2, pwm_pin_col3);

     pwm1.set_prescaler(pwm::Prescaler::Div32);
     pwm2.set_prescaler(pwm::Prescaler::Div32);
     pwm3.set_prescaler(pwm::Prescaler::Div32);

     pwm1.set_max_duty(10_000_u16);
     pwm2.set_max_duty(10_000_u16);
     pwm3.set_max_duty(10_000_u16);

     pwm1.loop_inf();
     pwm2.loop_inf();
     pwm3.loop_inf();

     let mut timer = Timer::new(board_peripherals.TIMER0);
     let f: f32  = 1.0;
     let mut t: f32 = 0.0;
     loop {

         let alpha = 2.0*3.14*f*t;
         let beta = alpha + 2.1;
         let gamma = alpha + 4.1;

         let v0: u16 = ((1.0+libm::sinf(alpha))*10000.0/2.0) as u16;
         let v1: u16 = ((1.0+libm::sinf(beta))*10000.0/2.0) as u16;
         let v2: u16 = ((1.0+libm::sinf(gamma))*10000.0/2.0) as u16;

         pwm1.set_duty_off(pwm::Channel::C0, v0);
         pwm2.set_duty_off(pwm::Channel::C1, v1);
         pwm3.set_duty_off(pwm::Channel::C2, v2);

         timer.delay_ms(1u32);
         t += 0.001;
     }

}
