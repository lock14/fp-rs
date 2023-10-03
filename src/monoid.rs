use crate::semigroup::Semigroup;
use crate::unital_magma::UnitalMagma;

/// A `Monoid` is a [`UnitalMagma`] and a [`Semigroup`]
///
/// i.e.
///
/// A `Monoid` defines a binary operation
/// 'add' (denoted hereafter by `+`)
/// over type `T`, and a unique element of T (denoted hereafter by `e`)
/// with the following properties:
/// * `+` is [closed](https://proofwiki.org/wiki/Definition:Closure_(Abstract_Algebra)/Algebraic_Structure) over type `T`
/// * `+` is [associative](https://proofwiki.org/wiki/Definition:Associative_Operation).
/// * `e` is an [identity](https://proofwiki.org/wiki/Definition:Identity_(Abstract_Algebra)/Two-Sided_Identity) under `+`.
pub trait Monoid: UnitalMagma + Semigroup {}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    impl Monoid for i32 {}

    #[test]
    fn add_is_associative_for_i32() {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(-100..=100);
        let b = rng.gen_range(-100..=100);
        let c = rng.gen_range(-100..=100);
        assert_eq!((a + b) + c, a + (b + c))
    }

    #[test]
    fn identity_element_is_left_identity() {
        let mut rng = rand::thread_rng();
        let random_val = rng.gen_range(-100..=100);
        assert_eq!(i32::IDENTITY + random_val, random_val)
    }

    #[test]
    fn identity_element_is_right_identity() {
        let mut rng = rand::thread_rng();
        let random_val = rng.gen_range(-100..=100);
        assert_eq!(random_val + i32::IDENTITY, random_val)
    }
}
