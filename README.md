# PDM

PDM (Pulse Density Modulator) is a library meant for digital-to-analog conversion in embedded systems, using a simple second-order delta sigma modulator.
If no hardware PWM channels are available, this is a lightweight alternative.

This can be used, for example, to modulate the intensity of an LED.


## Features

The library offers the following features:
* Easy to use
* Suitable for `no_std` environments
* No external dependencies
* Ultra-low RAM requirements
* Supports the following types: u8, u16, u32, u64, i8, i16, i32, i64, f32, f64


## Limitations

For good results, the output frequency must be high and stable in relation to the modulated signal frequency.
An external low-pass filter may be necessary to remove noise, depending on the application.


## Example

```rust no_run
let mut pdm = pdm::Pdm::<u8>::new();
pdm.set_value(42);
loop {
    if pdm.update() {
        // Set output high
    } else {
        // Set output low
    }
    // Sleep until next iteration
}

```


## Changelog

| Date       | Version | Changes         |
|------------|---------|-----------------|
| 2025-04-13 | 0.1.0   | Initial release |
