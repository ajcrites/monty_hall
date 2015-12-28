extern crate rand;
extern crate num_cpus;

use std::io;
use rand::Rng;
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

fn make_selection() -> String {
    loop {
        let mut selection = String::new();
        io::stdin().read_line(&mut selection)
            .ok()
            .expect("failed to read line");

        match selection.trim() {
            "w" => return selection,
            "n" => return selection,
            _ => {
                println!("Please enter w or n");
                continue;
            },
        };
    }
}

fn main() {
    println!("Welcome to the Monty Hall Problem Simulator");
    println!("This will run 1_000_000 attempts of the Monty Hall problem");
    println!("and switch (or not) based on your selection");
    println!("s[w]itch or do[n]'t?");

    let selection = make_selection();

    let fail = Arc::new(Mutex::new(0));
    let suc = Arc::new(Mutex::new(0));

    let (tx, rx) = mpsc::channel();

    let cpus = num_cpus::get() - 1;
    let mut threads = vec![];

    for _ in (0..cpus) {
        let (selection, suc, fail, tx) = (selection.clone(), suc.clone(), fail.clone(), tx.clone());
        threads.push(thread::spawn(move || {
            for _ in (0..1_000_000 / cpus) {
                let mut doors = ['g', 'g', 'g'];
                doors[rand::thread_rng().gen_range(0, 3)] = 'c';

                let choice = rand::thread_rng().gen_range(0, 3);

                let mut monty;
                let mut final_choice;
                // Select the right door
                loop {
                    monty = rand::thread_rng().gen_range(0, 3);
                    // Monty reveals the other wrong door
                    if choice == monty {
                        continue;
                    }

                    // Monty does not reveal the car
                    if 'c' == doors[monty] {
                        continue;
                    }
                    break;
                }

                // we choose to switch!
                if "w" == selection.trim() {
                    loop {
                        final_choice = rand::thread_rng().gen_range(0, 3);

                        // Don't select the goat that monty revealed
                        if final_choice == monty {
                            continue;
                        }

                        // We force a switch
                        if final_choice == choice {
                            continue;
                        }
                        break;
                    }
                }
                else {
                    // We do not switch
                    final_choice = choice;
                }

                if 'c' == doors[final_choice] {
                    let mut suc = suc.lock().unwrap();
                    *suc += 1;
                }
                else {
                    let mut fail = fail.lock().unwrap();
                    *fail += 1;
                }
            }
            tx.send((suc, fail));
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }

    let (suc, fail) = rx.recv().unwrap();

    println!("suc: {:?}, fail: {:?}", suc, fail);
}
