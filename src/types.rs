#![allow(dead_code, unreachable_code, unused_variables)]

/// 01 - TYPES
///
/// Rust is a statically typed programming language, which means that every expression, function
/// input, and function output has a "type" that is known statically at compile-time. The type
/// determines a set of values which are permissible at this point in the program, and can be
/// thought of as a compile-time description of how the memory that holds a value is to be
/// interpreted. Static types allow you to prevent invalid use of memory, preventing a large
/// class of runtime errors from ever occurring.
///
/// In this section, you will learn about how to declare your own types and how to use those
/// types in very simple ways.

/// STRUCTS
///
/// A struct is a type that you can define, which has a name and a set of fields. A
/// struct can be thought of as a record or object, similar to a class in object-oriented
/// languages, but without any methods (structs are pure data).
///
/// The fields of a struct may be anonymous, in which case the struct is called a tuple struct,
/// and the fields of the struct are accessed by index. An ordinary struct, however, has named
/// fields, and the fields of the struct are accessed by name.
mod structs {
    #[test]
    fn basic_struct_example() {
        // Add `name` and `age` fields to this struct, of type `String` and `u32` respectively.
        struct Person {}

        assert_eq!(std::mem::size_of::<Person>(), 32);
    }

    #[test]
    fn basic_tuple_struct_example() {
        // Add `name` and `age` fields to this tuple struct, of type `String` and `u32` respectively.
        struct Person();

        assert_eq!(std::mem::size_of::<Person>(), 32);
    }

    #[test]
    fn struct_debug_example() {
        // Derive the `Debug` trait for this struct, so that it can be printed.
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person = Person {
            name: "John Doe",
            age: 42,
        };

        assert_eq!(
            format!("{:?}", todo!("person")),
            "Person { name: \"John Doe\", age: 42 }"
        );
    }

    #[test]
    fn struct_eq_example() {
        // Derive the `PartialEq` and `Eq` traits for this struct, so that it can be compared for
        // equality.
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person1 = Person {
            name: "John Doe",
            age: 42,
        };
        let person2 = Person {
            name: "John Doe",
            age: 42,
        };

        assert_eq!(todo!("person1"), todo!("person2"));
    }

    #[test]
    fn struct_clone_example() {
        // Derive the `Clone` trait for this struct, so that it can be cloned.
        #[derive(Debug, PartialEq, Eq)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person1 = Person {
            name: "John Doe",
            age: 42,
        };
        let person2 = todo!("person1.clone()");

        assert_eq!(person1, person2);
    }

    #[test]
    fn struct_is_deep_clone() {
        // Derive the `Clone` trait for this struct, so that it can be cloned.
        struct Person {
            address: Address,
        }

        struct Address {
            street: u32,
        }

        let old_person: Person = Person {
            address: Address { street: 42 },
        };
        let new_person: Person = todo!("old_person.clone()");

        old_person.address.street = 0;

        assert_eq!(new_person.address.street, 42);
    }

    #[test]
    fn struct_copy() {
        // Derive the `Copy` trait for these structs, so that they can be copied.
        #[derive(Debug, PartialEq, Eq)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person1 = Person {
            name: "John Doe",
            age: 42,
        };
        let person2 = todo!("person1");

        assert_eq!(person1, person2);
    }

    #[test]
    fn struct_hash() {
        #![allow(unused_mut)]

        // Derive the `Hash` trait for this struct, so that it can be used in a `HashMap`.
        #[derive(Debug, Hash, PartialEq, Eq, Clone)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        use std::collections::HashMap;

        let mut person_to_address: HashMap<Person, &str> = HashMap::new();

        let sherlock = Person {
            name: "Sherlock Holmes",
            age: 42,
        };

        // person_to_address.insert(sherlock.clone(), "221B Baker Street");
        let gotten: Option<&&str> = todo!("person_to_address.get(&sherlock)");

        assert_eq!(gotten, Some(&"221B Baker Street"));
    }

    #[test]
    fn struct_default() {
        // Derive the `Default` trait for this struct, so that it can be created with `Default::default()`.
        #[derive(Debug, PartialEq, Eq)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person: Person = todo!("Default::default()");

        assert_eq!(person, Person { name: "", age: 0 });
    }

    #[test]
    fn destructuring_struct() {
        #[derive(Debug, PartialEq, Eq)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person = Person {
            name: "John Doe",
            age: 42,
        };

        // Destructure the person struct into its fields, so that the test passes.
        // The syntax is the same as for construction, except the field names are
        // on the left-hand side of the `=` sign, and you use `let` to create the
        // variables that are bound to the fields.

        assert_eq!(todo!("name") as &'static str, "John Doe");
        assert_eq!(todo!("age") as u32, 42);
    }

    #[test]
    fn pattern_matching_struct() {
        #[derive(Debug, PartialEq, Eq)]
        struct Person {
            name: &'static str,
            age: u32,
        }

        let person = Person {
            name: "John Doe",
            age: 42,
        };

        // Match on the person struct and extract out the name and age fields,
        // and put them into a tuple.
        let (name, age) = todo!("match person ...") as (&'static str, u32);

        assert_eq!(todo!("name") as &'static str, "John Doe");
        assert_eq!(todo!("age") as u32, 42);
    }
}

/// ENUMS
///
/// An enum is another data type that you can define, which has a name and a set of variants. An
/// enum corresponds to a "sum type" in functional programming, and can be thought of as a tagged
/// union. An enum value is constructed using one of the variants, and can be destructed using
/// pattern matching. Rust ensures that pattern matches against enums are exhaustive, meaning that
/// you must handle every possible variant of the enum.
///
/// Enums are used in the Rust standard library for a variety of purposes, including (and notably)
/// error handling.
mod enums {}
