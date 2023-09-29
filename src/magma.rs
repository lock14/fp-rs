pub trait Magma<T> {
    fn add(&self, other: &T) -> T;
}

#[cfg(test)]
mod tests {
    use super::*;

    // implement Magma for type i32
    impl Magma<i32> for i32 {
        fn add(&self, other: &i32) -> i32 {
            return self + other;
        }
    }

    #[test]
    fn i32_is_a_magma() {
        let x: i32 = 2;
        assert_eq!(x.add(&2), 4);
    }
}
