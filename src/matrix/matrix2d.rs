use std::{fmt, ops};
use crate::matrix::vector2d::Vector2d;

#[derive(Debug, Clone)]
pub struct Matrix2d<T>
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
    pub items: Vec<Vector2d<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Matrix2d<T>
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
    pub fn new(v1: Vector2d<T>, v2: Vector2d<T>) -> Matrix2d<T> {
        Matrix2d {
            items: vec![v1, v2],
            rows: 2,
            cols: 2,
        }
    }
    pub fn identity(item_type: T) -> Matrix2d<T> {
        let mut items = Vec::with_capacity(2);
        items.resize(2, Vector2d::new(item_type, item_type));
        Matrix2d {
            items,
            rows: 2,
            cols: 2,
        }
    }
    pub fn transpose(&mut self) {
        let tmp = self.items[0].y;
        self.items[0].y = self.items[1].x;
        self.items[1].x = tmp;
    }
    pub fn product(&mut self, mat: Matrix2d<T>) -> Matrix2d<T> {
        let mut res = Self::identity(mat.items[0].x - mat.items[0].x);
        let mut mat1 = self.clone();
        let mut mat2 = mat.clone();

        res.items[0].x = mat1.items[0].dot(mat2.items[0].clone());
        res.items[0].y = mat1.items[0].dot(mat2.items[1].clone());

        res.items[1].x = mat1.items[1].dot(mat2.items[0].clone());
        res.items[1].y = mat1.items[1].dot(mat2.items[1].clone());

        res
    }
    pub fn product_with_vector2d(&mut self, v: Vector2d<T>) -> Vector2d<T> {
        let mut res = Vector2d::identity(self.items[0].x - self.items[0].x);
        res.x = self.items[0].dot(v.clone());
        res.y = self.items[1].dot(v.clone());
        res
    }
    pub fn add(&mut self, mat: Matrix2d<T>) -> Matrix2d<T> {
        Matrix2d {
            items: vec![self.items[0].add(mat.items[0].clone()),
                        self.items[1].add(mat.items[1].clone())],
            rows: 2,
            cols: 2,
        }
    }
    pub fn hadamard(&mut self, mat: Matrix2d<T>) -> Matrix2d<T>{
        Matrix2d {
            items: vec![self.items[0].mul(mat.items[0].clone()),
                        self.items[1].mul(mat.items[1].clone())],
            rows: 2,
            cols: 2,
        }
    }

    pub fn sub(&mut self, mat: Matrix2d<T>) -> Matrix2d<T> {
        Matrix2d {
            items: vec![self.items[0].sub(mat.items[0].clone()),
                        self.items[1].sub(mat.items[1].clone())],
            rows: 2,
            cols: 2,
        }
    }

    pub fn div(&mut self, mat: Matrix2d<T>) -> Matrix2d<T> {
        Matrix2d {
            items: vec![self.items[0].div(mat.items[0].clone()),
                        self.items[1].div(mat.items[1].clone())],
            rows: 2,
            cols: 2,
        }
    }

    pub fn add_item(&mut self, item: T) -> Matrix2d<T> {
        Matrix2d {
            items: vec![
                Vector2d::new(self.items[0].x + item, self.items[0].y + item),
                Vector2d::new(self.items[1].x + item, self.items[1].y + item)
            ],
            rows: 2,
            cols: 2,
        }
    }
    pub fn sub_item(&mut self, item: T) -> Matrix2d<T> {
        Matrix2d {
            items: vec![
                Vector2d::new(self.items[0].x - item, self.items[0].y - item),
                Vector2d::new(self.items[1].x - item, self.items[1].y - item)
            ],
            rows: 2,
            cols: 2,
        }
    }
    pub fn mul_item(&mut self, item: T) -> Matrix2d<T> {
        Matrix2d {
            items: vec![
                Vector2d::new(self.items[0].x * item, self.items[0].y * item),
                Vector2d::new(self.items[1].x * item, self.items[1].y * item)
            ],
            rows: 2,
            cols: 2,
        }
    }
    pub fn div_item(&mut self, item: T) -> Matrix2d<T> {
        Matrix2d {
            items: vec![
                Vector2d::new(self.items[0].x / item, self.items[0].y / item),
                Vector2d::new(self.items[1].x / item, self.items[1].y / item)
            ],
            rows: 2,
            cols: 2,
        }
    }
}

impl<T> fmt::Display for Matrix2d<T>
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
            f, "[{:?} {:?}]\n[{:?} {:?}]\n",
            self.items[0].x, self.items[0].y,
            self.items[1].x, self.items[1].y
        )
    }
}