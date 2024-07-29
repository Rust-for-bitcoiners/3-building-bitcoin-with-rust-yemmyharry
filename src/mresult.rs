#![allow(unused)]

enum MResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MResult<T, E> {
    fn ok(value: T) -> Self {
        MResult::Ok((value))
    }
    // Function to create an Err variant
    fn err(error: E) -> Self {
        MResult::Err((error))
    }

    // Method to check if it's an Ok variant
    fn is_ok(&self) -> bool {
        matches!(self, MResult::Ok(_))
    }

    // Method to check if it's an Err variant
    fn is_err(&self) -> bool {
        matches!(self, MResult::Err(_))
    }

    // Method to unwrap the Ok value, panics if it's an Err
    fn unwrap(self) -> T {
        match self {
            MResult::Ok(value) => value,
            MResult::Err(_) => panic!("called `MResult::unwrap()` on an `Err` value"),
        }
    }

    // Method to unwrap the Err value, panics if it's an Ok
    fn unwrap_err(self) -> E {
        match self {
            MResult::Err(value) => value,
            MResult::Ok(_) => panic!("called `MResult::unwrap_err()` on an `Ok` value"),
        }
    }
}

// Add unit tests below