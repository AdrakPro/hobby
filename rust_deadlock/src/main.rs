// https://www.snoyman.com/blog/2024/01/best-worst-deadlock-rust/
// Actively learning more about Rust and multithreading.

use std::sync::Arc;

use parking_lot::{RwLock, RwLockReadGuard};

// The Clone trait provides a way to create a deep copy of an object,
// allowing you to duplicate the entire structure, including nested data.
#[derive(Clone)]
struct Person {
    inner: Arc<RwLock<PersonInner>>,
}

struct PersonInner {
    name: String,
    age: u32,
}

struct PersonReadGuard<'a> {
    guard: RwLockReadGuard<'a, PersonInner>,
}

impl Person {
    fn read(&self) -> PersonReadGuard {
        PersonReadGuard {
            guard: self.inner.read(),
        }
    }

    fn birthday(&self) -> u32 {
        let mut guard = self.inner.write();
        guard.age += 1;
        guard.age
    }
}

impl PersonReadGuard<'_> {
    fn can_access(&self) -> bool {
        const MIN_AGE: u32 = 18;

        self.guard.age >= MIN_AGE
    }

    fn get_name(&self) -> &String {
        &self.guard.name
    }
}

// impl Person {
//     fn can_access(&self) -> bool {
//         const MIN_AGE: u32 = 18;
//
//         // Call RwLock's read:
//         // 1. Waits until it's allowed to take a read guard.
//         //    For example, if there's an existing write guard active, it will block until that write guard finishes.
//         // 2. Increments a counter somewhere indicating that there's a new active read guard.
//         // 3. Constructs the RwLockReadGuard value.
//         // 4. When that value gets dropped, its Drop impl will decrement that counter.
//         // Prevent deadlock by occupying only one read lock
//         // self.inner.read().age >= MIN_AGE
//         self.age >= MIN_AGE
//     }
//
//     fn birthday(&self) -> u32 {
//         let mut guard = self.inner.write();
//         guard.age += 1;
//         // Return the new age
//         guard.age
//     }
// }

fn main() {
    let marek = Person {
        inner: Arc::new(RwLock::new(PersonInner {
            // Create an owned version of a value, typically converting from a borrowed type to an owned type.
            name: "Marek".to_owned(),
            age: 15,
        })),
    };

    // Experimental
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(2));
        for deadlock in parking_lot::deadlock::check_deadlock() {
            for deadlock in deadlock {
                println!(
                    "Found a deadlock! {}:\n{:?}",
                    deadlock.thread_id(),
                    deadlock.backtrace()
                );
            }
        }
    });

    // Keep a single shared piece of data in memory for multiple threads
    let marek_clone = marek.clone();
    // move -> Force the closure to take ownership of the variables it captures, rather than borrowing them.
    std::thread::spawn(move || loop {
        // 1. Access check thread takes a read lock (for reading the name)
        // 2. Birthday thread tries to take a write lock, but it can't because there's already a read lock. It stands in line waiting its turn.
        // 3. Access check thread tries to take a read lock (for checking the age). It sees that there's a write lock waiting in line,
        // and to avoid starving it, stands in line behind the birthday thread
        // 4. The access check thread is blocked until the birthday thread releases its lock.
        // The birthday thread is blocked until the access check thread releases its first lock. Neither thread can make progress. Deadlock!
        let guard = marek_clone.read();
        println!("Downloading a cute loading image, please wait...");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!(
            "Does the {} have access? {}",
            guard.get_name(),
            // marek_clone.can_access()
            guard.can_access()
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
    });

    for _ in 0..10 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let new_age = marek.birthday();

        println!("Happy birthday! Person is now {new_age} years old!")
    }
}
