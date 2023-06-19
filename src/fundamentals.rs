#![allow(dead_code, unreachable_code, unused_variables)]

// 00 - FUNDAMENTALS
//
// In this module, you will learn the fundamental syntax of Rust, without going into
// any real depth on the semantics of the language.
//
// After completing this module, you will be able to look at Rust code and mentally
// decompose it into its constituent parts, identifying the major components, such
// as literals, expressions, functions, traits, and so on.

/// VARIABLES
///
/// A variable is a name that refers to a value. In Rust, variables are immutable by
/// default, and must be explicitly marked as mutable. Variables can be declared with
/// the `let` keyword, and can be annotated with a type, or have their type inferred
/// by the compiler.
mod variables {
    #[test]
    fn immutable_variable() {
        let answer: i32 = 42;
        assert_eq!(answer as u32, 42);
    }

    #[test]
    fn mutable_variable() {
        let mut answer: i32 = 0;
        println!("Answer: {}", answer);
        answer = 42;

        assert_eq!(answer as u32, 42);
    }
}

/// LITERALS
///
/// A literal is a fixed value that appears directly in source code. In Rust, literals
/// have a type, which is inferred by the compiler, and can be explicitly annotated.
/// Rust supports a wide variety of literals, including integers, floats, booleans,
/// characters, strings, byte strings, arrays, tuples, and unit.
mod literals {
    #[test]
    fn signed_integer_literal_32() {
        let answer: i32 = 42;

        assert_eq!(answer, 42);
    }

    #[test]
    fn unsigned_integer_literal_32() {
        let answer: u32 = 42;

        assert_eq!(answer, 42u32);
    }

    #[test]
    fn signed_integer_literal_64() {
        let answer: i64 = 2147483648;

        assert_eq!(answer, 2_147_483_648i64);
    }

    #[test]
    fn unsigned_integer_literal_64() {
        let answer: u64 = 2147483648;

        assert_eq!(answer, 2_147_483_648u64);
    }

    #[test]
    fn float_literal() {
        let answer: f64 = 3.14159265358979323846264338327950288;

        assert_eq!(answer, 3.14159265358979323846264338327950288f64);
    }

    #[test]
    fn boolean_literal() {
        let answer: bool = true;

        assert_eq!(answer, true);
    }

    #[test]
    fn character_literal() {
        let answer: char = '🦀';

        assert_eq!(answer, '🦀');
    }

    #[test]
    fn string_literal() {
        let answer: &str = "Hello, world!";

        assert_eq!(answer, "Hello, world!");
    }

    #[test]
    fn byte_string_literal() {
        let answer: &[u8] = b"Hello, world!";

        assert_eq!(answer, b"Hello, world!");
    }

    fn byte_literal() {
        let answer: u8 = b'H';

        assert_eq!(answer, b'H');
    }

    #[test]
    fn array_literal() {
        let answer: [i32; 3] = [1, 2, 3];

        assert_eq!(answer, [1, 2, 3]);
    }

    #[test]
    fn tuple_literal() {
        let answer: (i32, f64, &str) = (1, 2.0, "three");

        assert_eq!(answer, (1, 2.0, "three"));
    }

    #[test]
    fn unit_literal() {
        let answer: () = ();

        assert_eq!(answer, ());
    }
}

/// EXPRESSIONS
///
/// As with other programming languages, Rust expressions describe a series of computations that
/// produce a value. Expressions can be as simple as a literal, or as complex as a chain of
/// function calls together with operators.
mod expressions {
    #[allow(unused_imports)]
    use std::ops::*;

    #[test]
    fn numeric_operator_expression() {
        let answer: i32 = { 2 + 2 };

        assert_eq!(answer, 4);
    }

    #[test]
    fn boolean_operator_expression() {
        let answer: bool = !true;

        assert_eq!(answer, false);
    }

    #[test]
    fn boolean_bit_operator_expression() {
        let answer: bool = true & false;

        assert_eq!(answer, false);
    }

    #[test]
    fn if_else_expression() {
        // ternary operator?
        let answer: i32 = if true { 1 } else { 2 };

        assert_eq!(answer, 1);
    }

    #[test]
    fn match_expression() {
        let result: Result<i32, String> = Result::Ok(42);

        let answer: i32 = match result {
            Ok(value) => value,
            Err(_) => todo!("Handle error"),
        };

        assert_eq!(answer, 42);
    }

    #[test]
    fn field_expression() {
        struct Person {
            name: &'static str,
            age: i32,
        }

        let person: Person = Person {
            name: "Alice",
            age: 42,
        };

        let answer: &str = person.name;

        assert_eq!(answer, "Alice");
    }

    #[test]
    fn tuple_expression() {
        let tuple = (1, 2.0, "three");

        let answer: &str = tuple.2;

        assert_eq!(answer, "three");
    }

    #[test]
    fn block_expression() {
        let answer: i32 = {
            println!(".");
            42
        };

        assert_eq!(answer, 42);
    }

    #[test]
    fn function_call_expression() {
        fn add_one(x: i32) -> i32 {
            x + 1
        }

        let answer: i32 = add_one(41);

        assert_eq!(answer, 42);
    }

    #[test]
    fn method_call_expression() {
        let answer: String = 42.to_string();

        assert_eq!(answer, "42");
    }

    #[test]
    fn array_indexing_expression() {
        let array: [i32; 3] = [1, 2, 3];
        let answer: i32 = array[0];

        assert_eq!(answer, 1);
    }

    #[test]
    fn closure_expression() {
        let answer = |x: i32| x + 1;

        assert_eq!(answer(41), 42);
    }

    #[test]
    fn range_inclusive_expression() {
        let mut range: RangeInclusive<i32> = todo!("Create a range from 1 to 3");

        let found = range.find(|&x| x == 3);

        assert_eq!(found, Some(3));
    }

    #[test]
    fn range_exclusive_expression() {
        let mut range: Range<i32> = 1..3;

        let found = range.find(|&x| x == 3);

        assert_eq!(found, None);
    }

    #[test]
    fn loop_with_break_expression() {
        let answer: i32 = loop {
            println!("Looping...");

            break 42;
        };

        assert_eq!(answer, 42);
    }

    #[test]
    fn for_loop_expression() {
        #[allow(unused_mut)]
        let mut sum = 0;

        let range = 1..=3;

        let answer: () = for x in range {
            sum = sum + x;
        };

        assert_eq!(sum, 6);
    }
}

/// STATEMENTS
///
/// Statements are how you tell the compiler to perform some action, such as printing to the
/// console, or declaring a variable. Statements are not expressions, and do not produce a
/// value, but are important.
mod statements {
    #[test]
    fn item_statement() {
        fn answer() -> i32 {
            42
        }

        assert_eq!(answer(), 42);
    }

    #[test]
    fn print_statement() {
        println!("Hello, world!");

        assert_eq!((), ());
    }

    #[test]
    fn let_statements() {
        let two: i32 = 2;
        let four: i32 = 4;

        assert_eq!(two + two, four);
    }

    #[test]
    fn assignment_statement() {
        #[allow(unused_mut)]
        let mut answer: i32 = 0;

        println!("The wrong answer is {}", answer);

        todo!("Assign 42 to answer");

        assert_eq!(answer, 42);
    }

    #[test]
    fn if_statement() {
        #[allow(unused_mut)]
        let mut answer: i32 = 0;

        println!("The wrong answer is {}", answer);

        if todo!("If true, assign 42 to answer") {
            answer = 42;
        }

        assert_eq!(answer, 42);
    }

    #[test]
    fn if_else_statement() {
        #[allow(unused_mut)]
        let mut answer: i32 = 0;

        println!("The wrong answer is {}", answer);

        if todo!("If true, assign 42 to answer") {
            answer = 42;
        } else {
            answer = -42;
        }

        assert_eq!(answer, 42);
    }

    #[test]
    fn expression_statement() {
        #[allow(unused_mut)]
        let mut buf = String::new();

        println!("Press any key to continue...");

        todo!("Read a line into &mut buf with std::io::stdin()");

        assert_eq!((), ());
    }
}

/// ITEMS
///
/// Items are the major components of a Rust program, and include functions, structs, enums,
/// traits, modules, type aliases, unions, and so on. Items are the declaration-level building
/// blocks of a Rust program.
mod items {
    #[test]
    fn function_item() {
        // Declare a function named `answer` that returns 42.

        // Call the function:
        let value: i32 = todo!("answer()");

        assert_eq!(value, 42);
    }

    #[test]
    fn struct_item() {
        // Declare a struct item named `Person` that has name and age.
        // The type of name is `&'static str` and the type of age is `i32`.
        struct Person {}

        let person: Person = todo!("Person {{ name: \"Alice\", age: 42 }}");

        assert_eq!(todo!("person.name") as &str, "Alice");
        assert_eq!(todo!("person.age") as i32, 42);
    }

    #[test]
    fn enum_item() {
        // Declare an enum item named `Direction` that has four variants:
        // `North`, `South`, `East`, and `West`.
        #[derive(PartialEq, Eq, Debug)]
        enum Direction {}

        let direction: Direction = todo!("Direction::North");

        assert_eq!(direction, todo!("Direction::North") as Direction);
    }

    #[test]
    fn trait_item() {
        // Declare a trait item named `Answer` that has a single function named `answer`
        // that returns an `i32`.
        trait Answer {}

        struct Question {}

        impl Question {
            // impl Answer for Question
            fn answer(&self) -> i32 {
                42
            }
        }

        let answer: i32 = Question {}.answer();

        assert_eq!(answer, 42);
    }

    #[test]
    fn module_item() {
        // Declare a module item named `math` that has a function named `add` that adds
        // two `i32` values together.
        mod math {}

        let answer: i32 = todo!("math::add(40, 2)");

        assert_eq!(answer, 42);
    }

    #[test]
    fn type_alias_item() {
        // Declare a type alias named `Answer` that is an `i32`.
        type Answer = ();

        let answer: Answer = todo!("42");

        assert_eq!(answer, todo!("42"));
    }

    #[test]
    fn union_item() {
        // Declare a union named `Number` that has two fields: `int: i32` and `float: f64`.
        union Number {
            float: f32,
        }

        let answer: Number = todo!("Number {{ int: 42 }}");

        assert_eq!(unsafe { answer.float }, 5.9e-44);
    }

    #[test]
    fn use_item() {
        // Declare a use item that brings the `std::collections::HashMap` type into scope
        // as `Map`.

        // let mut map: Map<i32, &str> = Map::new();
        // map.insert(42, "the answer");
        // assert_eq!(map.get(&42), Some(&"the answer"));
    }
}
