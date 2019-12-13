use gnuplot::AxesCommon;
use gnuplot::*;
use rand::random;

fn rk4<F: Fn(f64) -> f64>(f: F, y: f64, h: f64) -> f64 {
    let k1 = h * f(y);
    let k2 = h * f(y + 0.5 * k1);
    let k3 = h * f(y + 0.5 * k2);
    let k4 = h * f(y + k3);
    (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
}

struct Network {
    n: usize,
    neurons: Vec<Neuron>,
    count: usize,
}

impl Network {
    fn new(n: usize) -> Network {
        let mut neurons: Vec<Neuron> = Vec::with_capacity(n);
        for _ in 0..n {
            neurons.push(Neuron::new(n));
        }
        Network {
            n: n,
            neurons: neurons,
            count: 0,
        }
    }

    fn run(&mut self, spike_train: &Vec<Vec<u8>>, dt: f64) -> Vec<u8> {
        let mut spike: Vec<u8> = vec![0; self.n];
        for i in 0..self.n {
            spike[i] = self.neurons[i].run(&spike_train[self.count], dt);
        }
        self.count += 1;
        spike
    }

    fn input(&mut self, current: f64) {
        for i in 0..self.n {
            self.neurons[i].i_ext = current;
        }
    }
}

struct Neuron {
    synapses: Vec<usize>,
    weights: Vec<f64>,
    v: f64,
    i_ext: f64,
    threshold: f64,
    t_rest: f64,
}

impl Neuron {
    fn new(n: usize) -> Neuron {
        let mut synapses: Vec<usize> = Vec::new();
        let mut weights: Vec<f64> = Vec::new();
        for i in 0..n {
            if random::<f64>() < 0.5 {
                synapses.push(i);
                weights.push(random::<f64>() * 10.0);
            }
        }
        Neuron {
            synapses: synapses,
            weights: weights,
            v: 0.0,
            i_ext: 0.0,
            threshold: 10.0,
            t_rest: 0.0,
        }
    }

    fn run(&mut self, spike: &Vec<u8>, dt: f64) -> u8 {
        if self.t_rest > 0.0 {
            self.t_rest -= dt;
            if self.t_rest <= 0.0 {
                self.v = 0.0;
            }
            1
        } else {
            let mut i_rec = 0.0;
            for i in 0..self.synapses.len() {
                if spike[self.synapses[i]] == 1 {
                    i_rec += self.weights[i];
                }
            }
            let i_ext = self.i_ext * (1.0 + self.i_ext * random::<f64>());
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
}

fn main() {
    const N: usize = 100;
    let mut spike_train: Vec<Vec<u8>> = vec![Vec::new()];
    for _ in 0..N {
        if random::<f64>() < 0.5 {
            spike_train[0].push(1);
        } else {
            spike_train[0].push(0);
        }
    }
    let mut network: Network = Network::new(N);
    let mut t = 0.0;
    let dt = 0.1;
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    // y.push(0.0);

    while t <= 4000.0 { // 800.0
        if (t >= 1000.0) & (t <= 3000.0) { // 200, 600
            network.input(5.0);
        } else {
            network.input(4.0);
        }
        spike_train.push(network.run(&spike_train, dt));
        t += dt;
        // x.push(t);
        // y.push(network.neurons[0].v);
        println!("{}", t);
    }
    let time_all = spike_train.len();
    for i in 0..time_all {
        for j in 0..N {
            if spike_train[i][j] == 1 {
                x.push(i as f64 * dt);
                y.push(j as f64);
            }
        }
    }

    let mut y: Vec<f64> = Vec::with_capacity(spike_train.len());
    for i in 0..spike_train.len() {
        y.push(spike_train[i][0] as f64)
    }

    let mut fg = gnuplot::Figure::new();
    fg.axes2d()
        .lines(x.iter(), y.iter(), &[gnuplot::Color("blue")])
        //.lines(x.iter(), y.iter(), &[gnuplot::Color("blue")])
        .set_x_range(Fix(0.0), Fix(4000.0)); // 0.0, 800.0
    fg.echo_to_file("spike_train.plt"); // voltage.plt
}
