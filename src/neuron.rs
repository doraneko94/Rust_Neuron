use rand::distributions::{Bernoulli, Distribution, Uniform};

pub struct Neuron {
    synapses: Vec<(usize, f64)>,
    v: f64,
    i_ext: f64,
    threshold: f64,
    t_rest: f64,
}

impl Neuron {
    pub fn new(n: usize) -> Neuron {
        let bd = Bernoulli::new(0.5).unwrap();
        let ud = Uniform::new(0.0, 10.0);
        let synapses = (0..n)
            .filter_map(|i| {
                if bd.sample(&mut rand::thread_rng()) {
                    let weight = ud.sample(&mut rand::thread_rng());
                    Some((i, weight))
                } else {
                    None
                }
            })
            .collect();
        Neuron {
            synapses,
            v: 0.0,
            i_ext: 0.0,
            threshold: 10.0,
            t_rest: 0.0,
        }
    }

    fn calc_dv(&self, spike: &[u8], dt: f64) -> f64 {
        let i_rec = sum_rec(&self.synapses, spike);
        let noise = Uniform::new(0.0, 1.0).sample(&mut rand::thread_rng());
        let i_ext = self.i_ext * (1.0 + self.i_ext * noise);
        let d_v = |y: f64| (-y + 1.0 * (i_rec + i_ext)) / 50.0;
        rk4(d_v, self.v, dt)
    }

    pub fn run(&mut self, spike: &[u8], dt: f64) -> u8 {
        if self.t_rest > 0.0 {
            self.t_rest -= dt;
            if self.t_rest <= 0.0 {
                self.v = 0.0;
            }
            1
        } else {
            self.v += self.calc_dv(spike, dt);
            if self.v > self.threshold {
                self.t_rest = 2.0;
                1
            } else {
                0
            }
        }
    }

    pub fn set_ext(&mut self, current: f64) {
        self.i_ext = current;
    }

    pub fn get_v(&self) -> f64 {
        self.v
    }
}

fn sum_rec(synapses: &[(usize, f64)], spike: &[u8]) -> f64 {
    synapses
        .iter()
        .filter(|(i, _)| spike[*i] == 1)
        .map(|(_, w)| w)
        .sum()
}

fn rk4<F: Fn(f64) -> f64>(f: F, y: f64, h: f64) -> f64 {
    let k1 = h * f(y);
    let k2 = h * f(y + 0.5 * k1);
    let k3 = h * f(y + 0.5 * k2);
    let k4 = h * f(y + k3);
    (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
}
