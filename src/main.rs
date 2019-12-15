mod network;
mod neuron;
use gnuplot::AxesCommon;
use gnuplot::*;
use network::Network;
use rand::distributions::{Bernoulli, Distribution};

fn init_spike(n: usize) -> Vec<u8> {
    let dist = Bernoulli::new(0.5).unwrap();
    (0..n)
        .map(|_| {
            if dist.sample(&mut rand::thread_rng()) {
                1
            } else {
                0
            }
        })
        .collect()
}

fn run(
    spike_train: &mut Vec<Vec<u8>>,
    mut network: Network,
    start: f64,
    end: f64,
    dt: f64,
    t1: f64,
    t2: f64,
) {
    let step = ((end - start) / dt) as usize;
    for s in 0..=step {
        let t = start + (s as f64) * dt;
        if t1 <= t && t <= t2 {
            network.input(5.0);
        } else {
            network.input(4.0);
        }
        spike_train.push(network.run(&spike_train, dt));
        println!("{}", t);
    }
}

fn output(spike_train: &[Vec<u8>], start: f64, end: f64, dt: f64) {
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();

    for (i, spike) in spike_train.iter().enumerate() {
        for (j, s) in spike.iter().enumerate() {
            if *s == 1 {
                x.push(i as f64 * dt);
                y.push(j as f64);
            }
        }
    }

    //let mut y: Vec<f64> = Vec::with_capacity(spike_train.len());
    //for spike in spike_train {
    //    y.push(spike[0] as f64)
    //}

    let mut fg = gnuplot::Figure::new();
    fg.axes2d()
        .points(x.iter(), y.iter(), &[gnuplot::Color("blue")])
        //.lines(x.iter(), y.iter(), &[gnuplot::Color("blue")])
        .set_x_range(Fix(start), Fix(end));
    fg.save_to_png("spike_train.png", 1024, 768).unwrap(); // voltage.plt
}

fn main() {
    const N: usize = 100;
    const START_TIME: f64 = 0.;
    const END_TIME: f64 = 4000.; //800.
    let mut spike_train: Vec<Vec<u8>> = vec![init_spike(N)];
    let dt = 0.1;
    run(
        &mut spike_train,
        Network::new(N),
        START_TIME,
        END_TIME,
        dt,
        (START_TIME * 3. + END_TIME) / 4.,
        (START_TIME + END_TIME * 3.) / 4.,
    );
    output(&spike_train, START_TIME, END_TIME, dt);
}
