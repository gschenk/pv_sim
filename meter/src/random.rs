use rand::thread_rng;
use rand_distr::{Normal, Distribution};

// simulator returns a random power value
pub fn simulator(time: f64, power: f64, pmin: f64, pmax: f64, sigma: f64) -> f64 {

    // scale is reduced by enveloping curve
    let scale = (pmax - pmin) * envelope(time);

    // normalize power
    let x = (power - pmin)/scale;

    let p = scale * random_walker(x, sigma) + pmin;
    return p
}

// creates a new step in a random walk values are 
// it takes a previous value and the standard deviation
fn random_walker(x: f64, s: f64) -> f64{
    let sigma = s.min(0.1); // dont let steps get too big

    let mut rng = thread_rng();

    // soft boundaries move the distribution mean, the next
    // step goes towads the middle
    let mean = ((1.0 - x).min(sigma) - x.min(sigma) ) * 1.0;

    let normal = Normal::new(mean, sigma).unwrap();
    let v = normal.sample(&mut rng);

    return (x + v).max(0.0).min(1.0); // hard boundaries
}

// changes scale with time of day (fraction)
fn envelope(x: f64) -> f64 {
    let min = 0.1; // curves are offset by this

    let c = 0.07; // width of gaussian
    let x0 = 0.55; // centre of curve
    let sep = 2.0 * c;

    // two gaussing curves separated by sep
    let g1 =  gaussian(x, x0 - sep, c);
    let g2 =  gaussian(x, x0 + sep, c);

    // returns offset and normalized sum of both curves
    // squrt to make curves smoother
    return min + (1.0 - min) * (g1 + g2).min(1.0).sqrt()
}

// standard gaussian function
fn gaussian(x: f64, b: f64, c: f64) -> f64 {
    return (-0.5 * (x-b).powi(2)/c.powi(2)).exp()
}
