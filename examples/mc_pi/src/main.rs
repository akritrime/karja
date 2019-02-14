use karja::DispatchQueue;
use std::sync::mpsc::{channel, Receiver};

const MAX: usize = 1_000_000_000;

fn monte_carlo(r: f64, reps: usize) -> usize {

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

fn parallel_mc() -> Receiver<usize> {
    let (tx, rx) = channel();
    let rt = DispatchQueue::new("Monte Carlo", 8);

    for _ in 0..4 {
        let tx = tx.clone();
        rt.dispatch(move || {
            let _ = tx.send(monte_carlo(1., MAX / 4));
        })
    }

    rx

}

fn main() {

    
    let total: usize = parallel_mc().iter().sum();
    
    println!("{}", (total as f64 / MAX  as f64) * 4.);
}
