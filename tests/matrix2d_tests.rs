#[cfg(test)]
mod tests {
    use QRender::matrix::matrix2d::Matrix2d;
    use QRender::matrix::vector2d::Vector2d;

    #[test]
    fn matrix2d_transpose() {
        let mut m = Matrix2d::new(Vector2d::new(2, 1), Vector2d::new(3, 4));
        m.transpose();
        assert_eq!(m.items[0].x, 2);
        assert_eq!(m.items[0].y, 3);
        assert_eq!(m.items[1].x, 1);
        assert_eq!(m.items[1].y, 4);
    }

    #[test]
    fn matrix2d_product() {
        let m1 = Matrix2d::new(Vector2d::new(2, 1), Vector2d::new(3, 4));
        let m2 = Matrix2d::new(Vector2d::new(5, 7), Vector2d::new(2, 1));
        let m = m1 * m2;
        assert_eq!(m.items[0].x, 12);
        assert_eq!(m.items[0].y, 15);
        assert_eq!(m.items[1].x, 23);
        assert_eq!(m.items[1].y, 25);
    }

    #[test]
    fn matrix2d_product_with_vector2d() {
        let m = Matrix2d::new(Vector2d::new(1, 2), Vector2d::new(3, 4));
        let v1 = Vector2d::new(2, 3);
        let v = m.product_with_vector2d(v1);
        assert_eq!(v.x, 8);
        assert_eq!(v.y, 18);
    }

    #[test]
    fn matrix2d_add() {
        let m1 = Matrix2d::identity(1);
        let m = m1 + Matrix2d::identity(1);
        assert_eq!(m.items[0].x, 2);
        assert_eq!(m.items[0].y, 2);
        assert_eq!(m.items[1].x, 2);
        assert_eq!(m.items[1].y, 2);
    }

    #[test]
    fn matrix2d_sub() {
        let m1 = Matrix2d::identity(1);
        let m = m1 - Matrix2d::identity(1);
        assert_eq!(m.items[0].x, 0);
        assert_eq!(m.items[0].y, 0);
        assert_eq!(m.items[1].x, 0);
        assert_eq!(m.items[1].y, 0);
    }

    #[test]
    fn matrix2d_div() {
        let m1 = Matrix2d::identity(10);
        let m = m1 / Matrix2d::identity(2);
        assert_eq!(m.items[0].x, 5);
        assert_eq!(m.items[0].y, 5);
        assert_eq!(m.items[1].x, 5);
        assert_eq!(m.items[1].y, 5);
    }

    #[test]
    fn matrix2d_hadamard() {
        let m1 = Matrix2d::identity(3);
        let m = m1.hadamard(Matrix2d::identity(4));
        assert_eq!(m.items[0].x, 12);
        assert_eq!(m.items[0].y, 12);
        assert_eq!(m.items[1].x, 12);
        assert_eq!(m.items[1].y, 12);
    }

    #[test]
    fn matrix2d_add_item() {
        let m1 = Matrix2d::identity(1);
        let m = m1.add_item(2);
        assert_eq!(m.items[0].x, 3);
        assert_eq!(m.items[0].y, 3);
        assert_eq!(m.items[1].x, 3);
        assert_eq!(m.items[1].y, 3);
    }

    #[test]
    fn matrix2d_div_item() {
        let m1 = Matrix2d::identity(10);
        let m = m1.div_item(2);
        assert_eq!(m.items[0].x, 5);
        assert_eq!(m.items[0].y, 5);
        assert_eq!(m.items[1].x, 5);
        assert_eq!(m.items[1].y, 5);
    }

    #[test]
    fn matrix2d_sub_item() {
        let m1 = Matrix2d::identity(1);
        let m = m1.sub_item(2);
        assert_eq!(m.items[0].x, -1);
        assert_eq!(m.items[0].y, -1);
        assert_eq!(m.items[1].x, -1);
        assert_eq!(m.items[1].y, -1);
    }

    #[test]
    fn matrix2d_mul_item() {
        let m1 = Matrix2d::identity(3);
        let m = m1.mul_item(2);
        assert_eq!(m.items[0].x, 6);
        assert_eq!(m.items[0].y, 6);
        assert_eq!(m.items[1].x, 6);
        assert_eq!(m.items[1].y, 6);
    }
}