pub mod matrix;
pub use matrix::*;

#[cfg(test)]
mod tests {
    use crate::vector2d::Vector2d;
    use crate::vector3d::Vector3d;

    #[test]
    fn vector2d_dot() {
        let mut v1 = Vector2d::new(1, 2);
        let v2 = Vector2d::new(3, 4);
        let res = v1.dot(v2);
        assert_eq!(res, 11);
    }

    #[test]
    fn vector2d_add() {
        let mut v1 = Vector2d::identity(1);
        let v = v1.add(v1.clone());
        assert_eq!(v.x, 2);
        assert_eq!(v.y, 2);
    }

    #[test]
    fn vector2d_sub() {
        let mut v1 = Vector2d::identity(1);
        let v = v1.sub(v1.clone());
        assert_eq!(v.x, 0);
        assert_eq!(v.y, 0);
    }

    #[test]
    fn vector2d_mul() {
        let mut v1 = Vector2d::identity(2);
        let v = v1.mul(v1.clone());
        assert_eq!(v.x, 4);
        assert_eq!(v.y, 4);
    }

    #[test]
    fn vector2d_div() {
        let mut v1 = Vector2d::identity(2);
        let v = v1.div(v1.clone());
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 1);
    }

    #[test]
    fn vector2d_add_item() {
        let v1 = Vector3d::identity(2);
        let v = Vector3d::add_item(v1, 1);
        assert_eq!(v.x, 3);
        assert_eq!(v.y, 3);
        assert_eq!(v.z, 3);
    }

    #[test]
    fn vector2d_sub_item() {
        let v1 = Vector3d::identity(2);
        let v = Vector3d::sub_item(v1, 1);
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 1);
        assert_eq!(v.z, 1);
    }

    #[test]
    fn vector2d_mul_item() {
        let v1 = Vector3d::identity(2);
        let v = Vector3d::mul_item(v1, 5);
        assert_eq!(v.x, 10);
        assert_eq!(v.y, 10);
        assert_eq!(v.z, 10);
    }

    #[test]
    fn vector2d_div_item() {
        let v1 = Vector3d::identity(4);
        let v = Vector3d::div_item(v1, 2);
        assert_eq!(v.x, 2);
        assert_eq!(v.y, 2);
        assert_eq!(v.z, 2);
    }

    #[test]
    fn vector3d_dot() {
        let mut v1 = Vector3d::new(1, 2, 3);
        let v2 = Vector3d::new(4, 5, 6);
        let res = v1.dot(v2);
        assert_eq!(res, 32);
    }

    #[test]
    fn vector3d_add() {
        let mut v1 = Vector3d::identity(1);
        let v = v1.add(v1.clone());
        assert_eq!(v.x, 2);
        assert_eq!(v.y, 2);
        assert_eq!(v.z, 2);
    }

    #[test]
    fn vector3d_sub() {
        let mut v1 = Vector3d::identity(1);
        let v = v1.sub(v1.clone());
        assert_eq!(v.x, 0);
        assert_eq!(v.y, 0);
        assert_eq!(v.z, 0);
    }

    #[test]
    fn vector3d_mul() {
        let mut v1 = Vector3d::identity(2);
        let v = v1.mul(v1.clone());
        assert_eq!(v.x, 4);
        assert_eq!(v.y, 4);
        assert_eq!(v.z, 4);
    }

    #[test]
    fn vector3d_div() {
        let mut v1 = Vector3d::identity(2);
        let v = v1.div(v1.clone());
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 1);
        assert_eq!(v.z, 1);
    }

    #[test]
    fn vector3d_add_item() {
        let v1 = Vector3d::identity(4);
        let v = Vector3d::add_item(v1, 2);
        assert_eq!(v.x, 6);
        assert_eq!(v.y, 6);
        assert_eq!(v.z, 6);
    }

    #[test]
    fn vector3d_mul_item() {
        let v1 = Vector3d::identity(4);
        let v = Vector3d::mul_item(v1, 2);
        assert_eq!(v.x, 8);
        assert_eq!(v.y, 8);
        assert_eq!(v.z, 8);
    }

    #[test]
    fn vector3d_sub_item() {
        let v1 = Vector3d::identity(4);
        let v = Vector3d::sub_item(v1, 2);
        assert_eq!(v.x, 2);
        assert_eq!(v.y, 2);
        assert_eq!(v.z, 2);
    }

    #[test]
    fn vector3d_div_item() {
        let v1 = Vector3d::identity(4);
        let v = Vector3d::div_item(v1, 2);
        assert_eq!(v.x, 2);
        assert_eq!(v.y, 2);
        assert_eq!(v.z, 2);
    }
}