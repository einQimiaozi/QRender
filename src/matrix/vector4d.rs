use std::{fmt, ops};
use crate::matrix::matrix4d::Matrix4d;
use crate::matrix::utils::Major;
use crate::matrix::vector_errors::{VectorError, VectorErrorType};

/** 4D vector
  Supports regular vector calculations
  Supports comparing vectors for equality
  Each item type in the vector must support the Copy trait and alternate addition, subtraction, multiplication, division and Dot operations.
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4d<T>
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
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
    pub major: Major
}

impl<T> Vector4d<T>
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
    pub fn new(x: T, y: T, z: T, w: T) -> Vector4d<T> {
        Vector4d{x, y, z, w, major: Major::Row}
    }

    /// Create a vector from a Vec
    pub fn from_vector(v: Vec<T>) -> Result<Vector4d<T>, VectorError> {
        if v.len() != 4 {
            return Err(
                VectorError {
                    err_msg: String::from("The dimension of the vector must be 4 dimensions"),
                    err_type: VectorErrorType::InvalidInitSizeError,
                }
            )
        }
        Ok (
            Vector4d{x: v[0], y: v[1], z: v[2], w: v[3], major: Major::Row}
        )
    }

    /// Create a 4-dimensional vector with an initial value of item_type
    pub fn identity(type_item: T) -> Vector4d<T> {
        Vector4d {
            x: type_item,
            y: type_item,
            z: type_item,
            w: type_item,
            major: Major::Row
        }
    }

    /// Transpose the 4-dimensional vector. Note that here we simply use markers to distinguish
    pub fn transpose(&mut self) {
        match self.major {
            Major::Col => self.major = Major::Row,
            Major::Row => self.major = Major::Col
        }
    }

    pub fn from_transpose(v: Vector4d<T>) -> Vector4d<T> {
        let mut res = Vector4d::identity(v.x);
        match v.major {
            Major::Col => res.major = Major::Row,
            Major::Row => res.major = Major::Col
        }
        res
    }

    /// Add two 4D vectors
    pub fn add(&self, w: Vector4d<T>) -> Vector4d<T> {
        Vector4d {
            x: self.x + w.x,
            y: self.y + w.y,
            z: self.z + w.z,
            w: self.w + w.w,
            major: self.major.clone()
        }
    }

    /// Multiply two 4-dimensional vectors. Note that it is not a dot.
    pub fn mul(&self, w: Vector4d<T>) -> Vector4d<T> {
        Vector4d {
            x: self.x * w.x,
            y: self.y * w.y,
            z: self.z * w.z,
            w: self.w * w.w,
            major: self.major.clone()
        }
    }

    /// Subtract two 4D vectors
    pub fn sub(&self, w: Vector4d<T>) -> Vector4d<T> {
        Vector4d {
            x: self.x - w.x,
            y: self.y - w.y,
            z: self.z - w.z,
            w: self.w - w.w,
            major: self.major.clone()
        }
    }

    /// Divide two 4D vectors
    pub fn div(&self, w: Vector4d<T>) -> Vector4d<T> {
        Vector4d {
            x: self.x / w.x,
            y: self.y / w.y,
            z: self.z / w.z,
            w: self.w / w.w,
            major: self.major.clone()
        }
    }

    /** Dot two 4D vectors, Example
    ```rust
    let v1 = Vector4d::new(1, 2, 3, 4);
    let v2 = Vector4d::new(5, 6, 7, 8);
    let res = v1 * v2;
    ```

    output res:
    ```
    32
    ```
     */
    pub fn dot(&self, w: Vector4d<T>) -> T {
        self.x * w.x + self.y * w.y + self.z *w.z + self.w * w.w
    }

    /// Add a variable of the same type to each component of the 4-dimensional vector
    pub fn add_item(v: Vector4d<T>, item: T) -> Vector4d<T> {
        Vector4d {
            x: v.x + item,
            y: v.x + item,
            z: v.x + item,
            w: v.x + item,
            major: v.major.clone()
        }
    }

    /// Subtract a variable of the same type to each component of the 4-dimensional vector
    pub fn sub_item(v: Vector4d<T>, item: T) -> Vector4d<T> {
        Vector4d {
            x: v.x - item,
            y: v.x - item,
            z: v.x - item,
            w: v.x - item,
            major: v.major.clone()
        }
    }

    /// Multiply a variable of the same type to each component of the 4-dimensional vector
    pub fn mul_item(v: Vector4d<T>, item: T) -> Vector4d<T> {
        Vector4d {
            x: v.x * item,
            y: v.x * item,
            z: v.x * item,
            w: v.x * item,
            major: v.major.clone()
        }
    }

    /// Divide a variable of the same type to each component of the 4-dimensional vector
    pub fn div_item(v: Vector4d<T>, item: T) -> Vector4d<T> {
        Vector4d {
            x: v.x / item,
            y: v.x / item,
            z: v.x / item,
            w: v.x / item,
            major: v.major.clone()
        }
    }

    /// Find the norm length of a 4-dimensional vector
    pub fn norm(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /** Normalize a 2D vector, Example
    ```rust
    let mut v = Vector4d::new(3.0_f32, 5.0_f32, 2.0_f32, 6.0_f32);
    v.normalized();
    ```

    result v:
    ```
    [0.04054054054054054, 0.06756756756756757, 0.02702702702702703, 0.08108108108108109]
    ```
     */
    pub fn normalized(&mut self) {
        let n = self.norm();
        self.x = self.x / n;
        self.y = self.y / n;
        self.z = self.z / n;
        self.w = self.w / n;
    }

    /** Multiply a 4-dimensional vector left by a 4x4-dimensional matrix to return a 4-dimensional vector, Example
    ```rust
    let v1 = Vector4d::new(3, 5, 6, 1);
    let m = Matrix4d::new(
        Vector4d::new(1, 2, 3, 4),
        Vector4d::new(5, 6, 7, 8),
        Vector4d::new(9, 0, 1, 2),
        Vector4d::new(3, 4, 5, 6)
    );
    let v = v1.product_with_matrix4d(m);
    ```

    output v:
    ```
    [85, 40, 55, 70]
    ```
     */
    pub fn product_with_matrix4d(&self, m: Matrix4d<T>) -> Vector4d<T> {
        let mut mat2 = m.clone();
        mat2.transpose();
        Vector4d::new(
            self.dot(mat2.items[0]),
            self.dot(mat2.items[1]),
            self.dot(mat2.items[2]),
            self.dot(mat2.items[3])
        )
    }
}

impl<T> fmt::Display for Vector4d<T>
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
        match self.major {
            Major::Row => {
                write!(f, "[{:?} {:?} {:?} {:?}]", self.x, self.y, self.z, self.w)
            },
            Major::Col => {
                write!(f, "[{:?}]\n[{:?}]\n[{:?}]\n[{:?}]", self.x, self.y, self.z, self.w)
            }
        }
    }
}

impl<T> ops::Add for Vector4d<T>
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
    type Output = Vector4d<T>;

    /// Add two 2D vectors
    fn add(self, w: Vector4d<T>) -> Vector4d<T> {
        Vector4d {
            x: self.x + w.x,
            y: self.y + w.y,
            z: self.z + w.z,
            w: self.w + w.w,
            major: self.major
        }
    }
}

impl<T> ops::Sub for Vector4d<T>
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
    type Output = Vector4d<T>;

    /// Subtract two 2D vectors
    fn sub(self, w: Vector4d<T>) -> Vector4d<T> {
        Vector4d {
            x: self.x - w.x,
            y: self.y - w.y,
            z: self.z - w.z,
            w: self.w - w.w,
            major: self.major
        }
    }
}

impl<T> ops::Mul for Vector4d<T>
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
    type Output = T;

    /// Dot two 2D vectors
    fn mul(self, w: Vector4d<T>) -> T {
        self.x * w.x + self.y * w.y + self.z *w.z + self.w * w.w
    }
}

impl<T> ops::Div for Vector4d<T>
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
    type Output = Vector4d<T>;

    /// Divide two 2D vectors
    fn div(self, w: Vector4d<T>) -> Vector4d<T> {
        Vector4d {
            x: self.x / w.x,
            y: self.y / w.y,
            z: self.z / w.z,
            w: self.w / w.w,
            major: self.major
        }
    }
}