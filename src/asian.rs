use rand::prelude::*;
use rand_distr::StandardNormal;

const DT:f64 = 1.0 / 365.0;

//----------------------------------------------------------------------
// Asian Option
// naively price an asian option using the monte carlo method
//----------------------------------------------------------------------
pub fn price_option(expiry: f64, strike: f64, underlying: f64, rate: f64, vol: f64, num_sims: i64, mu: f64) -> f64 {
    let num_steps = (expiry / DT) as i64;
    let mut sum = 0.0;
    for _ in 0..num_sims {
        let mut stock_price = underlying;
        let mut price_sum = 0.0;
        for _ in 0..num_steps {
            let t1 = (mu - (vol * vol) / 2.0) * DT as f64;
            let t2 = vol * thread_rng().sample::<f64,_>(StandardNormal) * DT.sqrt() as f64;
            let ds = t1 + t2;
            let ds = ds.exp();
            stock_price *= ds;
            price_sum += stock_price;
        }
        let avg_price = price_sum / num_steps as f64;
        let payoff = avg_price - strike;
        if payoff > 0.0 {
            sum += payoff;
        }
    }
    let average_payoff = sum / num_sims as f64;
    let discounted_payoff = average_payoff * (-rate * expiry).exp();
    return discounted_payoff;
}