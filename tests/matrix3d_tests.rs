#[cfg(test)]
mod tests {
    use QRender::matrix::matrix3d::Matrix3d;
    use QRender::matrix::vector3d::Vector3d;

    #[test]
    fn matrix3d_transpose() {
        let mut m = Matrix3d::new(Vector3d::new(2, 1, 0), Vector3d::new(3, 4, 5), Vector3d::new(6, 7, 8));
        m.transpose();
        assert_eq!(m.items[0].x, 2);
        assert_eq!(m.items[0].y, 3);
        assert_eq!(m.items[0].z, 6);

        assert_eq!(m.items[1].x, 1);
        assert_eq!(m.items[1].y, 4);
        assert_eq!(m.items[1].z, 7);

        assert_eq!(m.items[2].x, 0);
        assert_eq!(m.items[2].y, 5);
        assert_eq!(m.items[2].z, 8);
    }

    #[test]
    fn matrix3d_product() {
        let m1 = Matrix3d::new(Vector3d::new(2, 1, 0), Vector3d::new(3, 4, 5), Vector3d::new(6, 7, 8));
        let m2 = Matrix3d::new(Vector3d::new(3, 0, 4), Vector3d::new(7, 1, 3), Vector3d::new(9, 2, 5));
        let m = m1 * m2;
        assert_eq!(m.items[0].x, 13);
        assert_eq!(m.items[0].y, 1);
        assert_eq!(m.items[0].z, 11);

        assert_eq!(m.items[1].x, 82);
        assert_eq!(m.items[1].y, 14);
        assert_eq!(m.items[1].z, 49);

        assert_eq!(m.items[2].x, 139);
        assert_eq!(m.items[2].y, 23);
        assert_eq!(m.items[2].z, 85);
    }

    #[test]
    fn matrix3d_product_with_vector3d() {
        let m = Matrix3d::new(Vector3d::new(1, 2, 5), Vector3d::new(3, 4, 7), Vector3d::new(0, 6, 3));
        let v1 = Vector3d::new(2, 3, 5);
        let v = m.product_with_vector3d(v1);
        assert_eq!(v.x, 33);
        assert_eq!(v.y, 53);
        assert_eq!(v.z, 33);
    }

    #[test]
    fn matrix3d_add() {
        let m1 = Matrix3d::fill(1);
        let m = m1 + Matrix3d::fill(1);
        assert_eq!(m.items[0].x, 2);
        assert_eq!(m.items[0].y, 2);
        assert_eq!(m.items[0].z, 2);

        assert_eq!(m.items[1].x, 2);
        assert_eq!(m.items[1].y, 2);
        assert_eq!(m.items[1].z, 2);

        assert_eq!(m.items[2].x, 2);
        assert_eq!(m.items[2].y, 2);
        assert_eq!(m.items[2].z, 2);
    }

    #[test]
    fn matrix3d_sub() {
        let m1 = Matrix3d::fill(1);
        let m = m1 - Matrix3d::fill(1);
        assert_eq!(m.items[0].x, 0);
        assert_eq!(m.items[0].y, 0);
        assert_eq!(m.items[0].z, 0);

        assert_eq!(m.items[1].x, 0);
        assert_eq!(m.items[1].y, 0);
        assert_eq!(m.items[1].z, 0);

        assert_eq!(m.items[2].x, 0);
        assert_eq!(m.items[2].y, 0);
        assert_eq!(m.items[2].z, 0);
    }

    #[test]
    fn matrix3d_div() {
        let m1 = Matrix3d::fill(10);
        let m = m1 / Matrix3d::fill(2);
        assert_eq!(m.items[0].x, 5);
        assert_eq!(m.items[0].y, 5);
        assert_eq!(m.items[0].z, 5);

        assert_eq!(m.items[1].x, 5);
        assert_eq!(m.items[1].y, 5);
        assert_eq!(m.items[1].z, 5);

        assert_eq!(m.items[2].x, 5);
        assert_eq!(m.items[2].y, 5);
        assert_eq!(m.items[2].z, 5);
    }

    #[test]
    fn matrix3d_hadamard() {
        let m1 = Matrix3d::fill(3);
        let m = m1.hadamard(Matrix3d::fill(4));
        assert_eq!(m.items[0].x, 12);
        assert_eq!(m.items[0].y, 12);
        assert_eq!(m.items[0].z, 12);

        assert_eq!(m.items[1].x, 12);
        assert_eq!(m.items[1].y, 12);
        assert_eq!(m.items[1].z, 12);

        assert_eq!(m.items[2].x, 12);
        assert_eq!(m.items[2].y, 12);
        assert_eq!(m.items[2].z, 12);
    }

    #[test]
    fn matrix3d_add_item() {
        let m1 = Matrix3d::fill(1);
        let m = m1.add_item(2);
        assert_eq!(m.items[0].x, 3);
        assert_eq!(m.items[0].y, 3);
        assert_eq!(m.items[0].z, 3);

        assert_eq!(m.items[1].x, 3);
        assert_eq!(m.items[1].y, 3);
        assert_eq!(m.items[1].z, 3);

        assert_eq!(m.items[2].x, 3);
        assert_eq!(m.items[2].y, 3);
        assert_eq!(m.items[2].z, 3);
    }

    #[test]
    fn matrix3d_div_item() {
        let m1 = Matrix3d::fill(10);
        let m = m1.div_item(2);
        assert_eq!(m.items[0].x, 5);
        assert_eq!(m.items[0].y, 5);
        assert_eq!(m.items[0].z, 5);

        assert_eq!(m.items[1].x, 5);
        assert_eq!(m.items[1].y, 5);
        assert_eq!(m.items[1].z, 5);

        assert_eq!(m.items[2].x, 5);
        assert_eq!(m.items[2].y, 5);
        assert_eq!(m.items[2].z, 5);
    }

    #[test]
    fn matrix3d_sub_item() {
        let m1 = Matrix3d::fill(1);
        let m = m1.sub_item(2);
        assert_eq!(m.items[0].x, -1);
        assert_eq!(m.items[0].y, -1);
        assert_eq!(m.items[0].z, -1);

        assert_eq!(m.items[1].x, -1);
        assert_eq!(m.items[1].y, -1);
        assert_eq!(m.items[1].z, -1);

        assert_eq!(m.items[2].x, -1);
        assert_eq!(m.items[2].y, -1);
        assert_eq!(m.items[2].z, -1);
    }

    #[test]
    fn matrix3d_mul_item() {
        let m1 = Matrix3d::fill(3);
        let m = m1.mul_item(2);
        assert_eq!(m.items[0].x, 6);
        assert_eq!(m.items[0].y, 6);
        assert_eq!(m.items[0].z, 6);

        assert_eq!(m.items[1].x, 6);
        assert_eq!(m.items[1].y, 6);
        assert_eq!(m.items[1].z, 6);

        assert_eq!(m.items[2].x, 6);
        assert_eq!(m.items[2].y, 6);
        assert_eq!(m.items[2].z, 6);
    }

    #[test]
    fn matrix3d_cross() {
        let m1 = Matrix3d::new(
            Vector3d::new(1, 2, 3),
            Vector3d::new(4, 5, 6),
            Vector3d::new(7, 8, 9)
        );
        let v = Vector3d::new(2, 3, 5);
        let m = m1.cross(v);
        assert_eq!(m.items[0], m1.items[0].cross(v));
        assert_eq!(m.items[1], m1.items[1].cross(v));
        assert_eq!(m.items[2], m1.items[2].cross(v));
    }
}