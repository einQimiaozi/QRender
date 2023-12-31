#[cfg(test)]
mod tests {
    use QRender::matrix::vector2d::Vector2d;

    #[test]
    fn vector2d_equal() {
        let v1 = Vector2d::new(1.0, 2.0);
        let v2 = Vector2d::new(1.0, 2.0);
        assert_eq!(v1, v2);
        let v = Vector2d::add_item(v1, 1.2);
        assert_ne!(v, v1);
    }

    #[test]
    fn vector2d_dot() {
        let v1 = Vector2d::new(1, 2);
        let v2 = Vector2d::new(3, 4);
        let res = v1 * v2;
        assert_eq!(res, 11);
    }

    #[test]
    fn vector2d_add() {
        let v1 = Vector2d::identity(1);
        let v = v1 + v1;
        assert_eq!(v.x, 2);
        assert_eq!(v.y, 2);
    }

    #[test]
    fn vector2d_sub() {
        let v1 = Vector2d::identity(1);
        let v = v1 - v1;
        assert_eq!(v.x, 0);
        assert_eq!(v.y, 0);
    }

    #[test]
    fn vector2d_mul() {
        let v1 = Vector2d::identity(2);
        let v = v1.mul(v1);
        assert_eq!(v.x, 4);
        assert_eq!(v.y, 4);
    }

    #[test]
    fn vector2d_div() {
        let v1 = Vector2d::identity(2);
        let v = v1 / v1.clone();
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 1);
    }

    #[test]
    fn vector2d_add_item() {
        let v1 = Vector2d::identity(2);
        let v = Vector2d::add_item(v1, 1);
        assert_eq!(v.x, 3);
        assert_eq!(v.y, 3);
    }

    #[test]
    fn vector2d_sub_item() {
        let v1 = Vector2d::identity(2);
        let v = Vector2d::sub_item(v1, 1);
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 1);
    }

    #[test]
    fn vector2d_mul_item() {
        let v1 = Vector2d::identity(2);
        let v = Vector2d::mul_item(v1, 5);
        assert_eq!(v.x, 10);
        assert_eq!(v.y, 10);
    }

    #[test]
    fn vector2d_div_item() {
        let v1 = Vector2d::identity(4);
        let v = Vector2d::div_item(v1, 2);
        assert_eq!(v.x, 2);
        assert_eq!(v.y, 2);
    }

    #[test]
    fn vector2d_normalized() {
        let mut v = Vector2d::new(3.0_f32, 5.0_f32);
        v.normalized();
        assert_eq!(v.x, 0.08823529411764706);
        assert_eq!(v.y, 0.14705882352941177);
    }
}