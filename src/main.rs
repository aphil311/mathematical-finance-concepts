use rand::prelude::*;
use rand_distr::StandardNormal;

mod asian;

fn main() {
    //------------------------------------------------------------------
    // Monte Carlo Pricing
    //------------------------------------------------------------------
    // parse arguments
    // get core values
    let underlying = 195.0;
    let strike = 200.0;
    let rate = 0.05;
    let vol = 0.3;
    let expiry = 0.1;
    let num_sims = 10;
    let days = (expiry * 365.0) as i64;
    let dt = expiry / days as f64;

    // loop over simulations
    let mut sum = 0.0;
    for _ in 0..num_sims {
        // loop over steps
        let mut change = 0.0;
        for _ in 0..days{
            // generate random number
            let t1 = underlying * rate * dt;
            let t2 = underlying * vol * thread_rng().sample::<f64,_>(StandardNormal) * dt.sqrt();
            let ds = t1 + t2;
            // print dS
            change += ds;
        }
        let payoff = (underlying + change) - strike;
        if payoff > 0.0 {
            sum += payoff;
        }
    }
    let average_payoff = sum / num_sims as f64;
    let discounted_payoff = average_payoff * (-rate * expiry).exp();
    println!("Monte Carlo Price: {discounted_payoff}");
    let asian_call = asian::price_option(expiry, strike, underlying, rate, vol, num_sims, 0.12);
    println!("Asian Call Price: {asian_call}");
}