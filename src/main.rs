
#[derive(Clone, Copy, Debug)]
pub struct BiquadCoefficients {
    pub a1: f64,
    pub a2: f64,
    pub b0: f64,
    pub b1: f64,
    pub b2: f64,
}

impl BiquadCoefficients {
    pub fn new(a1: f64, a2: f64, b0: f64, b1: f64, b2: f64) -> Self {
        BiquadCoefficients { a1, a2, b0, b1, b2 }
    }
}


pub struct Biquad {
    pub s1: f64,
    pub s2: f64,
    coeffs: BiquadCoefficients,
}

impl Biquad {

    /// Creates a Direct Form 2 Transposed biquad filter from a set of coefficients
    pub fn new(coefficients: BiquadCoefficients) -> Self {
        Biquad {
            s1: 0.0,
            s2: 0.0,
            coeffs: coefficients,
        }
    }

    /// Process a single sample
    fn process_single(&mut self, input: f64) -> f64 {
        let out = self.s1 + self.coeffs.b0 * input;
        self.s1 = self.s2 + self.coeffs.b1 * input - self.coeffs.a1 * out;
        self.s2 = self.coeffs.b2 * input - self.coeffs.a2 * out;
        out
    }

    #[inline(never)]
    fn process_waveform(&mut self, waveform: &mut [f64]) {
        for item in waveform.iter_mut() {
            *item = self.process_single(*item);
        }
    }
}


fn main() {
    let params = BiquadCoefficients::new(0.1, 0.2, 0.3, 0.4, 0.5);
    let mut bq = Biquad::new(params);
    let mut wf = vec![1.0; 1024];
    bq.process_waveform(&mut wf)
}
