use std::{fmt, ops};
use crate::matrix::vector3d::Vector3d;

/** 3D matrix
  Supports regular matrix calculations
  Supports comparing matrix for equality
  Each item type in the matrix must support the Copy trait and alternate addition, subtraction, multiplication, division and Product operations.
 */
#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub items: [Vector3d<T>; 3],
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
            items: [v1, v2, v3],
            rows: 3,
            cols: 3,
        }
    }

    /// Create a 3x3-dimensional matrix with an initial value of item_type
    pub fn identity(item_type: T) -> Matrix3d<T> {
        let items = [Vector3d::new(item_type, item_type, item_type); 3];
        Matrix3d {
            items,
            rows: 3,
            cols: 3,
        }
    }

    /** matrix transpose, Example
    ```rust
    let mut m = Matrix3d::new(Vector3d::new(2, 1, 0), Vector3d::new(3, 4, 5), Vector3d::new(6, 7, 8));
    m.transpose();
    ```

    output m:
    ```
    [2, 3, 6]
    [1, 4, 7]
    [0, 5, 8]
    ```
     */
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

    /** Dot product of two 3x3 dimensional matrices, Example
    ```rust
    let m1 = Matrix3d::new(Vector3d::new(2, 1, 0), Vector3d::new(3, 4, 5), Vector3d::new(6, 7, 8));
    let m2 = Matrix3d::new(Vector3d::new(3, 0, 4), Vector3d::new(7, 1, 3), Vector3d::new(9, 2, 5));
    let m = m1 * m2;
    ```

    output m:
    ```
    [13,  1,  11]
    [82,  14, 49]
    [139, 23, 85]
    ```
     */
    pub fn product(&self, mat: Matrix3d<T>) -> Matrix3d<T> {
        let mut res = Self::identity(mat.items[0].x - mat.items[0].x);
        let mut mat2 = self.clone();
        mat2.transpose();

        res.items[0].x = self.items[0].dot(mat2.items[0]);
        res.items[0].y = self.items[0].dot(mat2.items[1]);
        res.items[0].z = self.items[0].dot(mat2.items[2]);

        res.items[1].x = self.items[1].dot(mat2.items[0]);
        res.items[1].y = self.items[1].dot(mat2.items[1]);
        res.items[1].z = self.items[1].dot(mat2.items[2]);

        res.items[2].x = self.items[2].dot(mat2.items[0]);
        res.items[2].y = self.items[2].dot(mat2.items[1]);
        res.items[2].z = self.items[2].dot(mat2.items[2]);

        res
    }

    /** Dot product of a 3x3 dimensional matrix and a 3 dimensional vector, Example
    ```rust
    let m = Matrix3d::new(Vector3d::new(1, 2, 5), Vector3d::new(3, 4, 7), Vector3d::new(0, 6, 3));
    let v1 = Vector3d::new(2, 3, 5);
    let v = m.product_with_vector3d(v1);
    ```

    output v:
    ```
    [33, 53, 33]
    ```
     */
    pub fn product_with_vector3d(&self, v: Vector3d<T>) -> Vector3d<T> {
        let mut res = Vector3d::identity(self.items[0].x - self.items[0].x);
        res.x = self.items[0].dot(v);
        res.y = self.items[1].dot(v);
        res.z = self.items[2].dot(v);
        res
    }

    /// Add two 3x3 matrix
    pub fn add(&self, mat: Matrix3d<T>) -> Matrix3d<T> {
        Matrix3d {
            items: [self.items[0].add(mat.items[0]),
                        self.items[1].add(mat.items[1]),
                        self.items[2].add(mat.items[2])],
            rows: 3,
            cols: 3,
        }
    }

    /// Multiply two 3x3-dimensional matrix. Note that it is not a dot.
    pub fn hadamard(&self, mat: Matrix3d<T>) -> Matrix3d<T>{
        Matrix3d {
            items: [self.items[0].mul(mat.items[0]),
                        self.items[1].mul(mat.items[1]),
                        self.items[2].mul(mat.items[2])],
            rows: 3,
            cols: 3,
        }
    }

    /// Subtract two 3x3-dimensional matrix.
    pub fn sub(&self, mat: Matrix3d<T>) -> Matrix3d<T> {
        Matrix3d {
            items: [self.items[0].sub(mat.items[0]),
                        self.items[1].sub(mat.items[1]),
                        self.items[2].sub(mat.items[2])],
            rows: 3,
            cols: 3,
        }
    }

    /// Divide two 3x3-dimensional matrix.
    pub fn div(&self, mat: Matrix3d<T>) -> Matrix3d<T> {
        Matrix3d {
            items: [self.items[0].div(mat.items[0]),
                        self.items[1].div(mat.items[1]),
                        self.items[2].div(mat.items[2])],
            rows: 3,
            cols: 3,
        }
    }

    /// Add a variable of the same type to each component of the 3x3-dimensional matrix
    pub fn add_item(&self, item: T) -> Matrix3d<T> {
        Matrix3d {
            items: [
                Vector3d::new(self.items[0].x + item, self.items[0].y + item, self.items[0].z + item),
                Vector3d::new(self.items[1].x + item, self.items[1].y + item, self.items[1].z + item),
                Vector3d::new(self.items[2].x + item, self.items[2].y + item, self.items[2].z + item)
            ],
            rows: 3,
            cols: 3,
        }
    }

    /// Subtract a variable of the same type to each component of the 3x3-dimensional matrix
    pub fn sub_item(&self, item: T) -> Matrix3d<T> {
        Matrix3d {
            items: [
                Vector3d::new(self.items[0].x - item, self.items[0].y - item, self.items[0].z - item),
                Vector3d::new(self.items[1].x - item, self.items[1].y - item, self.items[1].z - item),
                Vector3d::new(self.items[2].x - item, self.items[2].y - item, self.items[2].z - item)
            ],
            rows: 3,
            cols: 3,
        }
    }

    /// Multiply a variable of the same type to each component of the 3x3-dimensional matrix
    pub fn mul_item(&self, item: T) -> Matrix3d<T> {
        Matrix3d {
            items: [
                Vector3d::new(self.items[0].x * item, self.items[0].y * item, self.items[0].z * item),
                Vector3d::new(self.items[1].x * item, self.items[1].y * item, self.items[1].z * item),
                Vector3d::new(self.items[2].x * item, self.items[2].y * item, self.items[2].z * item)
            ],
            rows: 3,
            cols: 3,
        }
    }

    /// Divide a variable of the same type to each component of the 3x3-dimensional matrix
    pub fn div_item(&self, item: T) -> Matrix3d<T> {
        Matrix3d {
            items: [
                Vector3d::new(self.items[0].x / item, self.items[0].y / item, self.items[0].z / item),
                Vector3d::new(self.items[1].x / item, self.items[1].y / item, self.items[1].z / item),
                Vector3d::new(self.items[2].x / item, self.items[2].y / item, self.items[2].z / item)
            ],
            rows: 3,
            cols: 3,
        }
    }

    /** Cross-multiply two 3x3-dimensional matrices and return a 3x3-dimensional matrix, Example
    ```rust
    let m1 = Matrix3d::new(
        Vector3d::new(1, 2, 3),
        Vector3d::new(4, 5, 6),
        Vector3d::new(7, 8, 9)
    );
    let v = Vector3d::new(2, 3, 5);
    let m = m1.cross(v);
    ```

    output m
    ```
    [1,   7,  13]
    [1,  -8, -17]
    [-1,  2,   5]
    ```
    */
    pub fn cross(&self, v: Vector3d<T>) -> Matrix3d<T> {
        Matrix3d::new(
            self.items[0].cross(v),
            self.items[1].cross(v),
            self.items[2].cross(v)
        )
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

impl<T> ops::Add for Matrix3d<T>
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
    type Output = Matrix3d<T>;

    /// Add two 2x2 matrix
    fn add(self, mat: Matrix3d<T>) -> Matrix3d<T> {
        Matrix3d {
            items: [self.items[0].add(mat.items[0]),
                        self.items[1].add(mat.items[1]),
                        self.items[2].add(mat.items[2])],
            rows: 3,
            cols: 3,
        }
    }
}

impl<T> ops::Sub for Matrix3d<T>
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
    type Output = Matrix3d<T>;

    /// Subtract two 3x3-dimensional matrix.
    fn sub(self, mat: Matrix3d<T>) -> Matrix3d<T> {
        Matrix3d {
            items: [self.items[0].sub(mat.items[0]),
                self.items[1].sub(mat.items[1]),
                self.items[2].sub(mat.items[2])],
            rows: 3,
            cols: 3,
        }
    }
}

impl<T> ops::Mul for Matrix3d<T>
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
    type Output = Matrix3d<T>;

    /// Dot two 3x3-dimensional matrix.
    fn mul(self, mat: Matrix3d<T>) -> Matrix3d<T>  {
        let mut res = Self::identity(mat.items[0].x - mat.items[0].x);
        let mut mat2 = Matrix3d::new(
            Vector3d::new(mat.items[0].x, mat.items[1].x, mat.items[2].x),
            Vector3d::new(mat.items[0].y, mat.items[1].y, mat.items[2].y),
            Vector3d::new(mat.items[0].z, mat.items[1].z, mat.items[2].z)
        );

        res.items[0].x = self.items[0].dot(mat2.items[0]);
        res.items[0].y = self.items[0].dot(mat2.items[1]);
        res.items[0].z = self.items[0].dot(mat2.items[2]);

        res.items[1].x = self.items[1].dot(mat2.items[0]);
        res.items[1].y = self.items[1].dot(mat2.items[1]);
        res.items[1].z = self.items[1].dot(mat2.items[2]);

        res.items[2].x = self.items[2].dot(mat2.items[0]);
        res.items[2].y = self.items[2].dot(mat2.items[1]);
        res.items[2].z = self.items[2].dot(mat2.items[2]);

        res
    }
}

impl<T> ops::Div for Matrix3d<T>
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
    type Output = Matrix3d<T>;

    /// Divide two 3x3-dimensional matrix.
    fn div(self, mat: Matrix3d<T>) -> Matrix3d<T> {
        Matrix3d {
            items: [self.items[0].div(mat.items[0]),
                self.items[1].div(mat.items[1]),
                self.items[2].div(mat.items[2])],
            rows: 3,
            cols: 3,
        }
    }
}