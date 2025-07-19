# Ultra Measure

`UltraMeasure` is an embedded Rust library for controlling and reading distance from ultrasonic sensors (e.g., HC-SR04) using the [`embassy-rp`](https://crates.io/crates/embassy-rp) async framework.

>  **Note:** Accurate, async distance sensing made simple with Embassy + RP2040.

## Features

- Async distance measurement using GPIO trigger and echo pins
- Designed for Embassy embedded framework
- Simple API for safe and non-blocking measurements
- Suitable for sensors like HC-SR04
  
> ⚠️ **Note**: This library is currently compatible only with **Raspberry Pi Pico W** using `embassy-rp`.

## how to use it

```rs
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{ Input, Level, Output, Pull };
use embassy_time::{ Duration, Timer };
use ultra_measure::UltraMeasure;
use ::{ defmt_rtt as _, panic_probe as _ };

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let trigger = Output::new(p.PIN_14, Level::Low);
    let echo = Input::new(p.PIN_15, Pull::None);

    // Init Sensor
    let mut sensor = UltraMeasure::new(trigger, echo);

    loop {
        // Wait for echo to go high
        match sensor.measure_distance(100.0).await {
            Ok(hit_point_distance) => {}
            Err(_) => {}
        }

        // Wait before next measurement
        Timer::after(Duration::from_millis(500)).await;
    }
}
```

> ⚠️ **Note:** Always call `Timer::after(Duration::from_millis(500)).await;` at the end of each loop iteration to ensure the sensor is detecting.
