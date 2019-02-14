use karja::DispatchQueue;
use std::sync::mpsc::channel;

const MAX: usize = 1_000_000_000;

fn monte_carlo(r: f64, reps: usize) -> usize {
    // let mut count = 0;

    (0..reps).filter_map(|_| {
        let x: f64 = rand::random();
        let y: f64 = rand::random();

        let rx = x * r;
        let ry = y * r;

        if (rx * rx + ry * ry).sqrt() < r {
            Some(1)
        } else {
            None
        }
    })
    .count()
}

fn main() {
    let rt = DispatchQueue::new("Monte Carlo", 8);

    let (tx, rx) = channel();

    for _ in 0..4 {
        let tx = tx.clone();
        rt.dispatch(move || {
            let _ = tx.send(monte_carlo(1., MAX / 4));
        })
    }
    drop(tx);

    let total = rx.iter()
        .fold(0, |acc, c| acc + c);
    
    println!("{}", (total as f64 / MAX  as f64) * 4.);
}

// use rayon::prelude::*;

// fn parallel_monte_carlo_pi(points: usize) -> f64 {
//     let within_circle = (0..points)
//         .into_par_iter()
//         .filter_map(|_| {
//             let x = rand::random::<f64>() * 2. - 1.;
//             let y = rand::random::<f64>() * 2. - 1.;
//             if x * x + y * y <= 1f64 { Some(1) } else { None }
//         })
//         .count();
//     4f64 * within_circle as f64 / points as f64
// }

// fn main() {
//     println!("{}", parallel_monte_carlo_pi(MAX))
// }