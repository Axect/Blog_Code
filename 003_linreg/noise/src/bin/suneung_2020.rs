extern crate peroxide;
use peroxide::fuga::*;

fn main() {
    let n = Normal(59.87, 20.22);
    (1f64 - n.cdf(91)).print();
}
