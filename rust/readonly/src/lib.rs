/// A struct that wraps any value and makes it readonly/immutable.
///
/// # Example
///
/// Given any value or data structure you want to make readonly, e.g.
///
/// ```rust
/// struct Data {
///     property: i32
/// }
/// ```
///
/// You can simple wrap it to protect it for mutable access, but still be able to access the data:
///
/// ```rust
/// use readonly::Readonly;
///
/// struct Data { property: i32 }
///
/// let data = Data { property: 1 };
/// let readonly_data = Readonly::new(data);
///
/// assert_eq!(readonly_data.property, 1);
/// ```
///
/// Following example fails compilation, because you cannot mutate the value inside
///
/// ```rust,compile_fail
/// use readonly::Readonly;
///
/// struct Data { property: i32 }
/// let data = Data { property: 2 };
/// let mut readonly_data = Readonly::new(data); // warning, because of `mut` keyword
///
/// readonly_data.property = 3; // fails compilation
/// ```
#[derive(Debug, PartialEq)]
pub struct Readonly<T> {
    value: T
}

impl<T> Readonly<T> {
    pub fn new(value: T) -> Self {
        Readonly { value }
    }
}

impl<T> std::ops::Deref for Readonly<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod test {
    use super::Readonly;

    /// example data structure
    struct Data {
        property: i32
    }

    #[test]
    fn primitive() {
        let readonly_data = Readonly::new(4);
        println!("# {:?}", readonly_data);
        assert_eq!(*readonly_data, 4);
    }

    #[test]
    fn r#struct() {
        let data = Data { property: 5 };
        let readonly_data = Readonly::new(data);
        assert_eq!(readonly_data.property, 5);
    }
}

