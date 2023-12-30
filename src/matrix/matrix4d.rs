use std::{fmt, ops};
use crate::matrix::vector4d::Vector4d;

#[derive(Debug, Clone)]
pub struct Matrix4d<T>
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
    pub items: Vec<Vector4d<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Matrix4d<T>
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
    pub fn new(v1: Vector4d<T>, v2: Vector4d<T>, v3: Vector4d<T>, v4: Vector4d<T>) -> Matrix4d<T> {
        Matrix4d {
            items: vec![v1, v2, v3, v4],
            rows: 4,
            cols: 4,
        }
    }
    pub fn identity(item_type: T) -> Matrix4d<T> {
        let mut items = Vec::with_capacity(4);
        items.resize(4, Vector4d::new(item_type, item_type, item_type, item_type));
        Matrix4d {
            items,
            rows: 4,
            cols: 4,
        }
    }
    pub fn transpose(&mut self) {
        let tmp = self.items[0].y;
        self.items[0].y = self.items[1].x;
        self.items[1].x = tmp;

        let tmp = self.items[0].z;
        self.items[0].z = self.items[2].x;
        self.items[2].x = tmp;

        let tmp = self.items[0].w;
        self.items[0].w = self.items[3].x;
        self.items[3].x = tmp;

        let tmp = self.items[1].z;
        self.items[1].z = self.items[2].y;
        self.items[2].y = tmp;

        let tmp = self.items[1].w;
        self.items[1].w = self.items[3].y;
        self.items[3].y = tmp;

        let tmp = self.items[2].w;
        self.items[2].w = self.items[3].z;
        self.items[3].z = tmp;
    }
    pub fn product(&self, mat: Matrix4d<T>) -> Matrix4d<T> {
        let mut res = Self::identity(mat.items[0].x - mat.items[0].x);
        let mut mat1 = self.clone();
        let mut mat2 = Matrix4d::new(
            Vector4d::new(mat.items[0].x.clone(), mat.items[1].x.clone(), mat.items[2].x.clone(), mat.items[3].x.clone()),
            Vector4d::new(mat.items[0].y.clone(), mat.items[1].y.clone(), mat.items[2].y.clone(), mat.items[3].y.clone()),
            Vector4d::new(mat.items[0].z.clone(), mat.items[1].z.clone(), mat.items[2].z.clone(), mat.items[3].z.clone()),
            Vector4d::new(mat.items[0].w.clone(), mat.items[1].w.clone(), mat.items[2].w.clone(), mat.items[3].w.clone())
        );

        res.items[0].x = mat1.items[0].dot(mat2.items[0].clone());
        res.items[0].y = mat1.items[0].dot(mat2.items[1].clone());
        res.items[0].z = mat1.items[0].dot(mat2.items[2].clone());
        res.items[0].w = mat1.items[0].dot(mat2.items[3].clone());

        res.items[1].x = mat1.items[1].dot(mat2.items[0].clone());
        res.items[1].y = mat1.items[1].dot(mat2.items[1].clone());
        res.items[1].z = mat1.items[1].dot(mat2.items[2].clone());
        res.items[1].w = mat1.items[1].dot(mat2.items[3].clone());

        res.items[2].x = mat1.items[2].dot(mat2.items[0].clone());
        res.items[2].y = mat1.items[2].dot(mat2.items[1].clone());
        res.items[2].z = mat1.items[2].dot(mat2.items[2].clone());
        res.items[2].w = mat1.items[2].dot(mat2.items[3].clone());

        res.items[3].x = mat1.items[3].dot(mat2.items[0].clone());
        res.items[3].y = mat1.items[3].dot(mat2.items[1].clone());
        res.items[3].z = mat1.items[3].dot(mat2.items[2].clone());
        res.items[3].w = mat1.items[3].dot(mat2.items[3].clone());

        res
    }
    pub fn product_with_vector4d(&self, v: Vector4d<T>) -> Vector4d<T> {
        let mut res = Vector4d::identity(self.items[0].x - self.items[0].x);
        res.x = self.items[0].dot(v.clone());
        res.y = self.items[1].dot(v.clone());
        res.z = self.items[2].dot(v.clone());
        res.w = self.items[3].dot(v.clone());
        res
    }
    pub fn add(&self, mat: Matrix4d<T>) -> Matrix4d<T> {
        Matrix4d {
            items: vec![self.items[0].add(mat.items[0].clone()),
                        self.items[1].add(mat.items[1].clone()),
                        self.items[2].add(mat.items[2].clone()),
                        self.items[3].add(mat.items[3].clone())],
            rows: 4,
            cols: 4,
        }
    }
    pub fn hadamard(&self, mat: Matrix4d<T>) -> Matrix4d<T>{
        Matrix4d {
            items: vec![self.items[0].mul(mat.items[0].clone()),
                        self.items[1].mul(mat.items[1].clone()),
                        self.items[2].mul(mat.items[2].clone()),
                        self.items[3].mul(mat.items[3].clone())],
            rows: 4,
            cols: 4,
        }
    }

    pub fn sub(&self, mat: Matrix4d<T>) -> Matrix4d<T> {
        Matrix4d {
            items: vec![self.items[0].sub(mat.items[0].clone()),
                        self.items[1].sub(mat.items[1].clone()),
                        self.items[2].sub(mat.items[2].clone()),
                        self.items[3].sub(mat.items[3].clone())],
            rows: 4,
            cols: 4,
        }
    }

    pub fn div(&self, mat: Matrix4d<T>) -> Matrix4d<T> {
        Matrix4d {
            items: vec![self.items[0].div(mat.items[0].clone()),
                        self.items[1].div(mat.items[1].clone()),
                        self.items[2].div(mat.items[2].clone()),
                        self.items[3].div(mat.items[3].clone())],
            rows: 4,
            cols: 4,
        }
    }

    pub fn add_item(&self, item: T) -> Matrix4d<T> {
        Matrix4d {
            items: vec![
                Vector4d::new(self.items[0].x + item, self.items[0].y + item, self.items[0].z + item, self.items[0].w + item),
                Vector4d::new(self.items[1].x + item, self.items[1].y + item, self.items[1].z + item, self.items[1].w + item),
                Vector4d::new(self.items[2].x + item, self.items[2].y + item, self.items[2].z + item, self.items[2].w + item),
                Vector4d::new(self.items[3].x + item, self.items[3].y + item, self.items[3].z + item, self.items[3].w + item),
            ],
            rows: 4,
            cols: 4,
        }
    }
    pub fn sub_item(&self, item: T) -> Matrix4d<T> {
        Matrix4d {
            items: vec![
                Vector4d::new(self.items[0].x - item, self.items[0].y - item, self.items[0].z - item, self.items[0].w - item),
                Vector4d::new(self.items[1].x - item, self.items[1].y - item, self.items[1].z - item, self.items[1].w - item),
                Vector4d::new(self.items[2].x - item, self.items[2].y - item, self.items[2].z - item, self.items[2].w - item),
                Vector4d::new(self.items[3].x - item, self.items[3].y - item, self.items[3].z - item, self.items[3].w - item),
            ],
            rows: 4,
            cols: 4,
        }
    }
    pub fn mul_item(&self, item: T) -> Matrix4d<T> {
        Matrix4d {
            items: vec![
                Vector4d::new(self.items[0].x * item, self.items[0].y * item, self.items[0].z * item, self.items[0].w * item),
                Vector4d::new(self.items[1].x * item, self.items[1].y * item, self.items[1].z * item, self.items[1].w * item),
                Vector4d::new(self.items[2].x * item, self.items[2].y * item, self.items[2].z * item, self.items[2].w * item),
                Vector4d::new(self.items[3].x * item, self.items[3].y * item, self.items[3].z * item, self.items[3].w * item),
            ],
            rows: 4,
            cols: 4,
        }
    }
    pub fn div_item(&self, item: T) -> Matrix4d<T> {
        Matrix4d {
            items: vec![
                Vector4d::new(self.items[0].x / item, self.items[0].y / item, self.items[0].z / item, self.items[0].w / item),
                Vector4d::new(self.items[1].x / item, self.items[1].y / item, self.items[1].z / item, self.items[1].w / item),
                Vector4d::new(self.items[2].x / item, self.items[2].y / item, self.items[2].z / item, self.items[2].w / item),
                Vector4d::new(self.items[3].x / item, self.items[3].y / item, self.items[3].z / item, self.items[3].w / item),
            ],
            rows: 4,
            cols: 4,
        }
    }
}

impl<T> fmt::Display for Matrix4d<T>
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
            f, "[{:?} {:?} {:?} {:?}]\n[{:?} {:?} {:?} {:?}]\n[{:?} {:?} {:?} {:?}]\n[{:?} {:?} {:?} {:?}]\n",
            self.items[0].x, self.items[0].y, self.items[0].z, self.items[0].w,
            self.items[1].x, self.items[1].y, self.items[1].z, self.items[1].w,
            self.items[2].x, self.items[2].y, self.items[2].z, self.items[2].w,
            self.items[3].x, self.items[3].y, self.items[3].z, self.items[3].w
        )
    }
}