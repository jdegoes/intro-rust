#![allow(dead_code, unreachable_code, unused_variables)]

#[allow(unused_imports)]
use std::{mem::swap, pin::Pin};

// 02 - MEMORY
//
// Memory management is fundamental to Rust. It is a core feature of the language, and the source
// of both many of its advantages over systems-level languages like C and C++, and many of its
// disadvantages compared to higher-level languages like Python and JavaScript.
//
// In this section, you will start by exploring the differences between stack and heap, and
// gradually learn about Rust's ownership model, which is the key to understanding how Rust
// manages memory. You will learn about borrowing and lifetimes, and how they are used to ensure
// memory safety without garbage collection.
//
// Although Rust memory management cannot be mastered in any single day workshop, you will learn
// enough to be able to write safe Rust code, and to understand the error messages that the
// compiler gives you when you don't.

mod stack {

    #[test]
    fn stack_size() {
        fn grow_stack(n: i32) -> i32 {
            if n <= 0 {
                return 0;
            } else {
                let x = std::mem::size_of::<i32>() as i32;
                let y = grow_stack(n - 1);

                x + y
            }
        }

        assert_eq!(grow_stack(10), todo!("What is the size of the stack?"));
    }

    #[test]
    fn copy_struct_using_stack() {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        fn transform_point(p: Point) -> Point {
            Point {
                x: p.x + 1,
                y: p.y + 1,
            }
        }

        let p1 = Point { x: 1, y: 2 };
        let p2 = transform_point(p1);

        assert_eq!(
            0,
            todo!("How much heap memory is allocated by the above invocation of transform_point()")
                as i32
        );
    }
}

mod heap {
    #[test]
    fn heap_size() {
        fn grow_heap(n: i32) -> i32 {
            if n <= 0 {
                return 0;
            } else {
                let x = Box::new(std::mem::size_of::<i32>() as i32);
                let y = grow_heap(n - 1);

                *x + y
            }
        }

        assert_eq!(grow_heap(10), todo!("What is the size of the heap?"));
    }

    #[test]
    fn copy_struct_using_heap() {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        fn transform_point(p: Point) -> Box<Point> {
            Box::new(Point {
                x: p.x + 1,
                y: p.y + 1,
            })
        }

        let p1 = Point { x: 1, y: 2 };
        let p2 = transform_point(p1);

        assert_eq!(
            0,
            todo!("How much heap memory is allocated by the above invocation of transform_point()")
                as i32
        );
    }

    #[test]
    fn cannot_return_unsized() {
        trait PersonLike {
            fn name(&self) -> String;
            fn age(&self) -> i32;
        }

        struct Person {
            name: String,
            age: i32,
        }

        impl PersonLike for Person {
            fn name(&self) -> String {
                self.name.clone()
            }

            fn age(&self) -> i32 {
                self.age
            }
        }

        // In Rust, all return values must be Sized, and the size of a trait is not known at
        // compile time. Uncomment the following code to see the error message, and then fix the
        // problem by changing the return type.
        // fn create_person(name: String, age: i32) -> dyn PersonLike {
        //     Person { name: name, age }
        // }

        // let sherlock = create_person("Sherlock Holmes".to_owned(), 64);

        assert_eq!(todo!("sherlock.name()") as String, "Sherlock Holmes");
    }
}

mod mutable_variables {
    #[test]
    fn mutable_variable_modify() {
        #[allow(unused_assignments)]
        let mut x = 1;

        x = 2;

        assert_eq!(x, todo!("What is the value of x?") as i32);
    }

    #[test]
    fn mutable_variable_size() {
        struct Person {
            name: String,
            age: i32,
        }
        fn increment_age(person: &mut Person) -> () {
            person.age += 1;
        }

        let person = Person {
            name: "Sherlock Holmes".to_string(),
            age: 64,
        };

        // Uncomment the following line to see what happens, and then fix the problem.
        // Hint: You will have to create a mutable local variable.
        todo!("increment_age(&mut person)");

        assert_eq!(65, person.age);
    }
}

mod safe_pointers {

    #[test]
    fn shared_pointer_read() {
        let x = 1;

        let pointer_x: &i32 = &x;

        let value = *pointer_x;

        assert_eq!(value, todo!("What is the value of x?"));
    }

    #[test]
    fn shared_pointer_size() {
        struct Person {
            name: String,
            age: i32,
        }

        let sherlock = Person {
            name: "Sherlock Holmes".to_string(),
            age: 64,
        };

        let sherlock_pointer = &sherlock;

        assert_eq!(
            std::mem::size_of::<&Person>(),
            todo!("What is the size of a pointer?")
        );
    }

    #[test]
    fn unique_pointer_read() {
        let mut x = 2;

        let pointer_x: &mut i32 = &mut x;

        let value = *pointer_x;

        assert_eq!(value, todo!("What is the value of x?"));
    }

    #[test]
    fn unique_pointer_size() {
        struct Person {
            name: String,
            age: i32,
        }

        let mut sherlock = Person {
            name: "Sherlock Holmes".to_string(),
            age: 64,
        };

        let sherlock_pointer = &mut sherlock;

        sherlock_pointer.age = 65;

        assert_eq!(
            std::mem::size_of::<&mut Person>(),
            todo!("What is the size of a mutable pointer?")
        );
    }

    #[test]
    fn unique_pointer_modify_via_manual_deref() {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        fn transform_point(p: &mut Point) -> () {
            todo!("Modify p to make the test pass using (*p).")
        }

        let mut p1 = Point { x: 1, y: 2 };

        transform_point(&mut p1);

        assert_eq!(p1, Point { x: 4, y: 2 });
    }

    #[test]
    fn unique_pointer_modify_via_auto_deref() {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        fn transform_point(p: &mut Point) -> () {
            todo!("Modify p to make the test pass using p.")
        }

        let mut p1 = Point { x: 1, y: 2 };

        transform_point(&mut p1);

        assert_eq!(p1, Point { x: 4, y: 2 });
    }

    fn shared_pointer_to_shared_pointer() {
        let x = 1;
        let y = &x;
        let z = &y;

        assert_eq!(**z, todo!("What is the value of z?") as i32);
    }

    #[test]
    fn unique_pointer_to_pointer_via_manual_deref() {
        let mut x = 1;
        let mut y = &mut x;
        let z = &mut y;

        todo!("Modify z to make the test pass using (**z).");

        assert_eq!(x, 4);
    }

    #[test]
    fn unique_pointer_to_pointer_via_auto_deref() {
        struct Person {
            name: String,
            age: i32,
        }

        let mut detective = Person {
            name: "Sherlock Holmes".to_string(),
            age: 64,
        };
        let mut detective_ptr = &mut detective;
        let detective_ptr_ptr = &mut detective_ptr;

        todo!("Modify z to make the test pass using detective_ptr_ptr.");

        assert_eq!(detective.age, 65);
    }
}

mod ownership {
    #[test]
    fn move_semantics() {
        #[derive(Debug, PartialEq, Clone)]
        struct Point {
            x: i32,
            y: i32,
        }

        fn transform_point(p: Point) -> Point {
            Point {
                x: p.x + 1,
                y: p.y + 1,
            }
        }

        let p1 = Point { x: 1, y: 2 };
        let p2 = transform_point(p1);

        // Uncomment the following line to see what happens, and then fix the problem
        // that arises by cloning `p1` at the right place.
        assert_eq!(todo!("p1") as Point, p2);
    }

    #[test]
    fn drop_semantics() {
        #[derive(Debug, PartialEq, Eq, Clone)]
        struct Person {
            name: &'static str,
            age: i32,
        }

        impl Drop for Person {
            fn drop(&mut self) {
                println!("Dropping {:?}", self);
            }
        }

        fn consume(p: Person) -> () {
            println!("Consuming {:?}", p);
        }

        let p1 = Person {
            name: "Sherlock Holmes",
            age: 64,
        };

        let p2 = p1.clone();

        consume(p1);

        println!("Is p1 still alive?");

        // Try comparing `p1` to `p2`, note what happens, and fix the problem.
        assert_eq!(todo!("p1") as Person, p2);
    }

    #[test]
    fn copied_shared_pointer_semantics() {
        #[derive(Debug, PartialEq, Clone)]
        struct Point {
            x: i32,
            y: i32,
        }

        let point = Point { x: 1, y: 2 };

        let point_ptr = &point;
        let copied_point_ptr = point_ptr;

        assert_eq!(1, todo!("point_ptr.x") as i32);
    }

    #[test]
    fn moved_unique_pointer_semantics() {
        #[derive(Debug, PartialEq, Clone)]
        struct Point {
            x: i32,
            y: i32,
        }

        let mut point = Point { x: 1, y: 2 };

        let point_ptr = &mut point;
        let moved_point_ptr = point_ptr;

        // Uncomment the following line to see what happens, and then fix the problem.
        // todo!("point_ptr.x = 3;");

        assert_eq!(3, point.x);
    }

    #[test]
    fn no_overlapping_unique_pointers() {
        struct Person {
            name: String,
            age: i32,
        }

        fn modify_age_and_name(name: &mut String, person: &mut Person) -> () {
            name.push_str(" Senior");
            person.age += 1;
        }

        #[allow(unused_mut)]
        let mut sherlock = Person {
            name: "Sherlock Holmes".to_string(),
            age: 64,
        };

        // Try the following code, identify the problem, and fix it to make the test pass.
        todo!("modify_age_and_name(&mut sherlock.name, &mut sherlock)");

        assert_eq!(sherlock.name, "Sherlock Holmes Senior");
        assert_eq!(sherlock.age, 65);
    }

    #[test]
    fn pin_semantics() {
        #[derive(Debug, PartialEq, Clone)]
        struct Point {
            x: i32,
            y: i32,
        }

        let mut point1 = Point { x: 1, y: 2 };
        let mut point2 = Point { x: 2, y: 1 };

        let pointer1 = &mut point1;
        let pointer2 = &mut point2;

        // Make this line of code impossible by pinning one or both of the pointers.
        core::mem::swap(pointer1, pointer2);

        assert_eq!(*pointer1, Point { x: 1, y: 2 });
        assert_eq!(*pointer2, Point { x: 2, y: 1 });
    }
}

mod closures {
    #[test]
    fn closure_move() {
        struct Person {
            name: String,
            age: i32,
            address: Address,
        }

        struct Address {
            street: String,
            city: String,
        }

        let sherlock = Person {
            name: "Sherlock Holmes".to_string(),
            age: 64,
            address: Address {
                street: "221B Baker Street".to_string(),
                city: "London".to_string(),
            },
        };

        let move_sherlock = || {
            let mut sherlock2 = sherlock;

            sherlock2.address.city = "New York".to_string();

            println!("Sherlock moved to New York!");
        };

        move_sherlock();

        // Explain why the following code does not and cannot compile. Then, fix the problem.
        assert_eq!(todo!("sherlock.age") as i32, 64);
    }

    #[test]
    fn closure_borrow() {
        struct Person {
            name: String,
            age: i32,
            address: Address,
        }

        struct Address {
            street: String,
            city: String,
        }

        let mut sherlock = Person {
            name: "Sherlock Holmes".to_string(),
            age: 64,
            address: Address {
                street: "221B Baker Street".to_string(),
                city: "London".to_string(),
            },
        };

        let borrow_sherlock = &mut sherlock;

        #[allow(unused_mut)]
        let mut move_sherlock = || {
            borrow_sherlock.address.city = "New York".to_string();

            println!("Sherlock moved to New York!");
        };

        let new_home = sherlock.address.city.clone();

        // Uncomment the following line to see what happens, and then fix the problem
        // by moving this line somewhere else.
        // move_sherlock();

        // Explain why the following code does not and cannot compile. Then, fix the problem.
        assert_eq!(new_home, "New York".to_string());
    }
}

mod threads {
    #[allow(unused_imports)]
    use std::sync::{Arc, Mutex};

    #[test]
    fn transferring_ownership() {
        struct Person {
            name: String,
            age: i32,
            address: Address,
        }

        struct Address {
            street: String,
            city: String,
        }

        let sherlock = Person {
            name: "Sherlock Holmes".to_string(),
            age: 64,
            address: Address {
                street: "221B Baker Street".to_string(),
                city: "London".to_string(),
            },
        };

        let _ = std::thread::spawn(|| {
            let mut sherlock2 = sherlock;

            sherlock2.address.city = "New York".to_string();

            println!("Sherlock moved to New York!");
        });

        // Explain why the following code does not and cannot compile. Then, fix the problem.
        assert_eq!(todo!("sherlock.age") as i32, 64);
    }

    #[test]
    fn move_closure() {
        struct Person {
            name: String,
            age: i32,
            address: Address,
        }

        struct Address {
            street: String,
            city: String,
        }

        let mut sherlock = Person {
            name: "Sherlock Holmes".to_string(),
            age: 64,
            address: Address {
                street: "221B Baker Street".to_string(),
                city: "London".to_string(),
            },
        };

        let _ = std::thread::spawn(move || {
            sherlock.address.city = "New York".to_string();

            println!("Sherlock moved to New York!");
        });

        // A value that is moved to a closure cannot be used after the move.
        // Uncomment the following line to see what happens, and then fix the problem.
        assert_eq!(todo!("sherlock.address.city") as String, "London");
    }

    #[test]
    fn closure_share() {
        struct Person {
            name: String,
            age: i32,
            address: Address,
        }

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
            let mut locked = thread_sherlock.lock().unwrap();

            locked.address.city = "New York".to_string();

            println!("Sherlock moved to New York!");
        });

        thread.join().unwrap();

        let locked = sherlock.lock().unwrap();

        // Try deleting Arc and Mutex and exploring the effect on compilation.
        assert_eq!(todo!("locked.address.city") as String, "New York");
    }
}
