use std::ops::Add;

/// A `Magma` defines a binary operation 'add' (denoted hereafter by `+`)
/// over type `T` with the following properties:
/// * `+` is [closed](https://proofwiki.org/wiki/Definition:Closure_(Abstract_Algebra)/Algebraic_Structure) over type `T`
pub trait Magma: Add<Self, Output = Self>
where
    Self: Sized,
{
}

// implement Magma for types that implement Add
// https://doc.rust-lang.org/std/ops/trait.Add.html#implementors

impl Magma for f32 {}
impl Magma for f64 {}
impl Magma for i8 {}
impl Magma for i16 {}
impl Magma for i32 {}
impl Magma for i64 {}
impl Magma for i128 {}
impl Magma for isize {}
impl Magma for u8 {}
impl Magma for u16 {}
impl Magma for u32 {}
impl Magma for u64 {}
impl Magma for u128 {}
impl Magma for usize {}

#[cfg(test)]
mod tests {
    use super::*;

    // define custom struct and implement Magma
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct Foo {
        x: i32,
    }

    impl Add<Self> for Foo {
        type Output = Foo;

        fn add(self, rhs: Self) -> Self::Output {
            return Foo { x: self.x + rhs.x };
        }
    }

    impl Magma for Foo {}

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
