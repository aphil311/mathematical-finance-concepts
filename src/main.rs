use rand::prelude::*;
use rand_distr::StandardNormal;

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
    let expiry = 1.0;
    let num_sims = 1000;
    let num_steps = 100;
    let dt = expiry / num_steps as f64;

    // loop over simulations
    let mut sum = 0.0;
    for _ in 0..num_sims {
        // loop over steps
        let mut delta = 0.0;
        for _ in 0..num_steps{
            // generate random number
            let t1 = underlying * rate * dt;
            let t2 = underlying * vol * thread_rng().sample::<f64,_>(StandardNormal) * dt.sqrt();
            let ds = t1 + t2;
            // print dS
            delta += ds;
        }
        let payoff = (underlying + delta) - strike;
        if payoff > 0.0 {
            sum += payoff;
        }
    }
    let average_payoff = sum / num_sims as f64;
    let discounted_payoff = average_payoff * (-rate * expiry).exp();
    println!("Monte Carlo Price: {discounted_payoff}");
}