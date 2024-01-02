#[cfg(test)]
mod tests {
    use QRender::matrix::matrix4d::Matrix4d;
    use QRender::matrix::vector4d::Vector4d;

    #[test]
    fn vector4d_dot() {
        let v1 = Vector4d::new(1, 2, 3, 4);
        let v2 = Vector4d::new(5, 6, 7, 8);
        let res = v1 * v2;
        assert_eq!(res, 70);
    }

    #[test]
    fn vector4d_add() {
        let v1 = Vector4d::fill(1);
        let v = v1 + v1;
        assert_eq!(v.x, 2);
        assert_eq!(v.y, 2);
        assert_eq!(v.z, 2);
        assert_eq!(v.w, 2);
    }

    #[test]
    fn vector4d_sub() {
        let v1 = Vector4d::fill(1);
        let v = v1 - v1;
        assert_eq!(v.x, 0);
        assert_eq!(v.y, 0);
        assert_eq!(v.z, 0);
        assert_eq!(v.w, 0);
    }

    #[test]
    fn vector4d_mul() {
        let v1 = Vector4d::fill(2);
        let v = v1.mul(v1);
        assert_eq!(v.x, 4);
        assert_eq!(v.y, 4);
        assert_eq!(v.z, 4);
        assert_eq!(v.w, 4);
    }

    #[test]
    fn vector4d_div() {
        let v1 = Vector4d::fill(2);
        let v = v1 / v1;
        assert_eq!(v.x, 1);
        assert_eq!(v.y, 1);
        assert_eq!(v.z, 1);
        assert_eq!(v.w, 1);
    }

    #[test]
    fn vector4d_add_item() {
        let v1 = Vector4d::fill(4);
        let v = Vector4d::add_item(v1, 2);
        assert_eq!(v.x, 6);
        assert_eq!(v.y, 6);
        assert_eq!(v.z, 6);
        assert_eq!(v.w, 6);
    }

    #[test]
    fn vector4d_mul_item() {
        let v1 = Vector4d::fill(4);
        let v = Vector4d::mul_item(v1, 2);
        assert_eq!(v.x, 8);
        assert_eq!(v.y, 8);
        assert_eq!(v.z, 8);
        assert_eq!(v.w, 8);
    }

    #[test]
    fn vector4d_sub_item() {
        let v1 = Vector4d::fill(4);
        let v = Vector4d::sub_item(v1, 2);
        assert_eq!(v.x, 2);
        assert_eq!(v.y, 2);
        assert_eq!(v.z, 2);
        assert_eq!(v.w, 2);
    }

    #[test]
    fn vector4d_div_item() {
        let v1 = Vector4d::fill(4);
        let v = Vector4d::div_item(v1, 2);
        assert_eq!(v.x, 2);
        assert_eq!(v.y, 2);
        assert_eq!(v.z, 2);
        assert_eq!(v.w, 2);
    }

    #[test]
    fn vector4d_normalized() {
        let mut v = Vector4d::new(3.0_f32, 5.0_f32, 2.0_f32, 6.0_f32);
        v.normalized();
        assert_eq!(v.x, 0.04054054054054054);
        assert_eq!(v.y, 0.06756756756756757);
        assert_eq!(v.z, 0.02702702702702703);
        assert_eq!(v.w, 0.08108108108108109);
    }

    #[test]
    fn vector4d_product_with_matrix4d() {
        let v1 = Vector4d::new(3, 5, 6, 1);
        let m = Matrix4d::new(
            Vector4d::new(1, 2, 3, 4),
            Vector4d::new(5, 6, 7, 8),
            Vector4d::new(9, 0, 1, 2),
            Vector4d::new(3, 4, 5, 6)
        );
        let v = v1.product_with_matrix4d(m);
        assert_eq!(v.x, 85);
        assert_eq!(v.y, 40);
        assert_eq!(v.z, 55);
        assert_eq!(v.w, 70);
    }
}