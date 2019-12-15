use crate::neuron::Neuron;

pub struct Network {
    neurons: Vec<Neuron>,
    count: usize,
}

impl Network {
    pub fn new(n: usize) -> Network {
        Network {
            neurons: (0..n).map(|_| Neuron::new(n)).collect(),
            count: 0,
        }
    }

    pub fn run(&mut self, spike_train: &[Vec<u8>], dt: f64) -> Vec<u8> {
        use rayon::prelude::*;
        let old_spike = &spike_train[self.count];
        self.count += 1;
        // rayon は作業の割当を work stealing で行うため
        // neuron が多い場合には自動で負荷のバランスが取れる
        self.neurons
            .par_iter_mut()
            .map(|neuron| neuron.run(old_spike, dt))
            .collect()
    }

    pub fn input(&mut self, current: f64) {
        self.neurons
            .iter_mut()
            .for_each(|neuron| neuron.set_ext(current));
    }
}
