use rand::distributions::{Bernoulli, Distribution, Uniform};

pub struct Neuron {
    synapses: Vec<usize>,
    weights: Vec<f64>,
    v: f64,
    i_ext: f64,
    threshold: f64,
    t_rest: f64,
}

impl Neuron {
    pub fn new(n: usize) -> Neuron {
        let mut synapses: Vec<usize> = Vec::new();
        let mut weights: Vec<f64> = Vec::new();
        let bd = Bernoulli::new(0.5).unwrap();
        let ud = Uniform::new(0.0, 10.0);
        for i in 0..n {
            if bd.sample(&mut rand::thread_rng()) {
                synapses.push(i);
                weights.push(ud.sample(&mut rand::thread_rng()));
            }
        }
        Neuron {
            synapses,
            weights,
            v: 0.0,
            i_ext: 0.0,
            threshold: 10.0,
            t_rest: 0.0,
        }
    }

    pub fn run(&mut self, spike: &[u8], dt: f64) -> u8 {
        if self.t_rest > 0.0 {
            self.t_rest -= dt;
            if self.t_rest <= 0.0 {
                self.v = 0.0;
            }
            1
        } else {
            let dist = Uniform::new(0.0, 1.0);
            let mut i_rec = 0.0;
            for (i, synapse) in self.synapses.iter().enumerate() {
                if spike[*synapse] == 1 {
                    i_rec += self.weights[i];
                }
            }
            let i_ext = self.i_ext * (1.0 + self.i_ext * dist.sample(&mut rand::thread_rng()));
            let d_v = |y: f64| (-y + 1.0 * (i_rec + i_ext)) / 50.0;
            self.v += rk4(d_v, self.v, dt);
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
}

fn rk4<F: Fn(f64) -> f64>(f: F, y: f64, h: f64) -> f64 {
    let k1 = h * f(y);
    let k2 = h * f(y + 0.5 * k1);
    let k3 = h * f(y + 0.5 * k2);
    let k4 = h * f(y + k3);
    (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
}
