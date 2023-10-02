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
    fn identity_element() -> T;
}

//[identity element] on T
#[cfg(test)]
mod tests {
    use rand::Rng;
    use super::*;
    impl UnitalMagma<i32> for i32 { fn identity_element() -> i32 { 0 } }

    #[test]
    fn idenenty_element_is_left_identity() {
        let mut rng = rand::thread_rng();
        let random_val = rng.gen_range(-100..=100);
        assert_eq!(i32::identity_element() + random_val, random_val)
    }

    fn idenenty_element_is_right_identity() {
        let mut rng = rand::thread_rng();
        let random_val = rng.gen_range(-100..=100);
        assert_eq!(random_val + i32::identity_element(), random_val)
    }
}