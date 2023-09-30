use std::ops::{Add};

pub trait Magma<T> {
    fn add(&self, other: &T) -> T;
}

impl<T: Magma<T>> Add for T
where T: Magma<T>
{
    type Output = T;

    fn add(self, rhs: &T) -> T {
        return self.add(rhs);
    }
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

    #[derive(Clone, Copy)]
    struct Foo {
        x: i32,
    }

    impl Magma<Foo> for Foo {
        fn add(&self, other: &Foo) -> Foo {
            return Foo {
                x: self.x + other.x
            };
        }
    }

    #[test]
    fn i32_is_a_magma() {
        let x: i32 = 2;
        assert_eq!(x.add(&2), 4);
    }

    #[test]
    fn foo_is_magma_with_operator_overloading() {
        let foo_x = Foo { x: 2 };
        let foo_y = Foo { x: 3 };
        assert_eq!(foo_x + foo_y, Foo { x: 5 });
    }
}
