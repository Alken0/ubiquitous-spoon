use crate::error::{Error, Result};

pub fn assert_vec_equal<T: std::cmp::PartialEq + std::fmt::Debug>(
    vec1: &[T],
    vec2: &[T],
    message: &'static str,
) -> Result<()> {
    let mut equal = vec1.len() == vec2.len();
    for e in vec1 {
        equal = vec2.contains(e) && equal;
    }

    if !equal {
        return Err(Error::msg(format!(
            "\"{}\" \n{:?}\n{:?}\n",
            message, vec1, vec2
        )));
    }

    return Ok(());
}
