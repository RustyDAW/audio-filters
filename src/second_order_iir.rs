use num_complex::Complex;

use crate::{units::ZSample, MAX_CASCADE_COUNT};

use crate::units::FP;

#[derive(Copy, Clone, Debug)]
pub struct IIR2Coefficients<T: FP> {
    pub a: T,
    pub g: T,
    pub gpow2: T,
    pub k: T,
    pub a1: T,
    pub a2: T,
    pub a3: T,
    pub m0: T,
    pub m1: T,
    pub m2: T,
}

impl<T: FP> IIR2Coefficients<T> {
    pub fn get_bode_sample(self, z: ZSample<T>) -> Complex<T> {
        //Use y.norm() for amplitude and y.arg().to_degrees() for phase. Add to combine phase.

        let denominator = (self.gpow2 + self.g * self.k + T::N1)
            + T::N2 * (self.gpow2 - T::N1) * z.pow1
            + (self.gpow2 - self.g * self.k + T::N1) * z.pow2;

        let y = self.m0
            + (self.m1 * self.g * (T::N1 - z.pow2)
                + self.m2 * self.gpow2 * (T::N1 + T::N2 * z.pow1 + z.pow2))
                / denominator;

        y
    }

    //TODO make const once possible
    pub fn empty() -> IIR2Coefficients<T> {
        IIR2Coefficients {
            a: T::N0,
            g: T::N0,
            gpow2: T::N0,
            k: T::N0,
            a1: T::N0,
            a2: T::N0,
            a3: T::N0,
            m0: T::N0,
            m1: T::N0,
            m2: T::N0,
        }
    }

    pub fn empty_cascade() -> [IIR2Coefficients<T>; MAX_CASCADE_COUNT] {
        [IIR2Coefficients::empty(); MAX_CASCADE_COUNT]
    }

    pub fn lowpass(
        cutoff_hz: T,
        _gain_db: T,
        q_value: T,
        sample_rate_hz: T,
    ) -> IIR2Coefficients<T> {
        let cutoff_hz = cutoff_hz.min(sample_rate_hz * T::N0_5);
        let a = T::N1;
        let g = (T::PI() * cutoff_hz / sample_rate_hz).tan();
        let k = T::N1 / q_value;
        let a1 = T::N1 / (T::N1 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = T::N0;
        let m1 = T::N0;
        let m2 = T::N1;
        IIR2Coefficients {
            a,
            g,
            gpow2: g * g,
            k,
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }
    pub fn highpass(
        cutoff_hz: T,
        _gain_db: T,
        q_value: T,
        sample_rate_hz: T,
    ) -> IIR2Coefficients<T> {
        let cutoff_hz = cutoff_hz.min(sample_rate_hz * T::N0_5);
        let a = T::N1;
        let g = (T::PI() * cutoff_hz / sample_rate_hz).tan();
        let k = T::N1 / q_value;
        let a1 = T::N1 / (T::N1 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = T::N1;
        let m1 = -k;
        let m2 = -T::N1;
        IIR2Coefficients {
            a,
            g,
            gpow2: g * g,
            k,
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }
    pub fn bandpass(
        cutoff_hz: T,
        _gain_db: T,
        q_value: T,
        sample_rate_hz: T,
    ) -> IIR2Coefficients<T> {
        let cutoff_hz = cutoff_hz.min(sample_rate_hz * T::N0_5);
        let a = T::N1;
        let g = (T::PI() * cutoff_hz / sample_rate_hz).tan();
        let k = T::N1 / q_value;
        let a1 = T::N1 / (T::N1 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = T::N0;
        let m1 = T::N1;
        let m2 = T::N0;
        IIR2Coefficients {
            a,
            g,
            gpow2: g * g,
            k,
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }
    pub fn notch(cutoff_hz: T, _gain_db: T, q_value: T, sample_rate_hz: T) -> IIR2Coefficients<T> {
        let cutoff_hz = cutoff_hz.min(sample_rate_hz * T::N0_5);
        let a = T::N1;
        let g = (T::PI() * cutoff_hz / sample_rate_hz).tan();
        let k = T::N1 / q_value;
        let a1 = T::N1 / (T::N1 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = T::N1;
        let m1 = -k;
        let m2 = T::N0;
        IIR2Coefficients {
            a,
            g,
            gpow2: g * g,
            k,
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }
    pub fn allpass(
        cutoff_hz: T,
        _gain_db: T,
        q_value: T,
        sample_rate_hz: T,
    ) -> IIR2Coefficients<T> {
        let cutoff_hz = cutoff_hz.min(sample_rate_hz * T::N0_5);
        let a = T::N1;
        let g = (T::PI() * cutoff_hz / sample_rate_hz).tan();
        let k = T::N1 / q_value;
        let a1 = T::N1 / (T::N1 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = T::N1;
        let m1 = -T::N2 * k;
        let m2 = T::N0;
        IIR2Coefficients {
            a,
            g,
            gpow2: g * g,
            k,
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }
    pub fn lowshelf(
        cutoff_hz: T,
        gain_db: T,
        q_value: T,
        sample_rate_hz: T,
    ) -> IIR2Coefficients<T> {
        let cutoff_hz = cutoff_hz.min(sample_rate_hz * T::N0_5);
        let a = T::N10.powf(gain_db / T::N40);
        let g = (T::PI() * cutoff_hz / sample_rate_hz).tan() / a.sqrt();
        let k = T::N1 / q_value;
        let a1 = T::N1 / (T::N1 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = T::N1;
        let m1 = k * (a - T::N1);
        let m2 = a * a - T::N1;
        IIR2Coefficients {
            a,
            g,
            gpow2: g * g,
            k,
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }
    pub fn highshelf(
        cutoff_hz: T,
        gain_db: T,
        q_value: T,
        sample_rate_hz: T,
    ) -> IIR2Coefficients<T> {
        let cutoff_hz = cutoff_hz.min(sample_rate_hz * T::N0_5);
        let a = T::N10.powf(gain_db / T::N40);
        let g = (T::PI() * cutoff_hz / sample_rate_hz).tan() * a.sqrt();
        let k = T::N1 / q_value;
        let a1 = T::N1 / (T::N1 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = a * a;
        let m1 = k * (T::N1 - a) * a;
        let m2 = T::N1 - a * a;
        IIR2Coefficients {
            a,
            g,
            gpow2: g * g,
            k,
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }
    pub fn bell(cutoff_hz: T, gain_db: T, q_value: T, sample_rate_hz: T) -> IIR2Coefficients<T> {
        let cutoff_hz = cutoff_hz.min(sample_rate_hz * T::N0_5);
        let a = T::N10.powf(gain_db / T::N40);
        let g = (T::PI() * cutoff_hz / sample_rate_hz).tan();
        let k = T::N1 / (q_value * a);
        let a1 = T::N1 / (T::N1 + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        let m0 = T::N1;
        let m1 = k * (a * a - T::N1);
        let m2 = T::N0;
        IIR2Coefficients {
            a,
            g,
            gpow2: g * g,
            k,
            a1,
            a2,
            a3,
            m0,
            m1,
            m2,
        }
    }
}

/// Internal states and coefficients of the SVF form
#[derive(Copy, Clone, Debug)]
pub struct IIR2<T: FP> {
    ic1eq: T,
    ic2eq: T,
    pub coeffs: IIR2Coefficients<T>,
}

impl<T: FP> IIR2<T> {
    /// Creates a SVF from a set of filter coefficients
    pub fn new(coefficients: IIR2Coefficients<T>) -> Self {
        IIR2 {
            ic1eq: T::N0,
            ic2eq: T::N0,
            coeffs: coefficients,
        }
    }

    pub fn process(&mut self, input_sample: T) -> T {
        let v3 = input_sample - self.ic2eq;
        let v1 = self.coeffs.a1 * self.ic1eq + self.coeffs.a2 * v3;
        let v2 = self.ic2eq + self.coeffs.a2 * self.ic1eq + self.coeffs.a3 * v3;
        self.ic1eq = T::N2 * v1 - self.ic1eq;
        self.ic2eq = T::N2 * v2 - self.ic2eq;

        self.coeffs.m0 * input_sample + self.coeffs.m1 * v1 + self.coeffs.m2 * v2
    }

    pub fn process_partial(&mut self, input_sample: T) -> (T, T) {
        let v3 = input_sample - self.ic2eq;
        let v1 = self.coeffs.a1 * self.ic1eq + self.coeffs.a2 * v3;
        let v2 = self.ic2eq + self.coeffs.a2 * self.ic1eq + self.coeffs.a3 * v3;
        self.ic1eq = T::N2 * v1 - self.ic1eq;
        self.ic2eq = T::N2 * v2 - self.ic2eq;

        (v1, v2)
    }

    pub fn update_coefficients(&mut self, new_coefficients: IIR2Coefficients<T>) {
        self.coeffs = new_coefficients;
    }
}
