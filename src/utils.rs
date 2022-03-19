use num::complex::Complex;

pub fn mandelbrot(iterations: usize, start: Complex<f64>) -> Vec<Complex<f64>> {
    let mut retunvec: Vec<Complex<f64>> = Vec::new();
    let mut c: Complex<f64> = Complex::new(0.0, 0.0);
    for _ in 0..iterations {
        let current = c * c + start;
        retunvec.push(current);
        c = current;
    }
    retunvec
}
