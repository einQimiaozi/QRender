#[cfg(test)]
mod tests {
    use QRender::matrix3d::Matrix3d;
    use QRender::matrix::vector3d::Vector3d;

    #[test]
    fn vector3d_dot() {
        let v1 = Vector3d::new(1, 2, 3);
        let v2 = Vector3d::new(4, 5, 6);
        let res = v1 * v2;
        assert_eq!(res, 32);
    }

    #[test]
    fn vector3d_add() {
        let v1 = Vector3d::identity(1);
        let v = v1 + v1;
        assert_eq!(v.x, 2);
        assert_eq!(v.y, 2);
        assert_eq!(v.z, 2);
    }

    #[test]
    fn vector3d_sub() {
        let v1 = Vector3d::identity(1);
        let v = v1 - v1;
        assert_eq!(v.x, 0);
        assert_eq!(v.y, 0);
        assert_eq!(v.z, 0);
    }

    #[test]
    fn vector3d_mul() {
        let v1 = Vector3d::identity(2);
        let v = v1.mul(v1);
        assert_eq!(v.x, 4);
        assert_eq!(v.y, 4);
        assert_eq!(v.z, 4);
    }

    #[test]
    fn vector3d_div() {
        let v1 = Vector3d::identity(2);
        let v = v1 / v1;
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

    #[test]
    fn vector3d_cross() {
        let v1 = Vector3d::new(1, 2, 3);
        let v2 = Vector3d::new(4, 5, 6);
        let v = v1.cross(v2);
        assert_eq!(v.x, -3);
        assert_eq!(v.y, 6);
        assert_eq!(v.z, -3);
    }

    #[test]
    fn vector3d_normalized() {
        let mut v = Vector3d::new(3.0_f32, 5.0_f32, 2.0_f32);
        v.normalized();
        assert_eq!(v.x, 0.07894736842105263);
        assert_eq!(v.y, 0.13157894736842105);
        assert_eq!(v.z, 0.05263157894736842);
    }

    #[test]
    fn vector3d_product_with_matrix3d() {
        let v1 = Vector3d::new(3, 5, 6);
        let m = Matrix3d::new(Vector3d::new(1, 2, 3), Vector3d::new(4, 5, 6), Vector3d::new(7, 8, 9));
        let v = v1.product_with_matrix3d(m);
        assert_eq!(v.x, 65);
        assert_eq!(v.y, 79);
        assert_eq!(v.z, 93);
    }
}