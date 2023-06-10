#![allow(dead_code, unreachable_code, unused_variables)]

// 04 -- ERRORS
//
// Rust does not have exceptions. Instead, it has built-in data types that you can use for error
// handling, and you can always create your own data types. In addition, to ease the process of
// propagating errors, Rust provides the ? operator. When you place the ? after a Result value,
// the value of the Result is returned from the current function as if you had used the return
// keyword so the error value gets propagated to the calling code.
//
// These choices mean that Rust programs are easily robust to errors, because the static type
// system of Rust is constantly checking that errors are being handled. Yet, the syntax added
// for error propagation ensures that boilerplate error handling code is minimized. This achieves
// the best of both worlds: Rust programs are robust and easy to write.

/// OPTION
///
/// Rust does not support "nulls". Instead, Rust has an enum named Option<T> that is used to
/// represent the presence or absence of a value. The Option<T> enum has two constructors:
/// Some and None. The Some constructor is used to represent the presence of a value, and
/// the None constructor is used to represent the absence of a value.
///
/// In this section, you will become more familiar with the Option data type.
mod option {
    #[test]
    fn basic_option() {
        // Option is an enum with two constructors: Some and None.
        // Use this enum to implement the divide function so the test can be made to pass:
        fn divide(numerator: f64, denominator: f64) -> Option<f64> {
            todo!("don't allow division by zero");
        }

        assert_eq!(divide(4.0, 2.0), Some(2.0));
        assert_eq!(divide(5.0, 0.0), None);
    }

    #[test]
    fn option_unwrap() {
        // The Option type has a method called unwrap that returns the value inside a Some value, or
        // panics if the value is None. This is useful when you know that the value will always be
        // Some, for example, when you are using a hard-coded value.

        fn divide(numerator: f64, denominator: f64) -> Option<f64> {
            if denominator == 0.0 {
                None
            } else {
                Some(numerator / denominator)
            }
        }

        // Use the unwrap method to either unwrap the Option or panic:
        assert_eq!(todo!("divide(4.0, 2.0)") as f64, 2.0);
    }

    #[test]
    fn unwrap_or() {
        // The Option type has a method called unwrap_or that returns the value inside a Some
        // value, or returns the specified constant value if the option value is None. This is
        // useful when you want to provide a default value for a value that might be None.

        fn divide(numerator: f64, denominator: f64) -> Option<f64> {
            if denominator == 0.0 {
                None
            } else {
                Some(numerator / denominator)
            }
        }

        // Use the unwrap_or method to either unwrap the Option or return a default value:
        assert_eq!(todo!("divide(4.0, 2.0)") as f64, 2.0);
        assert_eq!(todo!("divide(5.0, 0.0)") as f64, 0.0);
    }

    #[test]
    fn unwrap_or_else() {
        // The Option type has a method called unwrap_or_else that returns the value inside a Some
        // value, or returns the result of calling the specified function if the option value is
        // None. This is useful when you want to provide a default value for a value that might be
        // None, but the default value is expensive to compute.

        fn divide(numerator: f64, denominator: f64) -> Option<f64> {
            if denominator == 0.0 {
                None
            } else {
                Some(numerator / denominator)
            }
        }

        // Use the unwrap_or_else method to either unwrap the Option or return a default value:
        assert_eq!(todo!("divide(4.0, 2.0)") as f64, 2.0);
        assert_eq!(todo!("divide(5.0, 0.0)") as f64, 0.0);
    }

    #[test]
    fn option_map() {
        // The Option type has a method called map that returns a new Option value by applying the
        // specified function to the value inside a Some value, or returns None if the option value
        // is None. This is useful when you want to transform the value inside a Some value, but
        // ignore the None value.

        fn divide(numerator: f64, denominator: f64) -> Option<f64> {
            if denominator == 0.0 {
                None
            } else {
                Some(numerator / denominator)
            }
        }

        fn divide_render(numerator: f64, denominator: f64) -> String {
            // Use the map method to either transform the Option. Then unwrap safely,
            // using empty string if the division failed:
            todo!("don't allow division by zero");
        }

        // Use the map method to either transform the Option or return None:
        assert_eq!(divide_render(4.0, 2.0), "2.0");
    }

    #[test]
    fn option_and_then() {
        // The Option type has a method called and_then that returns a new Option value by applying
        // the specified function to the value inside a Some value, or returns None if the option
        // value is None. This is useful when you want to transform the value inside a Some value,
        // but ignore the None value.

        fn decode(s: &str) -> Option<i32> {
            match s.parse::<i32>() {
                Ok(n) => Some(n),
                Err(_) => None,
            }
        }

        fn divide(numerator: f64, denominator: f64) -> Option<f64> {
            if denominator == 0.0 {
                None
            } else {
                Some(numerator / denominator)
            }
        }

        fn decode_and_then_divide(numerator: &str, denominator: &str) -> Option<f64> {
            // Use the and_then method to either transform the Option or return None:
            todo!("don't allow division by zero");
        }

        assert_eq!(decode_and_then_divide("4", "2"), Some(2.0));
    }
}

/// RESULT
///
/// The Result<Ok, Err> type is used to represent the result of an operation that may fail. The
/// Ok value represents the successful result, and the Err value represents the error result.
/// This is the idiomatic way to describe and handle recoverable (non-fatal) errors in Rust.
///
/// The Result type is an enum with two constructors: Ok and Err. The Ok constructor is used to
/// represent the successful result, and the Err constructor is used to represent the error result.
///
/// Although you can use any type you want for Err, the standard library defines a trait named
/// std::error::Error, which your errors should generally implement.
mod result {
    #[test]
    fn basic_result() {
        // Result is an enum with two constructors: Ok and Err.
        // Use this enum to implement the divide function so the test can be made to pass:
        fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
            todo!("don't allow division by zero");
        }

        assert_eq!(divide(4.0, 2.0), Ok(2.0));
        assert_eq!(divide(5.0, 0.0), Err("division by zero"));
    }

    #[test]
    fn result_unwrap() {
        // The Result type has a method called unwrap that returns the value inside an Ok value, or
        // panics if the value is Err. This is useful when you know that the value will always be
        // Ok, for example, when you are using a hard-coded value.

        fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
            if denominator == 0.0 {
                Err("division by zero")
            } else {
                Ok(numerator / denominator)
            }
        }

        // Use the unwrap method to either unwrap the Result or panic:
        assert_eq!(todo!("divide(4.0, 2.0)") as f64, 2.0);
    }

    #[test]
    fn unwrap_or() {
        // The Result type has a method called unwrap_or that returns the value inside an Ok
        // value, or returns the specified constant value if the result value is Err. This is
        // useful when you want to provide a default value for a value that might be Err.

        fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
            if denominator == 0.0 {
                Err("division by zero")
            } else {
                Ok(numerator / denominator)
            }
        }

        // Use the unwrap_or method to either unwrap the Result or return a default value:
        assert_eq!(todo!("divide(4.0, 2.0)") as f64, 2.0);
        assert_eq!(todo!("divide(5.0, 0.0)") as f64, 0.0);
    }

    #[test]
    fn unwrap_or_else() {
        // The Result type has a method called unwrap_or_else that returns the value inside an Ok
        // value, or returns the result of calling the specified function if the result value is
        // Err. This is useful when you want to provide a default value for a value that might be
        // Err, but the default value is expensive to compute.

        fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
            if denominator == 0.0 {
                Err("division by zero")
            } else {
                Ok(numerator / denominator)
            }
        }

        // Use the unwrap_or_else method to either unwrap the Result or return a default value:
        assert_eq!(todo!("divide(4.0, 2.0)") as f64, 2.0);
        assert_eq!(todo!("divide(5.0, 0.0)") as f64, 0.0);
    }

    #[test]
    fn result_map() {
        // The Result type has a method called map that returns a new Result value by applying the
        // specified function to the value inside an Ok value, or returns Err if the result value
        // is Err. This is useful when you want to transform the value inside an Ok value, but
        // ignore the Err value.

        fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
            if denominator == 0.0 {
                Err("division by zero")
            } else {
                Ok(numerator / denominator)
            }
        }

        fn divide_render(numerator: f64, denominator: f64) -> Result<String, &'static str> {
            // Use the map method to either transform the Result or return Err:
            todo!("don't allow division by zero");
        }

        assert_eq!(divide_render(4.0, 2.0), Ok("2.0".to_string()));
        assert_eq!(divide_render(5.0, 0.0), Err("division by zero"));
    }

    #[test]
    fn result_and_then() {
        // The Result type has a method called and_then that returns a new Result value by applying
        // the specified function to the value inside an Ok value, or returns Err if the result
        // value is Err. This is useful when you want to transform the value inside an Ok value,
        // but ignore the Err value.

        fn decode(s: &str) -> Result<i32, &'static str> {
            match s.parse::<i32>() {
                Ok(n) => Ok(n),
                Err(_) => Err("invalid number"),
            }
        }

        fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
            if denominator == 0.0 {
                Err("division by zero")
            } else {
                Ok(numerator / denominator)
            }
        }

        fn decode_and_then_divide(numerator: &str, denominator: &str) -> Result<f64, &'static str> {
            // Use the map method to either transform the Result or return Err:
            todo!("don't allow division by zero");
        }

        assert_eq!(decode_and_then_divide("4", "2"), Ok(2.0));
        assert_eq!(decode_and_then_divide("4", "0"), Err("division by zero"));
    }

    #[test]
    fn question_mark() {
        fn decode(s: &str) -> Result<i32, &'static str> {
            match s.parse::<i32>() {
                Ok(n) => Ok(n),
                Err(_) => Err("invalid number"),
            }
        }

        fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
            if denominator == 0.0 {
                Err("division by zero")
            } else {
                Ok(numerator / denominator)
            }
        }

        fn decode_and_then_divide(numerator: &str, denominator: &str) -> Result<f64, &'static str> {
            // Use the ? operator to either unwrap the Result or return Err:
            todo!("don't allow division by zero");
        }

        assert_eq!(decode_and_then_divide("4", "2"), Ok(2.0));
        assert_eq!(decode_and_then_divide("4", "0"), Err("division by zero"));
    }
}

/// PANICS
///
/// Sometimes, your program will encounter an error that it cannot recover from, such as a fatal or
/// catastrophic error, or perhaps a bug in your program. In these cases, it is appropriate to
/// panic, which causes your program to abort. Panics should not be used for recoverable errors.
/// When in doubt, do not panic!
///
/// In this section, you will learn how to panic.
mod panics {
    #[test]
    fn basic_panic() {
        // Use the panic! macro to panic with a message:
        todo!("panic with a message");
    }

    #[test]
    fn catch_unwind_panic() {
        let result = std::panic::catch_unwind(|| {
            // Use the panic! macro to panic with a message:
            todo!("panic with a message");
        });

        assert!(result.is_err());
    }
}
