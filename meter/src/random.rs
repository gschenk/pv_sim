use rand::thread_rng;
use rand_distr::{Normal, Distribution};


// simulator returns a random power value
pub fn simulator(power: f64, xmin: f64, xmax: f64, sigma: f64) -> f64 {

    // normalize power
    let offset = xmin;
    let scale = xmax - xmin;
    let x = (power - offset)/scale;

    return scale * random_walker(x, sigma) + offset
}

// creates a new step in a random walk values are 
// it takes a previous value and the standard deviation
fn random_walker(x: f64, s: f64) -> f64{
    let sigma = s.min(0.1); // dont let steps get too big

    let mut rng = thread_rng();

    // soft boundaries move the distribution mean, the next
    // step goes towads the middle
    let mean = (1.0 - x).min(sigma) - x.min(sigma) ;

    let normal = Normal::new(mean, sigma).unwrap();
    let v = normal.sample(&mut rng);

    return (x + v).max(0.0).min(1.0); // hard boundaries
}
