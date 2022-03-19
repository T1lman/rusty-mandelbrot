use num::complex::Complex;

mod utils;

fn main() {
    let test = utils::mandelbrot(1, Complex::new(0.001, 0.002));
    println!("{:?}", test)
}
