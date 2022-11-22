use peroxide::fuga::*;

#[allow(non_snake_case)]
fn main() {
    let df = DataFrame::read_nc("data/test.nc").unwrap();
    let E: Vec<f64> = df["E"].to_vec();
    let dNdE: Vec<f64> = df["dNdE"].to_vec();

    let cs = cubic_hermite_spline(&E, &dNdE, Quadratic);
    let f = |x: f64| cs.eval(x);

    let E_sample = prs(f, 10000, (E[0], E[E.len()-1]), 100, 1e-6);

    let mut df = DataFrame::new(vec![]);
    df.push("E", Series::new(E_sample));

    df.print();

    df.write_nc("data/prs.nc").unwrap();
}
