use crate::magma::Magma;

/// A Unital Magma is a [`Magma`] equipped
/// with an [identity](https://proofwiki.org/wiki/Definition:Identity_(Abstract_Algebra)/Two-Sided_Identity) element.
///<br><br>i.e.<br><br>
/// A Unital [`Magma`] defines a binary operation
/// 'add' (denoted hereafter by `+`)
/// over type `T`, and a unique element of T (denoted hereafter by `e`)
/// with the following properties:
/// * `+` is [closed](https://proofwiki.org/wiki/Definition:Closure_(Abstract_Algebra)/Algebraic_Structure) over type `T`
/// * `e` is both a [left identity](https://proofwiki.org/wiki/Definition:Identity_(Abstract_Algebra)/Left_Identity)
/// and a [right identity](https://proofwiki.org/wiki/Definition:Identity_(Abstract_Algebra)/Right_Identity)
/// under `+`
pub trait UnitalMagma<T>: Magma<T> {
    const IDENTITY: T;
}

//[identity element] on T
#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    impl UnitalMagma<i32> for i32 {
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
