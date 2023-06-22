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

        impl Animal for Human {
            fn name(&self) -> &'static str {
                self.name
            }

            fn talk(&self) {
                println!("{} says hello", self.name());
            }
        }

        // Implement the Animal trait for Human so the test can be made to pass:
        let sherlock = Human { name: "Sherlock" };

        assert_eq!(sherlock.name() as &str, "Sherlock");
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
        impl Hopper for Rabbit {
            fn hop(&self) {
                println!("{} is hopping", self.name);
            }
        }

        // Add a trait bound to the hop function so the test can be made to pass:
        fn hop<T: Hopper>(hopper: T) {
            hopper.hop();
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
        fn hop<T>(hopper: T) where T: Hopper {
            hopper.hop();
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
        fn hop_and_swim<T>(animal: T) where T: Hopper + Swimmer {
            animal.hop();
            animal.swim();
        }

        let duck = Duck { name: "Duck" };

        assert_eq!(hop_and_swim(duck), ());
    }
}

/// STANDARD TRAITS
///
/// The Rust standard library provides a number of traits that are commonly used in Rust programs.
/// These traits are implemented for many of the primitive types, and also for some standard library
/// types such as `String` and `Vec<T>`. In this section, you will explore these standard traits
/// and what capabilities they provide.
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

/// ASSOCIATED TYPES
///
/// Traits may have associated types, which are types that are defined as part of the trait
/// definition. Associated types are similar to generic type parameters, except that they are
/// specified as part of the trait definition, which makes them deterministic functions of the
/// trait parameters.
mod associated_types {
    #[test]
    fn pseudo_function() {
        trait Function<In> {
            type Out;

            fn call(&self, input: In) -> Self::Out;
        }

        struct Add {
            amount: i32,
        }

        impl Function<i32> for Add {
            type Out = i32;

            fn call(&self, input: i32) -> Self::Out {
                input + self.amount
            }
        }

        // Implement the Function trait for Add so the test can be made to pass:
        // Note the return type of the call to `call`, which is tracked by the associated type `Out`.
        assert_eq!(Add { amount: 1 }.call(2) as i32, 3);
    }
}

/// DYNAMIC TRAIT OBJECTS
///
/// A dynamic trait object is a pointer to a type that implements a trait. It is called dynamic
/// because the type of the object is not known at compile time, but rather at runtime. Dynamic
/// trait objects are useful when we want to abstract over types that implement a trait, but we
/// don’t know the exact type at compile time.
///
/// Programmers from other backgrounds can most easily understand a dynamic trait as an
/// "interface" (Java, C#), "abstract class" (C++), or "trait" (Scala, Kotlin). The defining
/// feature of a dynamic trait is dynamic dispatch, which means that the method that is called is
/// determined at runtime, rather than at compile time. This happens through a "virtual function
/// table", otherwise known as a "vtable", which is a table of function pointers that is stored
/// alongside the trait object.
///
/// In Rust, dynamic traits occupy the space of two pointers: one for the vtable, and one for the
/// data associated with the object. This additional level of indirection means that dynamic traits
/// are notably slower than static traits. However, in some cases the runtime cost of dynamic
/// dispatch is more than paid for by the benefits of abstraction.
///
/// The type of a dynamic trait object is `dyn Trait`, where `Trait` is the trait name. For
/// example, `dyn Animal` is a dynamic trait object that points to a type that implements the
/// `Animal` trait. However, `dyn Trait` cannot be returned from a function, or stored in a
/// variable, because the size of the data of the type is not known at compile time. In order to
/// work around this limitation, we can use a trait object wrapped in a `Box<T>`, which is a
/// pointer to a heap-allocated value of type `T` that we will learn more about later.
///
/// For all these reasons, when you are using dynamic trait objects, you will typically use the
/// `Box<dyn Trait>` type.
mod dynamic {
    #[test]
    fn dynamic_trait_object() {
        trait Animal {
            fn name(&self) -> &'static str;
            fn talk(&self) {
                println!("{} cannot talk", self.name());
            }
        }

        struct Human {
            name: &'static str,
        }

        impl Animal for Human {
            fn name(&self) -> &'static str {
                self.name
            }

            fn talk(&self) {
                println!("{} says hello", self.name());
            }
        }

        let sherlock  = Human { name: "Sherlock" };

        // Create a dynamic trait object from the Human struct:
        let sherlock_animal: Box<dyn Animal> = Box::new(sherlock);

        assert_eq!(sherlock_animal.name(), "Sherlock");
    }
}

mod existential {
    #[test]
    fn accept_impl() {
        trait DuckLike {
            fn quack(&self) -> String;
        }

        struct Duck {
            name: &'static str,
        }

        impl DuckLike for Duck {
            fn quack(&self) -> String {
                format!("{} is quacking", self.name)
            }
        }

        // Refactor this to use impl DuckLike instead of the trait bound:
        fn make_duck_quack(duck: impl DuckLike) -> String {
            duck.quack()
        }

        fn make_duck_quack2<T: DuckLike>(duck: T) -> String {
            duck.quack()
        }

        fn make_duck_quack3<T>(duck: T) -> String where T: DuckLike {
            duck.quack()
        }

        assert_eq!(
            make_duck_quack(Duck { name: "Donald" }),
            "Donald is quacking"
        );
    }

    #[test]
    fn return_impl() {
        trait DuckLike {
            fn quack(&self) -> String;
        }

        struct Duck {
            name: &'static str,
        }
        struct Platypus {
            name: &'static str,
        }

        impl DuckLike for Duck {
            fn quack(&self) -> String {
                format!("{} is quacking", self.name)
            }
        }

        impl DuckLike for Platypus {
            fn quack(&self) -> String {
                format!("{} is quacking", self.name)
            }
        }

        fn create_some_duck_dyn(name: &'static str) -> Box<dyn DuckLike> {
            Box::new(Duck { name })
        }

        // Refactor this to return an existential DuckLike using `impl`:
        fn create_some_duck(name: &'static str) -> impl DuckLike {
            Duck { name }
        }

        assert_eq!(
            create_some_duck("Donald").quack() as String,
            "Donald is quacking"
        );
    }
}
