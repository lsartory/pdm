/* pdm | lib.rs
 * Copyright (c) 2025 L. Sartory
 * SPDX-License-Identifier: MIT
 */

/******************************************************************************/

#![no_std]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/******************************************************************************/

/// Pulse Density Modulator
pub struct Pdm<T: Modulator> {
    value: T,
    sigma: [T::SigmaType; 2]
}

impl<T> Default for Pdm<T>
where
    T: Modulator,
    T::SigmaType: Modulator
{
    fn default() -> Self {
        assert!(T::DELTA_POS != T::DELTA_NEG, "Unsupported type!");
        Self {
            value: T::ZERO,
            sigma: [T::SigmaType::ZERO; 2]
        }
    }
}

impl<T> Pdm<T>
where
    T: Modulator,
    T::SigmaType: Modulator
{
    /// Initializes a new modulator
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the modulator value
    ///
    /// Valid range is 0..T::MAX for unsigned types, -T::MAX..T::MAX for signed types, and -1.0..1.0 for
    /// float types.
    pub fn set_value(&mut self, value: T) {
        self.value = value
    }

    /// Returns the next output value of the modulator
    pub fn update(&mut self) -> bool {
        let delta = if self.sigma[1] >= T::SigmaType::ZERO {
            T::DELTA_NEG
        } else {
            T::DELTA_POS
        };
        self.sigma[0] = self.sigma[0] + self.value.into() + delta;
        self.sigma[1] = self.sigma[1] + self.sigma[0] + delta;
        self.sigma[1] >= T::SigmaType::ZERO
    }
}

/******************************************************************************/

/// A trait that allow generic modulator implementations
pub trait Modulator: Copy + core::ops::Add<Output = Self> + Into<Self::SigmaType> {
    /// The data type used for the integrator
    type SigmaType: PartialEq + PartialOrd;

    /// Zero constant
    const ZERO: Self;
    /// Positive delta value
    const DELTA_POS: Self::SigmaType;
    /// Negative delta value
    const DELTA_NEG: Self::SigmaType;
}

macro_rules! modulator_impl {
    ($T: ty, $S: ty, $ZERO: literal, $DELTA_POS: expr, $DELTA_NEG: expr) => {
        impl Modulator for $T {
            type SigmaType = $S;

            const ZERO: Self = $ZERO;
            const DELTA_POS: Self::SigmaType = $DELTA_POS;
            const DELTA_NEG: Self::SigmaType = $DELTA_NEG;
        }
    };
}

modulator_impl!(i128, i128, 0, 0, 0);

modulator_impl!(i64, i128, 0, i64::MAX as i128, -(i64::MAX as i128));
modulator_impl!(i32, i64,  0, i32::MAX as i64,  -(i32::MAX as i64));
modulator_impl!(i16, i32,  0, i16::MAX as i32,  -(i16::MAX as i32));
modulator_impl!(i8,  i16,  0,  i8::MAX as i16,  -( i8::MAX as i16));

modulator_impl!(u64, i128, 0, 0, -(u64::MAX as i128));
modulator_impl!(u32, i64,  0, 0, -(u32::MAX as i64));
modulator_impl!(u16, i32,  0, 0, -(u16::MAX as i32));
modulator_impl!(u8,  i16,  0, 0, -( u8::MAX as i16));

modulator_impl!(f32, f32, 0.0, 1.0, -1.0);
modulator_impl!(f64, f64, 0.0, 1.0, -1.0);

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

                let mut avg = 0.0;
                for _ in 0..$iterations {
                    avg += if pdm.update() {
                        <$T>::DELTA_POS
                    } else {
                        <$T>::DELTA_NEG
                    } as f64;
                }
                let ratio = (avg / $iterations as f64) / $setpoint as f64;
                assert!(ratio >= 0.99 && ratio <= 1.01, "ratio: {}", ratio);
            }
        };
    }
    gen_signed_test!(test_i8, i8, -42, 500_000);
    gen_signed_test!(test_i16, i16, 4_200, 500_000);
    gen_signed_test!(test_i32, i32, -420_000_000, 2_000_000);
    gen_signed_test!(test_i64, i64, 1.223e18 as i64, 2_000_000);

    gen_signed_test!(test_f32, f32, 0.42, 500_000);
    gen_signed_test!(test_f64, f64, -0.42, 500_000);
}
