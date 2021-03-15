extern crate peroxide;
use peroxide::fuga::*;

fn main() {
    let n = Normal(60, 20);
    //let eps = Normal(0, 5);
    //let y = n.sample(100).add_v(&eps.sample(100)).filter(|x| x <= 100f64);
    let y = n.sample(40).iter().map(|t| t.round()).filter(|x| *x <= 100f64).collect();
    
    let mut df = DataFrame::new(vec![]);
    df.push("y", Series::new(y));

    df.print();

    df.write_nc("data.nc").expect("Can't write nc");
}
