extern crate rand;
extern crate num_cpus;

use std::io;
use rand::Rng;
use std::thread;
use std::sync::{Arc, RwLock};
use std::fmt;

#[derive(Clone)]
pub struct ConcurrentCounter(Arc<RwLock<usize>>);

impl ConcurrentCounter {
    pub fn new(val: usize) -> Self {
        ConcurrentCounter(Arc::new(RwLock::new(val)))
    }

    pub fn inc(&self) {
        let mut counter = self.0.write().unwrap();
        *counter = *counter + 1;
    }

}

impl fmt::Display for ConcurrentCounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let counter = self.0.read().unwrap();
        write!(f, "{}", *counter)
    }
}

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

    let fail = ConcurrentCounter::new(0);
    let suc = ConcurrentCounter::new(0);

    let cpus = num_cpus::get() - 1;
    let mut threads = vec![];

    for _ in (0..cpus) {
        let (selection, suc, fail) = (selection.clone(), suc.clone(), fail.clone());
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
                    suc.inc();
                }
                else {
                    fail.inc();
                }
            }
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }

    println!("suc: {}, fail: {}", suc, fail);
}
