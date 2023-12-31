use std::{fmt, ops};
use crate::matrix::vector2d::Vector2d;

/** 2D matrix
 Supports regular matrix calculations
 Supports comparing matrix for equality
 Each item type in the matrix must support the Copy trait and alternate addition, subtraction, multiplication, division and Product operations.
 */
#[derive(Debug, Clone, PartialEq, Copy)]
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
    pub items: [Vector2d<T>; 2],
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
            items: [v1, v2],
            rows: 2,
            cols: 2,
        }
    }

    /// Create a 2x2-dimensional matrix with an initial value of item_type
    pub fn identity(item_type: T) -> Matrix2d<T> {
        let items = [Vector2d::new(item_type, item_type); 2];
        Matrix2d {
            items,
            rows: 2,
            cols: 2,
        }
    }

    /** matrix transpose, Example
    ```rust
    let mut m = Matrix2d::new(Vector2d::new(2, 1), Vector2d::new(3, 4));
    m.transpose();
    ```

    output m:
    ```
    [2, 3]
    [1, 4]
    ```
     */
    pub fn transpose(&mut self) {
        let tmp = self.items[0].y;
        self.items[0].y = self.items[1].x;
        self.items[1].x = tmp;
    }

    /** Dot product of two 2x2 dimensional matrices, Example
    ```rust
    let m1 = Matrix2d::new(Vector2d::new(2, 1), Vector2d::new(3, 4));
    let m2 = Matrix2d::new(Vector2d::new(5, 7), Vector2d::new(2, 1));
    let m = m1 * m2;
    ```

    output m:
    ```
    [12, 15]
    [23, 25]
    ```
    */
    pub fn product(&self, mat: Matrix2d<T>) -> Matrix2d<T> {
        let mut res = Self::identity(mat.items[0].x - mat.items[0].x);
        let mut mat2 = self.clone();
        mat2.transpose();

        res.items[0].x = self.items[0].dot(mat2.items[0]);
        res.items[0].y = self.items[0].dot(mat2.items[1]);

        res.items[1].x = self.items[1].dot(mat2.items[0]);
        res.items[1].y = self.items[1].dot(mat2.items[1]);

        res
    }

    /** Dot product of a 2x2 dimensional matrix and a 2 dimensional vector, Example
    ```rust
    let m = Matrix2d::new(Vector2d::new(1, 2), Vector2d::new(3, 4));
    let v1 = Vector2d::new(2, 3);
    let v = m.product_with_vector2d(v1);
    ```

    output v:
    ```
    [8, 18]
    ```
    */
    pub fn product_with_vector2d(&self, v: Vector2d<T>) -> Vector2d<T> {
        let mut res = Vector2d::identity(self.items[0].x - self.items[0].x);
        res.x = self.items[0].dot(v);
        res.y = self.items[1].dot(v);
        res
    }

    /// Add two 2x2 matrix
    pub fn add(&self, mat: Matrix2d<T>) -> Matrix2d<T> {
        Matrix2d {
            items: [self.items[0] + (mat.items[0]),
                        self.items[1] + (mat.items[1])],
            rows: 2,
            cols: 2,
        }
    }

    /// Multiply two 2x2-dimensional matrix. Note that it is not a dot.
    pub fn hadamard(&self, mat: Matrix2d<T>) -> Matrix2d<T>{
        Matrix2d {
            items: [self.items[0].mul(mat.items[0]),
                        self.items[1].mul(mat.items[1])],
            rows: 2,
            cols: 2,
        }
    }

    /// Subtract two 2x2-dimensional matrix.
    pub fn sub(&self, mat: Matrix2d<T>) -> Matrix2d<T> {
        Matrix2d {
            items: [self.items[0].sub(mat.items[0]),
                        self.items[1].sub(mat.items[1])],
            rows: 2,
            cols: 2,
        }
    }

    /// Divide two 2x2-dimensional matrix.
    pub fn div(&self, mat: Matrix2d<T>) -> Matrix2d<T> {
        Matrix2d {
            items: [self.items[0].div(mat.items[0]),
                        self.items[1].div(mat.items[1])],
            rows: 2,
            cols: 2,
        }
    }

    /// Add a variable of the same type to each component of the 2x2-dimensional matrix
    pub fn add_item(&self, item: T) -> Matrix2d<T> {
        Matrix2d {
            items: [
                Vector2d::new(self.items[0].x + item, self.items[0].y + item),
                Vector2d::new(self.items[1].x + item, self.items[1].y + item)
            ],
            rows: 2,
            cols: 2,
        }
    }

    /// Subtract a variable of the same type to each component of the 2x2-dimensional matrix
    pub fn sub_item(&self, item: T) -> Matrix2d<T> {
        Matrix2d {
            items: [
                Vector2d::new(self.items[0].x - item, self.items[0].y - item),
                Vector2d::new(self.items[1].x - item, self.items[1].y - item)
            ],
            rows: 2,
            cols: 2,
        }
    }

    /// Multiply a variable of the same type to each component of the 2x2-dimensional matrix
    pub fn mul_item(&self, item: T) -> Matrix2d<T> {
        Matrix2d {
            items: [
                Vector2d::new(self.items[0].x * item, self.items[0].y * item),
                Vector2d::new(self.items[1].x * item, self.items[1].y * item)
            ],
            rows: 2,
            cols: 2,
        }
    }

    /// Divide a variable of the same type to each component of the 2x2-dimensional matrix
    pub fn div_item(&self, item: T) -> Matrix2d<T> {
        Matrix2d {
            items: [
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

impl<T> ops::Add for Matrix2d<T>
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
    type Output = Matrix2d<T>;

    /// Add two 2x2 matrix
    fn add(self, mat: Matrix2d<T>) -> Matrix2d<T> {
        Matrix2d {
            items: [self.items[0] + (mat.items[0]),
                        self.items[1] + (mat.items[1])],
            rows: 2,
            cols: 2,
        }
    }
}

impl<T> ops::Sub for Matrix2d<T>
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
    type Output = Matrix2d<T>;

    /// Subtract two 2x2-dimensional matrix.
    fn sub(self, mat: Matrix2d<T>) -> Matrix2d<T> {
        Matrix2d {
            items: [self.items[0].sub(mat.items[0]),
                        self.items[1].sub(mat.items[1])],
            rows: 2,
            cols: 2,
        }
    }
}

impl<T> ops::Mul for Matrix2d<T>
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
    type Output = Matrix2d<T>;

    /// Dot two 2x2-dimensional matrix.
    fn mul(self, mat: Matrix2d<T>) -> Matrix2d<T>  {
        let mut res = Self::identity(mat.items[0].x - mat.items[0].x);
        let mut mat2 = Matrix2d::new(
            Vector2d::new(mat.items[0].x, mat.items[1].x),
            Vector2d::new(mat.items[0].y, mat.items[1].y)
        );

        res.items[0].x = self.items[0].dot(mat2.items[0]);
        res.items[0].y = self.items[0].dot(mat2.items[1]);

        res.items[1].x = self.items[1].dot(mat2.items[0]);
        res.items[1].y = self.items[1].dot(mat2.items[1]);

        res
    }
}

impl<T> ops::Div for Matrix2d<T>
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
    type Output = Matrix2d<T>;

    /// Divide two 2x2-dimensional matrix.
    fn div(self, mat: Matrix2d<T>) -> Matrix2d<T> {
        Matrix2d {
            items: [self.items[0].div(mat.items[0]),
                        self.items[1].div(mat.items[1])],
            rows: 2,
            cols: 2,
        }
    }
}