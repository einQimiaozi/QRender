mod matrix;

use matrix::{vector2d::Vector2d, vector3d::Vector3d, vector4d::Vector4d, matrix2d::Matrix2d, matrix3d::Matrix3d, matrix4d::Matrix4d};

fn main() {
    let v1 = Vector2d::new(1, 2);
    let v2 = Vector2d::new(3, 4);
    let v = v1 + v2;
    println!("{:?}", v);
}
