#![allow(dead_code, unreachable_code, unused_variables)]

// 07 - ASYNC/AWAIT
//
// Although Rust does provide operating system-level threads, these often cannot achieve
// the scalability and performance characteristics that are required for modern servers.
// To overcome the scaling limitations of traditional OS-level threads, Rust provides
// lightweight threads called green threads. These are sometimes called M:N threads,
// because they are multiplexed onto a smaller number of OS threads.
//
// Rust provides some baked in support for Future, and there are two primary schedulers
// that are used to run Futures. The first is the Tokio runtime, which is a scheduler
// that is optimized for asynchronous I/O. The second is the async-std runtime, which
// is a scheduler that is optimized for general purpose asynchronous programming.
//
// In this section, we will rely on Tokio.
mod futures {
    #![allow(unused_imports)]
    use std::pin::Pin;

    use async_trait::async_trait;
    use tokio::task::spawn;

    #[tokio::test]
    async fn basic_future() {
        let future = async {
            println!("Hello from the future!");
        };

        let result: Result<(), tokio::task::JoinError> =
            todo!("Spawn the future on the Tokio runtime and await its result");

        let message = match result {
            Ok(_) => "Ok",
            Err(_) => "Err",
        };

        assert_eq!(message, "Ok");
    }

    #[tokio::test]
    async fn basic_future_with_result() {
        let future = async {
            println!("Hello from the future!");
            42
        };

        let result: i32 =
            todo!("Spawn the future on the Tokio runtime, await its result, and unwrap it");

        assert_eq!(result, 42);
    }

    // #[tokio::test]
    #[test]
    fn async_trait_example() {
        #[derive(Debug, Clone)]
        struct User {
            id: i32,
            name: String,
        }

        // Using #[async_trait] and `async` on the method, turn this into an async trait
        // and ensure the test still passes.
        trait UserRepo {
            fn find_by_id(&self, id: i32) -> Option<User>;
        }

        struct TestUserRepo {
            user_map: std::collections::HashMap<i32, User>,
        }

        impl UserRepo for TestUserRepo {
            fn find_by_id(&self, id: i32) -> Option<User> {
                self.user_map.get(&id).map(|u| (*u).clone())
            }
        }

        let test_data = vec![
            User {
                id: 1,
                name: "Sherlock Holmes".to_string(),
            },
            User {
                id: 2,
                name: "John Watson".to_string(),
            },
            User {
                id: 3,
                name: "Mycroft Holmes".to_string(),
            },
        ];

        let test_repo = TestUserRepo {
            user_map: test_data.iter().map(|u| (u.id, u.clone())).collect(),
        };

        let user = test_repo.find_by_id(2).unwrap();

        assert_eq!(user.name, "John Watson");
    }
}
