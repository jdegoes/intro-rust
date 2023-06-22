#![allow(dead_code, unreachable_code, unused_variables, unused_imports)]

// 06 - CONCURRENCY
//
// Rust provides a number of tools for writing concurrent programs. The most
// basic of these tools is the thread. Threads are independent processes that
// can run in parallel and are scheduled by the operating system.
//
// Multithreading introduces particular challenges around data sharing. Rust
// provides a number of tools to help you write concurrent programs without
// suffering from many of the common pitfalls, such as races.

/// THREADS
///
/// As a multi-threaded programming language, Rust lets you create and use operating system
/// threads. Many applications will probably use asynchronous programming ("green threads") instead
/// of threads, but it is still useful to familiarize yourself with the lower-level primitives.
///
/// In this section, you will learn how to create threads and how to await their results.
mod threads {
    use std::thread::JoinHandle;

    #[test]
    fn spawn_join_thread() {
        let thread: JoinHandle<i32> = 
            std::thread::spawn(|| {
                println!("Hello from thread!");

                42
            });

        let result = thread.join().unwrap();

        assert_eq!(result, 42);
    }

    #[test]
    fn running_in_parallel() {
        let thread1 = std::thread::spawn(|| {
            println!("Hello from thread 1!");

            42
        });

        let thread2 = std::thread::spawn(|| {
            println!("Hello from thread 2!");

            43
        });

        let sum: i32 = thread1.join().unwrap() + thread2.join().unwrap();

        assert_eq!(sum, 85);
    }

    #[test]
    fn move_closure_in_spawn() {
        let user_ids = vec![1, 2, 3];

        let compute_min_thread = std::thread::spawn(move || {
            // Explain why this code doesn't compile and use the `move` keyword to fix it. Then see if
            // you can achieve the same result without using `move`.
            *user_ids.iter().min().unwrap()
        });

        let min = compute_min_thread.join().unwrap();

        assert_eq!(min, 1);
    }
}

/// SHARED DATA
///
/// In all multi-threaded programming languages, threads can share data with other threads. In Rust,
/// there are a variety of data types that can help you share immutable or mutable access to data
/// between different threads, all designed to achieve both correctness and performance.
mod sharing_data {
    use std::sync::{Arc, Mutex, RwLock};

    #[test]
    fn immutable_share() {
        // shared immutability and ownership
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct Person {
            name: String,
            age: i32,
        }

        let detectives = vec![
            Person {
                name: "Sherlock Holmes".to_string(),
                age: 64,
            },
            Person {
                name: "Hercule Poirot".to_string(),
                age: 54,
            },
        ];

        let shared_database = Arc::new(detectives);

        let thread1_database = shared_database.clone();
        let thread2_database = shared_database.clone();

        let thread1 = std::thread::spawn({
            move || {                
                // Get Sherlock from the shared database, and print and return his age:
                let age = (*thread1_database)[0].age;

                println!("Sherlock is {} years old", age);

                age
            }
        });

        let thread2 = std::thread::spawn({
            move || {   
                // Get Poirot from the shared database, print and return his age:
                let age = (*thread2_database)[1].age;

                println!("Poirot is {} years old", age);

                age
            }
        });

        let sum = thread1.join().unwrap() + thread2.join().unwrap();

        assert_eq!(sum, 118);
    }

    #[test]
    fn mutable_share() {
        // shared mutability and ownership
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct Person {
            name: String,
            age: i32,
            address: Address,
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        struct Address {
            street: String,
            city: String,
        }

        let sherlock = Arc::new(Mutex::new(Person {
            name: "Sherlock Holmes".to_string(),
            age: 64,
            address: Address {
                street: "221B Baker Street".to_string(),
                city: "London".to_string(),
            },
        }));

        let thread_sherlock = sherlock.clone();

        let thread = std::thread::spawn(move || {
            // Using `thread_sherlock`, obtain a lock on `Person`, and change the city to "New York".
            let mut locked = 
                thread_sherlock.lock().unwrap();

            (*locked).address.city = "New York".to_string();

            println!("Sherlock moved to New York!");
        });

        thread.join().unwrap();

        let locked = sherlock.lock().unwrap();

        // Try deleting Arc and Mutex and exploring the effect on compilation.
        assert_eq!(locked.address.city.clone(), "New York");
    }

    #[test]
    fn mutable_share_rw() {
        // shared mutability and ownership with a fine-grained read-write lock:
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct Person {
            name: String,
            age: i32,
        }
        let detectives = vec![
            Person {
                name: "Sherlock Holmes".to_string(),
                age: 64,
            },
            Person {
                name: "Hercule Poirot".to_string(),
                age: 54,
            },
        ];

        let shared_database = Arc::new(RwLock::new(detectives));

        let thread1 = std::thread::spawn({
            let shared_database = shared_database.clone();

            move || {
                // Using shared_database.read(), obtain a read lock, get sherlock,
                // and print and return his age:
                todo!("Get Sherlock from the shared database, and print and return his age") as i32
            }
        });

        let thread2 = std::thread::spawn({
            let shared_database = shared_database.clone();

            move || {
                // Using shared_database.write(), obtain a write lock, get poirot,
                // increment his age, print and return his age:
                todo!("Get Poirot from the shared database, increment his age, and print and return his age") as i32
            }
        });

        let sum = thread1.join().unwrap() + thread2.join().unwrap();

        assert_eq!(sum, 119);
    }
}
