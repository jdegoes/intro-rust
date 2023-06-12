#![allow(dead_code, unreachable_code, unused_variables, unused_imports)]

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

/// STACK
///
/// The stack is a region of memory allocated to each thread. It is used to store local variables,
/// function arguments, and return values. The stack is a LIFO (last in, first out) data structure.
/// Storing values on the stack is very fast, because it only requires incrementing a pointer as
/// the stack grows, and then decrementing the pointer to free the memory as the stack shrinks.
/// The stack grows with each function call, and shrinks with each function return.
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

        assert_eq!(
            grow_stack(10),
            todo!("What is the size of the stack?") as i32
        );
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

/// HEAP
///
/// The heap is a region of memory that is managed by the operating system. It is used to allocate
/// memory for objects whose size is not known at compile time, or whose lifetime is not known at
/// compile time. The heap is also used to allocate memory for objects that are very large, or
/// that must live for a long time.
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

        assert_eq!(grow_heap(10), todo!("What is the size of the heap?") as i32);
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

mod raii {
    /// RAII stands for "Resource Acquisition Is Initialization". It is a pattern that is used to
    /// ensure that resources are released when they go out of scope.
    ///
    /// In Rust, variables own resources. When objects go out of scope, their destructor is called,
    /// and the resources they own are released. This is the basis of Rust's memory safety.
    ///
    /// Rust provides a middle-ground between manual memory management and garbage collection.
    #[test]
    fn automatic_freeing_of_memory() {
        #[derive(Debug, PartialEq, Eq)]
        struct Person<'a> {
            name: &'static str,
            age: i32,
            dropped: &'a mut bool,
        }

        let dropped = false;

        impl Drop for Person<'_> {
            fn drop(&mut self) {
                (*self.dropped) = true;

                println!("Dropping {:?}", self);
            }
        }

        let mut dropped = false;

        let detective = Person {
            name: "Sherlock Holmes",
            age: 64,
            dropped: &mut dropped,
        };

        fn relocate(p: Person) -> () {
            println!("Relocating {:?} to another country", p);
        }

        relocate(detective);

        println!("Is detective still alive?");

        // Fix the test and try to understand why your change makes it pass.
        assert_eq!(dropped, false);
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

/// POINTERS (REFERENCES)
///
/// Rust provides two kinds of pointers: shared pointers and unique pointers. Shared pointers
/// allow multiple, read-only references to the same value. Unique pointers allow only a single,
/// mutable reference to a value. Also called references, pointers are a basic building block in
/// all programming languages.
///
/// In this section, you will learn about Rust's pointer types.
mod safe_pointers {

    #[test]
    fn shared_pointer_read() {
        let x = 1;

        let pointer_x: &i32 = &x;

        let value = *pointer_x;

        assert_eq!(value, todo!("What is the value of x?") as i32);
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
            todo!("What is the size of a pointer?") as usize
        );
    }

    #[test]
    fn unique_pointer_read() {
        let mut x = 2;

        let pointer_x: &mut i32 = &mut x;

        let value = *pointer_x;

        assert_eq!(value, todo!("What is the value of x?") as i32);
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
            todo!("What is the size of a mutable pointer?") as usize
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

/// OWNERSHIP
///
/// In Rust, every value has a single owner. When the owner goes out of scope, the value is dropped.
/// This is the basis of Rust's memory safety, and the feature that makes it possible to use Rust
/// without having to manually allocate and free memory.
///
/// In this section, you will learn about Rust's ownership model, including borrowing via pointers.
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

/// CLOSURES
///
/// Closures may capture over variables from their environment. This is a powerful feature, but
/// requires that you be aware of capture semantics on ownership and borrowing.
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

/// WRAPPER TYPES
///
/// Rust provides a number of wrapper types that are used to provide additional memory-related
/// functionality beyond what Rust's built-in pointer types provide. These types are used to
/// provide interior mutability, reference counting, and other features.
///
/// In this section, we will look at wrapper types that are useful in the context of a single
/// thread. In a later section, we will look at wrapper types that are useful in the context of
/// multiple threads.
mod wrapper_types {
    use std::mem::size_of;

    /// Box<A> is a smart pointer that allocates its contents on the heap.
    #[test]
    fn box_wrapper() {
        #[derive(Debug, PartialEq)]
        struct Person {
            name: String,
            age: i32,
        }

        let sherlock = Person {
            name: "Sherlock Holmes".to_string(),
            age: 64,
        };

        let sherlock_box = Box::new(sherlock);

        assert_eq!(
            size_of::<Box<Person>>(),
            todo!("What is the size of a Box?") as usize
        );
    }

    /// Rc<A> is a reference-counted type that allows sharing of immutable values.
    #[test]
    fn rc_wrapper() {
        use std::rc::Rc;

        #[derive(Debug, PartialEq)]
        struct Person {
            name: String,
            age: i32,
        }

        let sherlock = Person {
            name: "Sherlock Holmes".to_string(),
            age: 64,
        };

        let sherlock_rc = todo!("Create a Rc to Sherlock");

        let pointer1 = todo!("Clone a Rc to Sherlock");
        let pointer2 = todo!("Clone a Rc to Sherlock");

        assert_eq!(todo!("pointer1.age") as i32, todo!("pointer2.age") as i32);
    }

    /// Cell<A> is a type that allows zero-cost interior mutability for Copy types.
    #[test]
    fn cell_wrapper() {
        use std::cell::Cell;

        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        struct Person {
            name: &'static str,
            age: i32,
        }

        let sherlock = Person {
            name: "Sherlock Holmes",
            age: 64,
        };

        let sherlock_cell = Cell::new(sherlock);

        let pointer1 = &sherlock_cell;
        let pointer2 = &sherlock_cell;

        // Use the `replace` method to change the age of Sherlock to 65, through `pointer1`:
        let original_sherlock: Person = todo!("Create a new version of Sherlock whose page is 65");

        // Use the `replace` method to change the age of Sherlock to 66, through `pointer1`:
        let older_sherlock: Person = todo!("Create a new version of Sherlock whose page is 66");

        assert_eq!(
            original_sherlock,
            Person {
                name: "Sherlock Holmes",
                age: 64
            }
        );
        assert_eq!(
            older_sherlock,
            Person {
                name: "Sherlock Holmes",
                age: 65
            }
        );
        assert_eq!(
            sherlock_cell.get(),
            Person {
                name: "Sherlock Holmes",
                age: 66
            }
        );
    }

    // RefCell<A> is a type that allows interior mutability for non-Copy types, at higher cost.
    #[test]
    fn ref_cell_wrapper() {
        use std::cell::RefCell;

        #[derive(Clone, Eq, PartialEq, Debug)]
        struct Person {
            name: String,
            age: i32,
        }

        let sherlock = Person {
            name: "Sherlock Holmes".to_string(),
            age: 64,
        };

        let sherlock_ref_cell = RefCell::new(sherlock);

        let pointer1 = &sherlock_ref_cell;
        let pointer2 = &sherlock_ref_cell;

        // Use the `borrow_mut` method to change the age of Sherlock to 65, through `pointer1`:
        todo!("Change Sherlock's age to 65");

        // Use the `borrow_mut` method to change the age of Sherlock to 66, through `pointer2`:
        todo!("Change Sherlock's age to 66");

        assert_eq!(sherlock_ref_cell.borrow().age, 66);
    }

    // OnceCell<A> is a type that allows single assignment of non-Copy types, at zero cost.
    #[test]
    fn once_cell_wrapper() {
        use std::cell::RefCell;
        use std::collections::HashMap;

        use once_cell::unsync::OnceCell;

        #[derive(Clone, Eq, PartialEq, Debug)]
        struct Person {
            name: String,
            age: i32,
        }

        let sherlock_once_cell: OnceCell<Person> = OnceCell::new();

        let pointer1: &OnceCell<Person> = &sherlock_once_cell;
        let pointer2: &OnceCell<Person> = &sherlock_once_cell;

        // Use the `get_or_init` method to set the value of Sherlock to 64, through `pointer1`:
        todo!("Create a Sherlock whose age is 64");

        // Use the `get_or_init` method to set the value of Sherlock to 65, through `pointer2`:
        todo!("Create a Sherlock whose age is 65");

        assert_eq!(sherlock_once_cell.get().unwrap().age, 64);
    }
}

/// LIFETIMES
///
/// Lifetimes are a way to ensure that pointers (references) are valid for as long as they are
/// used. Rust uses the concept of lifetimes even when you don't explicitly see them. However,
/// there are many occassions when you need to explicitly specify lifetimes, and this section
/// will teach you how to do that.
mod lifetimes {
    #[test]
    fn lifetime_elision() {
        fn identity_explicit<'a>(x: &'a i32) -> &'a i32 {
            x
        }

        fn identity_implicit(x: &i32) -> &i32 {
            todo!("Write the same function as identity_explicit, but without explicit lifetimes")
        }

        let x = 1;

        assert_eq!(identity_explicit(&x), identity_implicit(&x));
    }

    #[test]
    fn lifetime_max() {
        todo!("Try to rewrite this function to not use explicit lifetimes");
        fn max_explicit<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
            if x > y {
                x
            } else {
                y
            }
        }

        let x = 1;
        let y = 2;

        assert_eq!(max_explicit(&x, &y), &y);
    }

    #[test]
    fn struct_lifetime_simple() {
        /// Refactor this from using 'static lifetime for the name to using a lifetime parameter,
        /// called `'a`, and ensure the code still compiles and passes.
        #[derive(Debug, PartialEq)]
        struct Person {
            name: &'static str,
            age: i32,
        }

        let sherlock = Person {
            name: "Sherlock Holmes",
            age: 64,
        };

        assert_eq!(sherlock.name, "Sherlock Holmes");
    }

    #[test]
    fn struct_lifetime_complex() {
        enum Tree<A> {
            Leaf(A),
            Branch(Box<Tree<A>>, Box<Tree<A>>),
        }

        struct TreeIterator<'a, A> {
            current: Option<&'a Tree<A>>,
            todo: Vec<&'a Tree<A>>,
        }

        fn advance<'a>(iterator: &mut TreeIterator<'a, i32>) -> Option<&'a i32> {
            todo!("Implement advance for TreeIterator")
        }

        let tree = Tree::Branch(
            Box::new(Tree::Leaf(1)),
            Box::new(Tree::Branch(
                Box::new(Tree::Leaf(2)),
                Box::new(Tree::Leaf(3)),
            )),
        );

        let mut iterator = TreeIterator {
            current: Some(&tree),
            todo: Vec::new(),
        };

        assert_eq!(advance(&mut iterator), Some(&1));
        assert_eq!(advance(&mut iterator), Some(&2));
        assert_eq!(advance(&mut iterator), Some(&3));
        assert_eq!(advance(&mut iterator), None);
    }
}
