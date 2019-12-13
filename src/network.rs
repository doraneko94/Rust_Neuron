use crate::neuron::Neuron;

pub struct Network {
    n: usize,
    neurons: Vec<Neuron>,
    count: usize,
}

impl Network {
    pub fn new(n: usize) -> Network {
        let mut neurons: Vec<Neuron> = Vec::with_capacity(n);
        for _ in 0..n {
            neurons.push(Neuron::new(n));
        }
        Network {
            n,
            neurons,
            count: 0,
        }
    }

    pub fn run(&mut self, spike_train: &[Vec<u8>], dt: f64) -> Vec<u8> {
        let mut spike: Vec<u8> = vec![0; self.n];
        for i in 0..self.n {
            spike[i] = self.neurons[i].run(&spike_train[self.count], dt);
        }
        self.count += 1;
        spike
    }

    pub fn input(&mut self, current: f64) {
        for i in 0..self.n {
            self.neurons[i].set_ext(current);
        }
    }
}

