pub mod matrix;
pub use matrix::*;

#[cfg(test)]
mod tests {
    use crate::vector2d::Vector2d;

    #[test]
    fn vector2d_dot() {
        let mut v1 = Vector2d::new(1, 2);
        let v2 = Vector2d::new(3, 4);
        let res = v1.dot(v2);
        assert_eq!(res, 11);
    }

    fn vector2d_
}