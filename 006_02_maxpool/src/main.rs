use peroxide::fuga::*;

#[allow(non_snake_case)]
fn main() {
    // Read data - ALP from PBH
    let df = DataFrame::read_nc("data/DM_primary_spectrum.nc").unwrap();
    let E: Vec<f64> = df["E"].to_vec().mul_s(1e+3);
    let d2NdEdt: Vec<f64> = df["d2NdEdt"].to_vec().mul_s(1e-3);
    let d2NdEdt_shape: Vec<usize> = df["d2NdEdt_shape"].to_vec().into_iter().map(|x: u64| x as usize).collect();
    let d2NdEdt = matrix(d2NdEdt, d2NdEdt_shape[0], d2NdEdt_shape[1], Row);
    let dNdE = d2NdEdt.row(0);

    // Cubic Hermite Spline
    let cs = cubic_hermite_spline(&E, &dNdE, Quadratic);
    let dN_dE_ = |x : f64| cs.eval(x);
    let E_trial = seq(0, 200, 0.1);
    let dNdE_trial = E_trial.fmap(dN_dE_);

    // Create weighted unform distribution
    let wu = WeightedUniform::from_max_pool_1d(
        |x| cs.eval(x),
        (0f64, 200f64),
        10,
        1e-2,
    );

    // Get max pool
    let maxpool = E_trial.fmap(|x| wu.weight_at(x));

    // Write a dataframe
    let mut dg = DataFrame::new(vec![]);
    dg.push("E", Series::new(E_trial));
    dg.push("dNdE", Series::new(dNdE_trial));
    dg.push("maxpool", Series::new(maxpool));
    dg.print();

    dg.write_nc("data/test.nc").unwrap();
}
