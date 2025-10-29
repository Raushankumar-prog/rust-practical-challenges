use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

struct SharedData {
    value: AtomicUsize,
}

impl SharedData {
    fn new(v: usize) -> Self {
        SharedData {
            value: AtomicUsize::new(v),
        }
    }

    fn try_update(&self, new_val: usize) {
        loop {
            
            let current = self.value.load(Ordering::Relaxed);

            
            match self.value.compare_exchange(
                current,
                new_val,
                Ordering::SeqCst,  
                Ordering::Relaxed, 
            ) {
                Ok(_) => {
                    println!("Updated successfully from {current} -> {new_val}");
                    break;
                }
                Err(actual) => {
                    println!("Failed: expected {current}, found {actual}. Retrying...");
                    continue; 
                }
            }
        }
    }
}

fn main() {
    let shared = Arc::new(SharedData::new(0));

    let mut handles = vec![];

   
    for i in 0..5 {
        let s = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            for _ in 0..3 {
                let new_value = i + 1; 
                s.try_update(new_value);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Final value: {}", shared.value.load(Ordering::SeqCst));
}
