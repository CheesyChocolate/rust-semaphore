use std::sync::{Arc, Mutex, Condvar};
use std::thread;

#[derive(Debug)]
enum Direction {
    East,
    West,
}

struct Semaphore {
    permits: Mutex<usize>,
    condvar: Condvar,
}

impl Semaphore {
    fn new(permits: usize) -> Self {
        Semaphore {
            permits: Mutex::new(permits),
            condvar: Condvar::new(),
        }
    }

    fn acquire(&self) {
        let mut permits = self.permits.lock().unwrap();
        while *permits == 0 {
            permits = self.condvar.wait(permits).unwrap();
        }
        *permits -= 1;
    }

    fn release(&self) {
        let mut permits = self.permits.lock().unwrap();
        *permits += 1;
        self.condvar.notify_one();
    }
}

struct Bridge {
    semaphore: Arc<Semaphore>,
}

impl Bridge {
    fn new() -> Self {
        Bridge {
            semaphore: Arc::new(Semaphore::new(1)),
        }
    }

    fn cross_bridge(&self, id: usize, direction: Direction) {
        println!("Vehicle {} wants to cross the bridge from {:?} direction.", id, direction);

        self.semaphore.acquire();
        println!("Vehicle {} starts crossing the bridge from {:?} direction.", id, direction);

        // Simulate crossing time
        thread::sleep(std::time::Duration::from_secs(1));

        println!("Vehicle {} has crossed the bridge from {:?} direction.", id, direction);
        self.semaphore.release();
    }
}

fn simulate_traffic(bridge: Arc<Bridge>) {
    let mut handles = vec![];

    for i in 1..=10 {
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
    let bridge = Arc::new(Bridge::new());
    simulate_traffic(Arc::clone(&bridge));
}
