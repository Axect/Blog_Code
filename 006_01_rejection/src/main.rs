use peroxide::fuga::*;
use rayon::prelude::*;
use Kernel::*;

const N: usize = 100000;
const LAMBDA: f64 = 0.4;

#[allow(non_snake_case)]
fn main() {
    let M = f(-0.5);
    // Create Uniform Distribution
    let g = Uniform(-0.5, 10.5);
    let h = Uniform(0.0, M);

    let mut x_vec = vec![0.0; N];
    let mut i = 0usize;
    while i < N {
        let x = g.sample(1)[0];
        
        // Rejection Sampling
        let y = h.sample(1)[0];
        if y <= f(x) {
            x_vec[i] = x;
            i += 1;
        } else {
            continue;
        }
    }

    let kde = |x: f64| parzen_kde(x, &x_vec, LAMBDA, Epanechnikov);
    let x_domain = linspace(0.0, 10.0, N);
    let pdf_estimate = x_domain.fmap(kde);

    let mut df = DataFrame::new(vec![]);
    df.push("x", Series::new(x_vec));
    df.push("x_domain", Series::new(x_domain));
    df.push("pdf", Series::new(pdf_estimate));

    df.print();

    df.write_nc("data/test.nc").unwrap();
}

fn f(x: f64) -> f64 {
    1f64 / (x+1f64).sqrt() + 0.2 * (-(x-3f64).powi(2) / 0.2).exp()
}

#[derive(Debug, Copy, Clone)]
enum Kernel {
    Gaussian,
    Epanechnikov,
}

fn parzen_kde(x0: f64, x_vec: &Vec<f64>, lambda: f64, kernel: Kernel) -> f64 {
    let n = x_vec.len();
    match kernel {
        Kernel::Epanechnikov => {
            x_vec.par_iter().fold(|| 0f64, |acc, &x| acc + epanechnikov_kernel(x, x0, lambda)).sum::<f64>() / (n as f64 * lambda)
        },
        Kernel::Gaussian => {
            x_vec.par_iter().fold(|| 0f64, |acc, &x| acc + gaussian_kernel(x, x0, lambda)).sum::<f64>() / (n as f64 * lambda)
        },
    }
}

fn gaussian_kernel(x: f64, x0: f64, lambda: f64) -> f64 {
    (- (x - x0).powi(2) / (2f64 * lambda.powi(2))).exp()
}

fn epanechnikov_kernel(x: f64, x0: f64, lambda: f64) -> f64 {
    let t = (x - x0).abs() / lambda;
    if t < 1f64 {
        0.75f64 * (1f64 - t.powi(2))
    } else {
        0f64
    }
}
