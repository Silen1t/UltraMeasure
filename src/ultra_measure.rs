#![no_std]
use embassy_rp::gpio::{ Input, Level, Output };
use embassy_time::{ Duration, Instant, Timer };

pub struct UltraMeasure<'a> {
    trigger: Output<'a>,
    echo: Input<'a>,
}

fn distance_cm_to_echo_ms(distance_cm: f32) -> f32 {
    let echo_us = (distance_cm * 2.0) / 0.0343;
    echo_us / 1000.0
}

impl<'a> UltraMeasure<'a> {
    /// Create new UltraMeasure instance
    pub fn new(mut trigger: Output<'a>, echo: Input<'a>) -> Self {
        trigger.set_level(Level::Low);
        Self { trigger, echo }
    }

    pub async fn measure_distance(&mut self, distance_cm: f32) -> Result<f32, ()> {
        // Send trigger pulse
        self.trigger.set_high();
        Timer::after(Duration::from_micros(10)).await;
        self.trigger.set_low();

        let echo_timeout = distance_cm_to_echo_ms(distance_cm) as u64;

        let start = Instant::now();
        while self.echo.is_low() {
            if Instant::now() - start > Duration::from_millis(echo_timeout) {
                return Err(());
            }
        }

        // Measure echo pulse duration echo timeout
        let start = Instant::now();
        while self.echo.is_high() {
            if Instant::now() - start > Duration::from_millis(echo_timeout) {
                return Err(());
            }
        }
        let duration = Instant::now() - start;

        // Calculate distance
        let distance = (duration.as_micros() as f32) / 58.0;
        Ok(distance)
    }
}
