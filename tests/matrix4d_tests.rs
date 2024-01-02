#![feature(test)]
#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use QRender::matrix::matrix4d::Matrix4d;
    use QRender::matrix::vector4d::Vector4d;

    #[test]
    fn matrix4d_transpose() {
        let mut m = Matrix4d::new(
            Vector4d::new(2, 1, 0, 3),
            Vector4d::new(3, 4, 5, 2),
            Vector4d::new(6, 7, 8, 9),
            Vector4d::new(4, 3, 2, 1)
        );
        m.transpose();
        assert_eq!(m.items[0].x, 2);
        assert_eq!(m.items[0].y, 3);
        assert_eq!(m.items[0].z, 6);
        assert_eq!(m.items[0].w, 4);

        assert_eq!(m.items[1].x, 1);
        assert_eq!(m.items[1].y, 4);
        assert_eq!(m.items[1].z, 7);
        assert_eq!(m.items[1].w, 3);

        assert_eq!(m.items[2].x, 0);
        assert_eq!(m.items[2].y, 5);
        assert_eq!(m.items[2].z, 8);
        assert_eq!(m.items[2].w, 2);

        assert_eq!(m.items[3].x, 3);
        assert_eq!(m.items[3].y, 2);
        assert_eq!(m.items[3].z, 9);
        assert_eq!(m.items[3].w, 1);
    }

    #[test]
    fn matrix4d_product() {
        let m1 = Matrix4d::new(
            Vector4d::new(2, 1, 0, 3),
            Vector4d::new(3, 4, 5, 2),
            Vector4d::new(6, 7, 8, 9),
            Vector4d::new(4, 3, 2, 1)
        );
        let m2 = Matrix4d::new(
            Vector4d::new(3, 0, 4, 9),
            Vector4d::new(7, 1, 3, 8),
            Vector4d::new(9, 2, 5, 4),
            Vector4d::new(4, 7, 6, 0)
        );
        let m = m1 * m2;
        assert_eq!(m.items[0].x, 25);
        assert_eq!(m.items[0].y, 22);
        assert_eq!(m.items[0].z, 29);
        assert_eq!(m.items[0].w, 26);


        assert_eq!(m.items[1].x, 90);
        assert_eq!(m.items[1].y, 28);
        assert_eq!(m.items[1].z, 61);
        assert_eq!(m.items[1].w, 79);

        assert_eq!(m.items[2].x, 175);
        assert_eq!(m.items[2].y, 86);
        assert_eq!(m.items[2].z, 139);
        assert_eq!(m.items[2].w, 142);

        assert_eq!(m.items[3].x, 55);
        assert_eq!(m.items[3].y, 14);
        assert_eq!(m.items[3].z, 41);
        assert_eq!(m.items[3].w, 68);
    }

    #[bench]
    fn bench_matrix4d_product(b: &mut Bencher) {
        let m1 = Matrix4d::new(
            Vector4d::new(2, 1, 0, 3),
            Vector4d::new(3, 4, 5, 2),
            Vector4d::new(6, 7, 8, 9),
            Vector4d::new(4, 3, 2, 1)
        );
        let m2 = Matrix4d::new(
            Vector4d::new(3, 0, 4, 9),
            Vector4d::new(7, 1, 3, 8),
            Vector4d::new(9, 2, 5, 4),
            Vector4d::new(4, 7, 6, 0)
        );
        b.iter(|| m1.product(m2));
    }

    #[test]
    fn matrix4d_product_with_vector4d() {
        let m = Matrix4d::new(
            Vector4d::new(1, 2, 5, 9),
            Vector4d::new(3, 4, 7, 8),
            Vector4d::new(0, 6, 3, 6),
            Vector4d::new(4, 3, 2, 1)
        );
        let v1 = Vector4d::new(2, 3, 5, 1);
        let v = m.product_with_vector4d(v1);
        assert_eq!(v.x, 42);
        assert_eq!(v.y, 61);
        assert_eq!(v.z, 39);
        assert_eq!(v.w, 28);
    }

    #[test]
    fn matrix4d_add() {
        let m1 = Matrix4d::fill(1);
        let m = m1 + Matrix4d::fill(1);
        assert_eq!(m.items[0].x, 2);
        assert_eq!(m.items[0].y, 2);
        assert_eq!(m.items[0].z, 2);
        assert_eq!(m.items[0].w, 2);

        assert_eq!(m.items[1].x, 2);
        assert_eq!(m.items[1].y, 2);
        assert_eq!(m.items[1].z, 2);
        assert_eq!(m.items[1].w, 2);

        assert_eq!(m.items[2].x, 2);
        assert_eq!(m.items[2].y, 2);
        assert_eq!(m.items[2].z, 2);
        assert_eq!(m.items[2].w, 2);

        assert_eq!(m.items[3].x, 2);
        assert_eq!(m.items[3].y, 2);
        assert_eq!(m.items[3].z, 2);
        assert_eq!(m.items[3].w, 2);
    }

    #[test]
    fn matrix4d_sub() {
        let m1 = Matrix4d::fill(1);
        let m = m1 - Matrix4d::fill(1);
        assert_eq!(m.items[0].x, 0);
        assert_eq!(m.items[0].y, 0);
        assert_eq!(m.items[0].z, 0);
        assert_eq!(m.items[0].w, 0);

        assert_eq!(m.items[1].x, 0);
        assert_eq!(m.items[1].y, 0);
        assert_eq!(m.items[1].z, 0);
        assert_eq!(m.items[1].w, 0);

        assert_eq!(m.items[2].x, 0);
        assert_eq!(m.items[2].y, 0);
        assert_eq!(m.items[2].z, 0);
        assert_eq!(m.items[2].w, 0);

        assert_eq!(m.items[3].x, 0);
        assert_eq!(m.items[3].y, 0);
        assert_eq!(m.items[3].z, 0);
        assert_eq!(m.items[3].w, 0);
    }

    #[test]
    fn matrix4d_div() {
        let m1 = Matrix4d::fill(10);
        let m = m1 / Matrix4d::fill(2);
        assert_eq!(m.items[0].x, 5);
        assert_eq!(m.items[0].y, 5);
        assert_eq!(m.items[0].z, 5);
        assert_eq!(m.items[0].w, 5);

        assert_eq!(m.items[1].x, 5);
        assert_eq!(m.items[1].y, 5);
        assert_eq!(m.items[1].z, 5);
        assert_eq!(m.items[1].w, 5);

        assert_eq!(m.items[2].x, 5);
        assert_eq!(m.items[2].y, 5);
        assert_eq!(m.items[2].z, 5);
        assert_eq!(m.items[2].w, 5);

        assert_eq!(m.items[3].x, 5);
        assert_eq!(m.items[3].y, 5);
        assert_eq!(m.items[3].z, 5);
        assert_eq!(m.items[3].w, 5);
    }

    #[test]
    fn matrix4d_hadamard() {
        let m1 = Matrix4d::fill(3);
        let m = m1.hadamard(Matrix4d::fill(4));
        assert_eq!(m.items[0].x, 12);
        assert_eq!(m.items[0].y, 12);
        assert_eq!(m.items[0].z, 12);
        assert_eq!(m.items[0].w, 12);

        assert_eq!(m.items[1].x, 12);
        assert_eq!(m.items[1].y, 12);
        assert_eq!(m.items[1].z, 12);
        assert_eq!(m.items[1].w, 12);

        assert_eq!(m.items[2].x, 12);
        assert_eq!(m.items[2].y, 12);
        assert_eq!(m.items[2].z, 12);
        assert_eq!(m.items[2].w, 12);

        assert_eq!(m.items[3].x, 12);
        assert_eq!(m.items[3].y, 12);
        assert_eq!(m.items[3].z, 12);
        assert_eq!(m.items[3].w, 12);
    }

    #[test]
    fn matrix4d_add_item() {
        let m1 = Matrix4d::fill(1);
        let m = m1.add_item(2);
        assert_eq!(m.items[0].x, 3);
        assert_eq!(m.items[0].y, 3);
        assert_eq!(m.items[0].z, 3);
        assert_eq!(m.items[0].w, 3);

        assert_eq!(m.items[1].x, 3);
        assert_eq!(m.items[1].y, 3);
        assert_eq!(m.items[1].z, 3);
        assert_eq!(m.items[1].w, 3);

        assert_eq!(m.items[2].x, 3);
        assert_eq!(m.items[2].y, 3);
        assert_eq!(m.items[2].z, 3);
        assert_eq!(m.items[2].w, 3);

        assert_eq!(m.items[3].x, 3);
        assert_eq!(m.items[3].y, 3);
        assert_eq!(m.items[3].z, 3);
        assert_eq!(m.items[3].w, 3);
    }

    #[test]
    fn matrix4d_div_item() {
        let m1 = Matrix4d::fill(10);
        let m = m1.div_item(2);
        assert_eq!(m.items[0].x, 5);
        assert_eq!(m.items[0].y, 5);
        assert_eq!(m.items[0].z, 5);
        assert_eq!(m.items[0].w, 5);

        assert_eq!(m.items[1].x, 5);
        assert_eq!(m.items[1].y, 5);
        assert_eq!(m.items[1].z, 5);
        assert_eq!(m.items[1].w, 5);

        assert_eq!(m.items[2].x, 5);
        assert_eq!(m.items[2].y, 5);
        assert_eq!(m.items[2].z, 5);
        assert_eq!(m.items[2].w, 5);

        assert_eq!(m.items[3].x, 5);
        assert_eq!(m.items[3].y, 5);
        assert_eq!(m.items[3].z, 5);
        assert_eq!(m.items[3].w, 5);
    }

    #[test]
    fn matrix4d_sub_item() {
        let m1 = Matrix4d::fill(1);
        let m = m1.sub_item(2);
        assert_eq!(m.items[0].x, -1);
        assert_eq!(m.items[0].y, -1);
        assert_eq!(m.items[0].z, -1);
        assert_eq!(m.items[0].w, -1);

        assert_eq!(m.items[1].x, -1);
        assert_eq!(m.items[1].y, -1);
        assert_eq!(m.items[1].z, -1);
        assert_eq!(m.items[1].w, -1);

        assert_eq!(m.items[2].x, -1);
        assert_eq!(m.items[2].y, -1);
        assert_eq!(m.items[2].z, -1);
        assert_eq!(m.items[2].w, -1);

        assert_eq!(m.items[3].x, -1);
        assert_eq!(m.items[3].y, -1);
        assert_eq!(m.items[3].z, -1);
        assert_eq!(m.items[3].w, -1);
    }

    #[test]
    fn matrix4d_mul_item() {
        let m1 = Matrix4d::fill(3);
        let m = m1.mul_item(2);
        assert_eq!(m.items[0].x, 6);
        assert_eq!(m.items[0].y, 6);
        assert_eq!(m.items[0].z, 6);
        assert_eq!(m.items[0].w, 6);

        assert_eq!(m.items[1].x, 6);
        assert_eq!(m.items[1].y, 6);
        assert_eq!(m.items[1].z, 6);
        assert_eq!(m.items[1].w, 6);

        assert_eq!(m.items[2].x, 6);
        assert_eq!(m.items[2].y, 6);
        assert_eq!(m.items[2].z, 6);
        assert_eq!(m.items[2].w, 6);

        assert_eq!(m.items[3].x, 6);
        assert_eq!(m.items[3].y, 6);
        assert_eq!(m.items[3].z, 6);
        assert_eq!(m.items[3].w, 6);
    }

    #[test]
    fn matrix4d_identity() {
        let m = Matrix4d::identity(1.0_f32);
        let m2 = Matrix4d::new(
            Vector4d::new(1.0_f32, 0.0_f32, 0.0_f32, 0.0_f32),
            Vector4d::new(0.0_f32, 1.0_f32, 0.0_f32, 0.0_f32),
            Vector4d::new(0.0_f32, 0.0_f32, 1.0_f32, 0.0_f32),
            Vector4d::new(0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32),
        );
        assert_eq!(m, m2);
    }
}