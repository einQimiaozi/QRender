use std::{fmt, ops};
use crate::matrix::vector3d::Vector3d;

#[derive(Debug, Clone)]
pub struct Matrix3d<T>
    where T:
    fmt::Debug +
    Copy +
    PartialOrd +
    PartialEq +
    ops::Add<Output=T> +
    ops::Mul<Output=T> +
    ops::Div<Output=T> +
    ops::Sub<Output=T>
{
    pub items: Vec<Vector3d<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Matrix3d<T>
    where T:
    fmt::Debug +
    Copy +
    PartialOrd +
    PartialEq +
    ops::Add<Output=T> +
    ops::Mul<Output=T> +
    ops::Div<Output=T> +
    ops::Sub<Output=T>
{
    pub fn new(v1: Vector3d<T>, v2: Vector3d<T>, v3: Vector3d<T>) -> Matrix3d<T> {
        Matrix3d {
            items: vec![v1, v2, v3],
            rows: 3,
            cols: 3,
        }
    }
    pub fn identity(item_type: T) -> Matrix3d<T> {
        let mut items = Vec::with_capacity(3);
        items.resize(3, Vector3d::new(item_type, item_type, item_type));
        Matrix3d {
            items,
            rows: 3,
            cols: 3,
        }
    }
    pub fn transpose(&mut self) {
        let tmp = self.items[0].y;
        self.items[0].y = self.items[1].x;
        self.items[1].x = tmp;

        let tmp = self.items[0].z;
        self.items[0].z = self.items[2].x;
        self.items[2].x = tmp;

        let tmp = self.items[1].z;
        self.items[1].z = self.items[2].y;
        self.items[2].y = tmp;
    }
    pub fn product(&mut self, mat: Matrix3d<T>) -> Matrix3d<T> {
        let mut res = Self::identity(mat.items[0].x - mat.items[0].x);
        let mut mat1 = self.clone();
        let mut mat2 = mat.clone();

        res.items[0].x = mat1.items[0].dot(mat2.items[0].clone());
        res.items[0].y = mat1.items[0].dot(mat2.items[1].clone());
        res.items[0].z = mat1.items[0].dot(mat2.items[2].clone());

        res.items[1].x = mat1.items[1].dot(mat2.items[0].clone());
        res.items[1].y = mat1.items[1].dot(mat2.items[1].clone());
        res.items[1].z = mat1.items[1].dot(mat2.items[2].clone());

        res.items[2].x = mat1.items[2].dot(mat2.items[0].clone());
        res.items[2].y = mat1.items[2].dot(mat2.items[1].clone());
        res.items[2].z = mat1.items[2].dot(mat2.items[2].clone());

        res
    }
    pub fn product_with_vector3d(&mut self, v: Vector3d<T>) -> Vector3d<T> {
        let mut res = Vector3d::identity(self.items[0].x - self.items[0].x);
        res.x = self.items[0].dot(v.clone());
        res.y = self.items[1].dot(v.clone());
        res.z = self.items[2].dot(v.clone());
        res
    }
    pub fn add(&mut self, mat: Matrix3d<T>) -> Matrix3d<T> {
        Matrix3d {
            items: vec![self.items[0].add(mat.items[0].clone()),
                        self.items[1].add(mat.items[1].clone()),
                        self.items[2].add(mat.items[2].clone())],
            rows: 3,
            cols: 3,
        }
    }
    pub fn hadamard(&mut self, mat: Matrix3d<T>) -> Matrix3d<T>{
        Matrix3d {
            items: vec![self.items[0].mul(mat.items[0].clone()),
                        self.items[1].mul(mat.items[1].clone()),
                        self.items[2].mul(mat.items[2].clone())],
            rows: 3,
            cols: 3,
        }
    }

    pub fn sub(&mut self, mat: Matrix3d<T>) -> Matrix3d<T> {
        Matrix3d {
            items: vec![self.items[0].sub(mat.items[0].clone()),
                        self.items[1].sub(mat.items[1].clone()),
                        self.items[2].sub(mat.items[2].clone())],
            rows: 3,
            cols: 3,
        }
    }

    pub fn div(&mut self, mat: Matrix3d<T>) -> Matrix3d<T> {
        Matrix3d {
            items: vec![self.items[0].div(mat.items[0].clone()),
                        self.items[1].div(mat.items[1].clone()),
                        self.items[2].div(mat.items[2].clone())],
            rows: 3,
            cols: 3,
        }
    }

    pub fn add_item(&mut self, item: T) -> Matrix3d<T> {
        Matrix3d {
            items: vec![
                Vector3d::new(self.items[0].x + item, self.items[0].y + item, self.items[0].z + item),
                Vector3d::new(self.items[1].x + item, self.items[1].y + item, self.items[1].z + item),
                Vector3d::new(self.items[2].x + item, self.items[2].y + item, self.items[2].z + item)
            ],
            rows: 3,
            cols: 3,
        }
    }
    pub fn sub_item(&mut self, item: T) -> Matrix3d<T> {
        Matrix3d {
            items: vec![
                Vector3d::new(self.items[0].x - item, self.items[0].y - item, self.items[0].z - item),
                Vector3d::new(self.items[1].x - item, self.items[1].y - item, self.items[1].z - item),
                Vector3d::new(self.items[2].x - item, self.items[2].y - item, self.items[2].z - item)
            ],
            rows: 3,
            cols: 3,
        }
    }
    pub fn mul_item(&mut self, item: T) -> Matrix3d<T> {
        Matrix3d {
            items: vec![
                Vector3d::new(self.items[0].x * item, self.items[0].y * item, self.items[0].z * item),
                Vector3d::new(self.items[1].x * item, self.items[1].y * item, self.items[1].z * item),
                Vector3d::new(self.items[2].x * item, self.items[2].y * item, self.items[2].z * item)
            ],
            rows: 3,
            cols: 3,
        }
    }
    pub fn div_item(&mut self, item: T) -> Matrix3d<T> {
        Matrix3d {
            items: vec![
                Vector3d::new(self.items[0].x / item, self.items[0].y / item, self.items[0].z / item),
                Vector3d::new(self.items[1].x / item, self.items[1].y / item, self.items[1].z / item),
                Vector3d::new(self.items[2].x / item, self.items[2].y / item, self.items[2].z / item)
            ],
            rows: 3,
            cols: 3,
        }
    }
}

impl<T> fmt::Display for Matrix3d<T>
    where T:
    fmt::Debug +
    Copy +
    PartialOrd +
    PartialEq +
    ops::Add<Output=T> +
    ops::Mul<Output=T> +
    ops::Div<Output=T> +
    ops::Sub<Output=T>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "[{:?} {:?} {:?}]\n[{:?} {:?} {:?}]\n[{:?} {:?} {:?}]\n",
            self.items[0].x, self.items[0].y, self.items[0].z,
            self.items[1].x, self.items[1].y, self.items[1].z,
            self.items[2].x, self.items[2].y, self.items[2].z
        )
    }
}