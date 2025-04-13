/* pdm | lib.rs
 * Copyright (c) 2025 L. Sartory
 * SPDX-License-Identifier: MIT
 */

/******************************************************************************/

#![no_std]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/******************************************************************************/

/// A trait that allow generic modulator implementations
pub trait Modulator {
    /// The data type used for the integrator
    type SigmaType;
}

/******************************************************************************/

/// Pulse Density Modulator
///
/// Contains the current setpoint and integrator values
pub struct Pdm<T: Modulator> {
    value: T,
    sigma: [T::SigmaType; 2]
}

impl<T: Modulator> Pdm<T> {
    /// Sets the modulator value
    pub fn set_value(&mut self, value: T) {
        self.value = value
    }
}

/******************************************************************************/

macro_rules! gen_unsigned_impl {
    ($T: ty, $S: ty) => {
        impl Modulator for $T {
            type SigmaType = $S;
        }

        impl Pdm<$T> {
            /// Initializes a new modulator
            pub fn new() -> Self {
                Self {
                    value: 0,
                    sigma: [0; 2]
                }
            }

            /// Returns the next output value of the modulator
            pub fn update(&mut self) -> bool {
                let mut sigma_new: [$S; 2] = [0; 2];
                if self.sigma[1] >= 0 {
                    sigma_new[0] = self.sigma[0] + self.value as $S - <$T>::MAX as $S;
                    sigma_new[1] = self.sigma[1] + sigma_new[0] - <$T>::MAX as $S;
                } else {
                    sigma_new[0] = self.sigma[0] + self.value as $S;
                    sigma_new[1] = self.sigma[1] + sigma_new[0];
                }
                self.sigma = sigma_new;
                self.sigma[1] >= 0
            }
        }
    };
}

gen_unsigned_impl!(u64, i128);
gen_unsigned_impl!(u32, i64);
gen_unsigned_impl!(u16, i32);
gen_unsigned_impl!(u8, i16);

/******************************************************************************/

macro_rules! gen_signed_impl {
    ($T: ty, $S: ty) => {
        impl Modulator for $T {
            type SigmaType = $S;
        }

        impl Default for Pdm<$T> {
            fn default() -> Self {
                Self {
                    value: 0,
                    sigma: [0; 2]
                }
            }
        }

        impl Pdm<$T> {
            /// Initializes a new modulator
            pub fn new() -> Self {
                Default::default()
            }

            /// Returns the next output value of the modulator
            pub fn update(&mut self) -> bool {
                let mut sigma_new: [$S; 2] = [0; 2];
                if self.sigma[1] >= 0 {
                    sigma_new[0] = self.sigma[0] + self.value as $S - <$T>::MAX as $S;
                    sigma_new[1] = self.sigma[1] + sigma_new[0] - <$T>::MAX as $S;
                } else {
                    sigma_new[0] = self.sigma[0] + self.value as $S + <$T>::MAX as $S;
                    sigma_new[1] = self.sigma[1] + sigma_new[0] + <$T>::MAX as $S;
                }
                self.sigma = sigma_new;
                self.sigma[1] >= 0
            }
        }
    };
}

gen_signed_impl!(i64, i128);
gen_signed_impl!(i32, i64);
gen_signed_impl!(i16, i32);
gen_signed_impl!(i8, i16);

/******************************************************************************/

macro_rules! gen_float_impl {
    ($T: ty) => {
        impl Modulator for $T {
            type SigmaType = $T;
        }

        impl Default for Pdm<$T> {
            fn default() -> Self {
                Self {
                    value: 0.0,
                    sigma: [0.0; 2]
                }
            }
        }

        impl Pdm<$T> {
            /// Initializes a new modulator
            ///
            /// Values are expected between -1.0 and 1.0
            pub fn new() -> Self {
                Default::default()
            }

            /// Returns the next output value of the modulator
            pub fn update(&mut self) -> bool {
                let mut sigma_new: [$T; 2] = [0.0; 2];
                if self.sigma[1] >= 0.0 {
                    sigma_new[0] = self.sigma[0] + self.value as $T - 1.0 as $T;
                    sigma_new[1] = self.sigma[1] + sigma_new[0] - 1.0 as $T;
                } else {
                    sigma_new[0] = self.sigma[0] + self.value as $T + 1.0 as $T;
                    sigma_new[1] = self.sigma[1] + sigma_new[0] + 1.0 as $T;
                }
                self.sigma = sigma_new;
                self.sigma[1] >= 0.0
            }
        }
    };
}

gen_float_impl!(f64);
gen_float_impl!(f32);

/******************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! gen_unsigned_test {
        ($name: ident, $T: ty, $setpoint: expr, $iterations: literal) => {
            #[test]
            fn $name() {
                let mut pdm = Pdm::<$T>::new();
                pdm.set_value($setpoint);

                let mut avg = 0;
                for _ in 0..$iterations {
                    if pdm.update() {
                        avg += <$T>::MAX as u128;
                    }
                }
                let ratio = (avg as f64 / $iterations as f64) / $setpoint as f64;
                assert!(ratio >= 0.99 && ratio <= 1.01, "ratio: {}", ratio);
            }
        };
    }
    gen_unsigned_test!(test_u8, u8, 42, 500_000);
    gen_unsigned_test!(test_u16, u16, 42_000, 500_000);
    gen_unsigned_test!(test_u32, u32, 420_000_000, 2_000_000);
    gen_unsigned_test!(test_u64, u64, 1.223e18 as u64, 2_000_000);

    macro_rules! gen_signed_test {
        ($name: ident, $T: ty, $setpoint: expr, $iterations: literal) => {
            #[test]
            fn $name() {
                let mut pdm = Pdm::<$T>::new();
                pdm.set_value($setpoint);

                let mut avg = 0;
                for _ in 0..$iterations {
                    if pdm.update() {
                        avg += <$T>::MAX as i128;
                    } else {
                        avg -= <$T>::MAX as i128;
                    }
                }
                let ratio = (avg as f64 / $iterations as f64) / $setpoint as f64;
                assert!(ratio >= 0.99 && ratio <= 1.01, "ratio: {}", ratio);
            }
        };
    }
    gen_signed_test!(test_i8, i8, -42, 500_000);
    gen_signed_test!(test_i16, i16, 4_200, 500_000);
    gen_signed_test!(test_i32, i32, -420_000_000, 2_000_000);
    gen_signed_test!(test_i64, i64, 1.223e18 as i64, 2_000_000);

    macro_rules! gen_float_test {
        ($name: ident, $T: ty, $setpoint: expr, $iterations: literal) => {
            #[test]
            fn $name() {
                let mut pdm = Pdm::<$T>::new();
                pdm.set_value($setpoint);

                let mut avg = 0.0;
                for _ in 0..$iterations {
                    if pdm.update() {
                        avg += 1.0;
                    } else {
                        avg -= 1.0;
                    }
                }
                let ratio = (avg / $iterations as f64) / $setpoint as f64;
                assert!(ratio >= 0.99 && ratio <= 1.01, "ratio: {}", ratio);
            }
        };
    }
    gen_float_test!(test_f32, f32, 0.42, 500_000);
    gen_float_test!(test_f64, f64, -0.42, 500_000);
}
