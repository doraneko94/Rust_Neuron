use crate::neuron::Neuron;

pub struct Network {
    neurons: Vec<Neuron>,
    count: usize,
}

impl Network {
    pub fn new(n: usize) -> Network {
        let mut neurons: Vec<Neuron> = Vec::with_capacity(n);
        for _ in 0..n {
            neurons.push(Neuron::new(n));
        }
        Network { neurons, count: 0 }
    }

    pub fn run(&mut self, spike_train: &[Vec<u8>], dt: f64) -> Vec<u8> {
        use rayon::prelude::*;
        let count = self.count;
        self.count += 1;
        self.neurons
            .par_iter_mut()
            .map(|neuron| neuron.run(&spike_train[count], dt))
            .collect()
    }

    pub fn input(&mut self, current: f64) {
        self.neurons
            .iter_mut()
            .for_each(|neuron| neuron.set_ext(current));
    }
}
