use core::ops::{Add, Mul, Sub};

use num_complex::Complex;

use num_traits::{Float, FloatConst, NumCast, One, Zero};

pub trait FP:
    Sized
    + Copy
    + Float
    + Zero
    + One
    + FloatConst
    + From<f32>
    + From<u8>
    + Into<f64>
    + Into<Complex<Self>>
    + Add<Complex<Self>, Output = Complex<Self>>
    + Mul<Complex<Self>, Output = Complex<Self>>
    + Sub<Complex<Self>, Output = Complex<Self>>
{
    const N00_5: Self;
    const N0: Self;
    const N0_5: Self;
    const N1: Self;
    const N2: Self;
    const N3: Self;
    const N4: Self;
    const N5: Self;
    const N6: Self;
    const N7: Self;
    const N8: Self;
    const N9: Self;
    const N10: Self;
    const N20: Self;
    const N40: Self;
}

impl FP for f32 {
    const N00_5: f32 = 0.05;
    const N0: f32 = 0.0;
    const N0_5: f32 = 0.5;
    const N1: f32 = 1.0;
    const N2: f32 = 2.0;
    const N3: f32 = 3.0;
    const N4: f32 = 4.0;
    const N5: f32 = 5.0;
    const N6: f32 = 6.0;
    const N7: f32 = 7.0;
    const N8: f32 = 8.0;
    const N9: f32 = 9.0;
    const N10: f32 = 10.0;
    const N20: f32 = 20.0;
    const N40: f32 = 40.0;
}

impl FP for f64 {
    const N00_5: f64 = 0.05;
    const N0: f64 = 0.0;
    const N0_5: f64 = 0.5;
    const N1: f64 = 1.0;
    const N2: f64 = 2.0;
    const N3: f64 = 3.0;
    const N4: f64 = 4.0;
    const N5: f64 = 5.0;
    const N6: f64 = 6.0;
    const N7: f64 = 7.0;
    const N8: f64 = 8.0;
    const N9: f64 = 9.0;
    const N10: f64 = 10.0;
    const N20: f64 = 20.0;
    const N40: f64 = 40.0;
}

/// Used to implement conversions to the Hertz struct
pub trait Units<T> {
    /// From hertz
    fn to_range(self, bottom: T, top: T) -> T;
    fn from_range(self, bottom: T, top: T) -> T;
    fn db_to_lin(self) -> T;
    fn lin_to_db(self) -> T;
    fn sign(self, b: T) -> T;
    fn bw_to_q(self, f0: T, fs: T) -> T;
}

impl<T: FP> Units<T> for T {
    fn to_range(self, bottom: T, top: T) -> T {
        self * (top - bottom) + bottom
    }
    fn from_range(self, bottom: T, top: T) -> T {
        (self - bottom) / (top - bottom)
    }
    fn db_to_lin(self) -> T {
        T::N10.powf(self * T::N00_5)
    }
    fn lin_to_db(self) -> T {
        (self.max(T::N0)).log10() * T::N20
    }
    fn sign(self, b: T) -> T {
        if b < T::N0 {
            -self
        } else {
            self
        }
    }
    fn bw_to_q(self, _f0: T, _fs: T) -> T {
        let two = T::N2;
        T::N1 / (two * (T::LN_2() / two * self).sinh())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ZSample<T> {
    pub z: Complex<T>,
    pub pow1: Complex<T>,
    pub pow2: Complex<T>,
}

impl<T: FP> ZSample<T> {
    pub fn new(f_hz: T, fs: T) -> ZSample<T> {
        let z = -T::TAU() * f_hz / fs;
        let z: Complex<T> =
            Into::<T>::into(z.cos()) + Into::<T>::into(z.sin()) * Complex::<T>::new(T::N0, T::N1);
        ZSample {
            z,
            pow1: z,
            pow2: z * z,
        }
    }
}

pub fn butterworth_cascade_q<T: FP>(filter_order: u8, pole: u8) -> T {
    let mut pole = pole;
    let pole_inc: T = T::PI() / (NumCast::from(filter_order).unwrap());
    let even_order = filter_order & 1 == 0;
    let point_five = T::N0_5;
    let two: T = T::N2;

    let first_angle = if even_order {
        pole_inc * point_five
    } else {
        if pole == 0 {
            return point_five; //Also needs to be 1 pole (not biquad)
        }
        pole -= 1;
        pole_inc
    };
    let fpole: T = NumCast::from(pole).unwrap();
    let a: T = first_angle + fpole * pole_inc;
    T::N1 / (two * a.cos())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_butterworth_cascade_q() {
        assert_eq!(0.7071067811865475, butterworth_cascade_q(2, 0));

        assert_eq!(0.5, butterworth_cascade_q(3, 0));
        assert_eq!(0.9999999999999998, butterworth_cascade_q(3, 1));

        assert_eq!(0.541196100146197, butterworth_cascade_q(4, 0));
        assert_eq!(1.3065629648763764, butterworth_cascade_q(4, 1));

        assert_eq!(0.5, butterworth_cascade_q(5, 0));
        assert_eq!(0.6180339887498948, butterworth_cascade_q(5, 1));
        assert_eq!(1.6180339887498947, butterworth_cascade_q(5, 2));

        assert_eq!(0.5176380902050415, butterworth_cascade_q(6, 0));
        assert_eq!(0.7071067811865475, butterworth_cascade_q(6, 1));
        assert_eq!(1.931851652578135, butterworth_cascade_q(6, 2));
        dbg!(butterworth_cascade_q::<f64>(5, 2));
    }
}
