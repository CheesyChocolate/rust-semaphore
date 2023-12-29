use std::sync::{Arc, Mutex, Condvar};
use std::thread;

#[derive(Debug)]
enum Direction {
        East,
        West,
}

// Semaphore to control the number of vehicles on the bridge
struct Semaphore {
        permits_east: Mutex<usize>,
        permits_west: Mutex<usize>,
        condvar: Condvar,
}

impl Semaphore {
        // initialize the semaphore with the number of permits for each direction
        fn new(permits_east: usize, permits_west: usize) -> Self {
                Semaphore {
                        permits_east: Mutex::new(permits_east),
                        permits_west: Mutex::new(permits_west),
                        condvar: Condvar::new(),
                }
        }

        fn acquire(&self, direction: &Direction) {
                match direction {
                        Direction::East => {
                                let mut permits = self.permits_east.lock().unwrap();
                                while *permits == 0 {
                                        permits = self.condvar.wait(permits).unwrap();
                                }
                                *permits -= 1;
                        }
                        Direction::West => {
                                let mut permits = self.permits_west.lock().unwrap();
                                while *permits == 0 {
                                        permits = self.condvar.wait(permits).unwrap();
                                }
                                *permits -= 1;
                        }
                }
        }

        fn release(&self, direction: &Direction) {
                match direction {
                        Direction::East => {
                                let mut permits = self.permits_east.lock().unwrap();
                                *permits += 1;
                                self.condvar.notify_one();
                        }
                        Direction::West => {
                                let mut permits = self.permits_west.lock().unwrap();
                                *permits += 1;
                                self.condvar.notify_one();
                        }
                }
        }
}

struct Bridge {
        semaphore: Arc<Semaphore>,
}

impl Bridge {
        // initialize the bridge with the number of permits for each direction
        fn new() -> Self {
                Bridge {
                        semaphore: Arc::new(Semaphore::new(2, 3)), // 2 permits for East, 3 permits for West
                }
        }

        fn cross_bridge(&self, id: usize, direction: Direction) {
                println!("Vehicle {} wants to cross the bridge from {:?} direction.", id, direction);

                self.semaphore.acquire(&direction);
                println!("Vehicle {} starts crossing the bridge from {:?} direction.", id, direction);

                // Simulate crossing time
                thread::sleep(std::time::Duration::from_secs(1));

                println!("Vehicle {} has crossed the bridge from {:?} direction.", id, direction);
                self.semaphore.release(&direction);
        }
}

// simulate traffic
fn simulate_traffic(bridge: Arc<Bridge>) {
        let mut handles = vec![];

        for i in 1..=13 {
                let bridge_clone = Arc::clone(&bridge);
                let direction = if i % 2 == 0 { Direction::West } else { Direction::East };
                let handle = thread::spawn(move || {
                        bridge_clone.cross_bridge(i, direction);
                });
                handles.push(handle);
        }

        for handle in handles {
                handle.join().unwrap();
        }
}

fn main() {
        // create a shared bridge between threads and simulate traffic
        let bridge = Arc::new(Bridge::new());
        simulate_traffic(Arc::clone(&bridge));
}
