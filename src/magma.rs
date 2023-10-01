use std::ops::{Add};

pub trait Magma<T>: Add<T, Output=T> {}

#[cfg(test)]
mod tests {
    use super::*;

    // implement Magma for type i32
    impl Magma<i32> for i32 {

    }

    // define custom struct and implement Magma
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct Foo {
        x: i32,
    }

    impl Add for Foo {
        type Output = Foo;

        fn add(self, rhs: Foo) -> Foo {
            return Foo{x: self.x + rhs.x}
        }
    }

    impl Magma<Foo> for Foo {
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
        assert_eq!(foo_x.add(foo_y), Foo { x: 5 });
    }
}
