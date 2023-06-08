#![allow(dead_code, unreachable_code, unused_variables)]

// 03 - TRAITS
//
// Traits provide a way to abstract over data types that, although they are in fact different,
// still possess similar structure. Traits are similar to interfaces in object-oriented
// programming, but more similar to type classes in functional programming, because Rust does
// not have objects or inheritance.
//
// In this section, you will learn how to create and implement traits, and the difference between
// dynamic trait objects and static trait bounds. You will also learn how to use the standard
// library’s common traits, such as the `ToString` trait that enables us to print our custom
// structs.

mod basics {
    #[test]
    fn simple_trait() {
        // Traits are declared using the `trait` keyword, followed by the trait name and a block
        // containing the function signatures that describe the behavior of the types that implement
        // the trait.
        trait Animal {
            // fn create(name: &'static str) -> Self;
            fn name(&self) -> &'static str;
            fn talk(&self) {
                println!("{} cannot talk", self.name());
            }
        }

        // To implement a trait for a type, we use the `impl` keyword followed by the trait name and
        // the block containing the function implementations.
        struct Human {
            name: &'static str,
        }

        // Implement the Animal trait for Human so the test can be made to pass:
        let sherlock = Human { name: "Sherlock" };

        assert_eq!(todo!("sherlock.name()") as &str, "Sherlock");
    }

    #[test]
    fn trait_bound() {
        trait Hopper {
            fn hop(&self);
        }

        struct Rabbit {
            name: &'static str,
        }

        // Implement the Hopper trait for Rabbit:

        // Add a trait bound to the hop function so the test can be made to pass:
        fn hop<T>(hopper: T) {
            todo!("hopper.hop()");
        }

        let rabbit = Rabbit { name: "Rabbit" };

        assert_eq!(hop(rabbit), ());
    }

    #[test]
    fn trait_bound_where() {
        trait Hopper {
            fn hop(&self);
        }

        impl Hopper for Rabbit {
            fn hop(&self) {
                println!("{} is hopping", self.name);
            }
        }

        struct Rabbit {
            name: &'static str,
        }

        // Use a where clause to add a trait bound to the hop function:
        fn hop<T>(hopper: T) {
            todo!("hopper.hop()");
        }

        let rabbit = Rabbit { name: "Rabbit" };

        assert_eq!(hop(rabbit), ());
    }

    #[test]
    fn trait_bound_multiple() {
        trait Hopper {
            fn hop(&self);
        }
        trait Swimmer {
            fn swim(&self);
        }

        impl Hopper for Duck {
            fn hop(&self) {
                println!("{} is hopping", self.name);
            }
        }
        impl Swimmer for Duck {
            fn swim(&self) {
                println!("{} is swimming", self.name);
            }
        }

        struct Duck {
            name: &'static str,
        }

        // Use a composite trait bound so you can make the animal hop and swim:
        fn hop_and_swim<T>(animal: T) {
            todo!("animal.hop(); animal.swim()");
        }

        let duck = Duck { name: "Duck" };

        assert_eq!(hop_and_swim(duck), ());
    }
}

mod standard_traits {
    #[test]
    fn to_string() {
        // The `ToString` trait enables us to convert a type to a string representation. It is
        // implemented for many primitive types, and also for some standard library types such as
        // `String` and `Vec<T>`.
        //
        // To convert a type to a string, we can use the `to_string` method, which is defined on any
        // type that implements the `ToString` trait.
        let s = 42.to_string();

        assert_eq!(todo!("s") as String, "42");
    }

    #[test]
    fn custom_to_string() {
        struct Person {
            name: String,
            age: i32,
        }

        // Implement ToString for Person so the test can be made to pass:
        assert_eq!(todo!("person.to_string()") as String, "John (42)");
    }

    #[test]
    fn from_str() {
        // The `FromStr` trait enables us to convert a string to a type. It is implemented for many
        // primitive types, and also for some standard library types such as `String` and `Vec<T>`.
        //
        // To convert a string to a type, we can use the `parse` method, which is defined on any type
        // that implements the `FromStr` trait.
        let s = "42".parse::<i32>().unwrap();

        assert_eq!(todo!("s") as i32, 42);
    }

    #[test]
    fn custom_from_str() {
        struct Person {
            name: String,
            age: i32,
        }

        // Implement FromStr for Person so the test can be made to pass:
        // let person = "John (42)".parse::<Person>().unwrap();

        assert_eq!(todo!("person.name") as &str, "John");
        assert_eq!(todo!("person.age") as i32, 42);
    }

    #[test]
    fn debug() {
        // The `Debug` trait enables us to print a type using the `{:?}` format specifier. It is
        // implemented for many primitive types, and also for some standard library types such as
        // `String` and `Vec<T>`.
        //
        // To print a type, we can use the `println!` macro, which accepts the `{:?}` format specifier
        // for types that implement the `Debug` trait.
        let s = format!("{:?}", 42);

        assert_eq!(todo!("s") as String, "42");
    }

    #[test]
    fn custom_debug() {
        struct Person {
            name: String,
            age: i32,
        }

        // Implement Debug for Person so the test can be made to pass:
        // (Note that ordinarily, you would derive Debug instead of implementing it manually.)
        let person = Person {
            name: "John".to_string(),
            age: 42,
        };

        let fmt: String = todo!(""); // format!("{:?}", person)

        assert_eq!(fmt, "Person { name: \"John\", age: 42 }");
    }
}

mod associated_types {
    #[test]
    fn pseudo_function() {
        // Traits may have associated types, which are types that are defined as part of the trait
        // definition. Associated types are similar to generic type parameters, except that they are
        // specified as part of the trait definition, which makes them deterministic functions of the
        // trait parameters. In this example, `Out` is a function of `In` for some type `Self`.
        trait Function<In> {
            type Out;

            fn call(&self, input: In) -> Self::Out;
        }

        struct Add {
            amount: i32,
        }

        // Implement the Function trait for Add so the test can be made to pass:
        // Note the return type of the call to `call`, which is tracked by the associated type `Out`.
        assert_eq!(todo!("Add {{ amount: 1 }}.call(2)") as i32, 3);
    }
}
