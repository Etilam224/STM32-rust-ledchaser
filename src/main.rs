#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;

use embassy_stm32::{
    gpio::OutputType, 
    time::hz,
    timer::{low_level::CountingMode, simple_pwm::{self, PwmPin}},
    adc::Adc
};
use embassy_time::Timer;
use led_driver::{HIGHEST_VALUE, LOWEST_VALUE, NUMBER_LED_USIZE};

use {defmt_rtt as _, panic_probe as _};

pub mod led_driver;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // Potentiometer
    let mut adc_pin = p.PF4; // ADC_B_IN .. ADC123_IN12
    let mut adc = Adc::new(p.ADC3);

    // Red Leds
    let pin_red_1 = PwmPin::new_ch1(p.PE9, OutputType::PushPull);
    let pin_red_2 = PwmPin::new_ch2(p.PE11, OutputType::PushPull);
    let pin_red_3 = PwmPin::new_ch3(p.PE13, OutputType::PushPull);
    let pin_red_4 = PwmPin::new_ch4(p.PE14, OutputType::PushPull);

    // Yellow Leds
    let pin_yel_1 = PwmPin::new_ch1(p.PA0, OutputType::PushPull);
    let pin_yel_2 = PwmPin::new_ch2(p.PB3, OutputType::PushPull);
    let pin_yel_3 = PwmPin::new_ch3(p.PB10, OutputType::PushPull);
    let pin_yel_4 = PwmPin::new_ch4(p.PB11, OutputType::PushPull);

    // Green Leds
    let pin_gre_1 = PwmPin::new_ch1(p.PB4, OutputType::PushPull);
    let pin_gre_2 = PwmPin::new_ch2(p.PB5, OutputType::PushPull);
    let pin_gre_3 = PwmPin::new_ch3(p.PB0, OutputType::PushPull);
    let pin_gre_4 = PwmPin::new_ch4(p.PB1, OutputType::PushPull);

    let mut pwm_tim1 = simple_pwm::SimplePwm::new(
        p.TIM1, 
        Some(pin_red_1), 
        Some(pin_red_2),
        Some(pin_red_3),
        Some(pin_red_4),
        hz(2000),
        CountingMode::EdgeAlignedUp
    );

    let mut pwm_tim2 = simple_pwm::SimplePwm::new(
        p.TIM2, 
        Some(pin_yel_1), 
        Some(pin_yel_2),
        Some(pin_yel_3),
        Some(pin_yel_4), 
        hz(2000),
        CountingMode::EdgeAlignedUp
    );

    let mut pwm_tim3 = simple_pwm::SimplePwm::new(
        p.TIM3, 
        Some(pin_gre_1), 
        Some(pin_gre_2),
        Some(pin_gre_3),
        Some(pin_gre_4), 
        hz(2000),
        CountingMode::EdgeAlignedUp
    );

    let max_duty_tim1 = pwm_tim1.max_duty_cycle();
    pwm_tim1.ch1().set_duty_cycle(0);
    pwm_tim1.ch2().set_duty_cycle(0);
    pwm_tim1.ch3().set_duty_cycle(0);
    pwm_tim1.ch4().set_duty_cycle(0);
    pwm_tim1.ch1().enable();
    pwm_tim1.ch2().enable();
    pwm_tim1.ch3().enable();
    pwm_tim1.ch4().enable();

    let max_duty_tim2 = pwm_tim2.max_duty_cycle();
    pwm_tim2.ch1().set_duty_cycle(0);
    pwm_tim2.ch2().set_duty_cycle(0);
    pwm_tim2.ch3().set_duty_cycle(0);
    pwm_tim2.ch4().set_duty_cycle(0);
    pwm_tim2.ch1().enable();
    pwm_tim2.ch2().enable();
    pwm_tim2.ch3().enable();
    pwm_tim2.ch4().enable();

    let max_duty_tim3 = pwm_tim3.max_duty_cycle();
    pwm_tim3.ch1().set_duty_cycle(0);
    pwm_tim3.ch2().set_duty_cycle(0);
    pwm_tim3.ch3().set_duty_cycle(0);
    pwm_tim3.ch4().set_duty_cycle(0);
    pwm_tim3.ch1().enable();
    pwm_tim3.ch2().enable();
    pwm_tim3.ch3().enable();
    pwm_tim3.ch4().enable();

    info!("MAX PWM : {}, {}, {}", max_duty_tim1, max_duty_tim2, max_duty_tim3);

    let mut x: [u8; NUMBER_LED_USIZE]  = [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        11
    ];


    loop{

        let sample = adc.blocking_read(&mut adc_pin);

        // for item in x.into_iter().enumerate() {
        //     let (i, y): (usize, u8) = item;
            
        //     if y >= 11{
        //         x[i] = 0;
        //     }
        //     else {
        //         x[i] += 1;
        //     }

        // }

        analog_to_led(sample, &mut x);

        pwm_tim1.ch1().set_duty_cycle_percent(x[11]);
        pwm_tim1.ch2().set_duty_cycle_percent(x[10]);
        pwm_tim1.ch3().set_duty_cycle_percent(x[9]);
        pwm_tim1.ch4().set_duty_cycle_percent(x[8]);
        pwm_tim2.ch1().set_duty_cycle_percent(x[7]);
        pwm_tim2.ch2().set_duty_cycle_percent(x[6]);
        pwm_tim2.ch3().set_duty_cycle_percent(x[5]);
        pwm_tim2.ch4().set_duty_cycle_percent(x[4]);
        pwm_tim3.ch1().set_duty_cycle_percent(x[3]);
        pwm_tim3.ch2().set_duty_cycle_percent(x[2]);
        pwm_tim3.ch3().set_duty_cycle_percent(x[1]);
        pwm_tim3.ch4().set_duty_cycle_percent(x[0]);
        Timer::after_millis(20).await;

    }
}

fn analog_to_led(analog_input: u16,led_array: &mut[u8; NUMBER_LED_USIZE]){

    match analog_input{
        0..LOWEST_VALUE => led_driver::min_value(led_array),
        LOWEST_VALUE..HIGHEST_VALUE => led_driver::value_by_range(led_array, analog_input),
        _=> led_driver::max_value(led_array),
    }

}