pub fn some<T: Default>() -> T {
    T::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let some_val = some::<i32>();

        assert_eq!(i32::default(), some_val);
    }
}
