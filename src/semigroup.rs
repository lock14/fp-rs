use crate::magma::Magma;

/// A `Semigroup` is a [`Magma`] where the 'add' operation is associative.
///
/// i.e.
///
/// A `Semigroup` defines a binary operation 'add' (denoted hereafter by `+`)
/// over type `T` with the following properties:
/// * `+` is [closed](https://proofwiki.org/wiki/Definition:Closure_(Abstract_Algebra)/Algebraic_Structure) over type `T`
/// * `+` is [associative](https://proofwiki.org/wiki/Definition:Associative_Operation).
pub trait Semigroup: Magma {}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    impl Semigroup for i32 {}

    #[test]
    fn add_is_associative_for_i32() {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(-100..=100);
        let b = rng.gen_range(-100..=100);
        let c = rng.gen_range(-100..=100);
        assert_eq!((a + b) + c, a + (b + c))
    }
}
