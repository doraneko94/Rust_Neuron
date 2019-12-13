mod neuron;
mod network;
use gnuplot::AxesCommon;
use gnuplot::*;
use rand::random;
use network::Network;

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
