use crate::magma::Magma;

/// A `UnitalMagma` is a [`Magma`] equipped with an identity element.
///
/// i.e.
///
/// A `Unital Magma` defines a binary operation
/// 'add' (denoted hereafter by `+`)
/// over type `T`, and a unique element of T (denoted hereafter by `e`)
/// with the following properties:
/// * `+` is [closed](https://proofwiki.org/wiki/Definition:Closure_(Abstract_Algebra)/Algebraic_Structure) over type `T`
/// * `e` is an [identity](https://proofwiki.org/wiki/Definition:Identity_(Abstract_Algebra)/Two-Sided_Identity) under `+`.
pub trait UnitalMagma: Magma {
    const IDENTITY: Self;
}

//[identity element] on T
#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    impl UnitalMagma for i32 {
        const IDENTITY: i32 = 0;
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
