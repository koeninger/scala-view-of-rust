use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

fn main() {
    let (input_tx, input_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();

    let counter = thread::spawn(move || { count_evens_vs_odds(input_rx, result_tx) });
    let reporter = thread::spawn(move || { report(result_rx) });
    
    for x in (1..100) { input_tx.send(x).unwrap() }

    counter.join().unwrap();
    reporter.join().unwrap();
}

fn report(rx: Receiver<&Vec<i32>>) {
    loop {
        let state = rx.recv().unwrap();
        println!("I saw {} {}", state[0], state[1]);
    }
}

fn count_evens_vs_odds(rx: Receiver<i32>, tx: Sender<&Vec<i32>>) {
    let mut state = vec![0, 0];
    loop {
        let x = rx.recv().unwrap();
        if x % 2 == 0 {
            state[0] += 1;
        } else {
            state[1] += 1;
        }
        if (state[0] + state[1]) % 10 == 0 {
            // COMPILE error: state does not live long enough
            //tx.send(&state).unwrap();
        }
    }
}
