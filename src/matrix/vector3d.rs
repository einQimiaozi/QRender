use std::{fmt, ops};
use crate::matrix::vector4d::Vector4d;
use crate::matrix::matrix3d::Matrix3d;
use crate::matrix::utils::Major;
use crate::matrix::vector2d::Vector2d;
use crate::matrix::vector_errors::{VectorError, VectorErrorType};

/** 3D vector
   Supports regular vector calculations
   Supports comparing vectors for equality
   Each item type in the vector must support the Copy trait and alternate addition, subtraction, multiplication, division, Dot and Cross operations.
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3d<T>
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
    pub major: Major
}

impl<T> Vector3d<T>
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
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Vector3d<T> {
        Vector3d{x, y, z, major: Major::Row}
    }

    /// Create a vector from a Vec
    #[inline]
    pub fn from_vector(v: Vec<T>) -> Result<Vector3d<T>, VectorError> {
        if v.len() != 3 {
            return Err(
                VectorError {
                    err_msg: String::from("The dimension of the vector must be 3 dimensions"),
                    err_type: VectorErrorType::InvalidInitSizeError,
                }
            )
        }
        Ok (
            Vector3d{x: v[0], y: v[1], z: v[2], major: Major::Row}
        )
    }

    /// Create a 3-dimensional vector with an initial value of item_type
    #[inline]
    pub fn fill(type_item: T) -> Vector3d<T> {
        Vector3d {
            x: type_item,
            y: type_item,
            z: type_item,
            major: Major::Row
        }
    }

    /// Create a 3-dimensional zero vector
    #[inline]
    pub fn zero(type_item: T) -> Vector3d<T> {
        Self::fill(type_item - type_item)
    }

    /// Transpose the 3-dimensional vector. Note that here we simply use markers to distinguish
    #[inline]
    pub fn transpose(&mut self) {
        match self.major {
            Major::Col => self.major = Major::Row,
            Major::Row => self.major = Major::Col
        }
    }

    #[inline]
    pub fn from_transpose(v: Vector3d<T>) -> Vector3d<T> {
        let mut res = Vector3d::fill(v.x);
        match v.major {
            Major::Col => res.major = Major::Row,
            Major::Row => res.major = Major::Col
        }
        res
    }

    /// Add two 3D vectors
    #[inline]
    pub fn add(&self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.x + w.x,
            y: self.y + w.y,
            z: self.z + w.z,
            major: self.major.clone()
        }
    }

    /// Multiply two 3-dimensional vectors. Note that it is not a dot.
    #[inline]
    pub fn mul(&self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.x * w.x,
            y: self.y * w.y,
            z: self.z * w.z,
            major: self.major.clone()
        }
    }

    /// Divide two 3D vectors
    #[inline]
    pub fn div(&self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.x / w.x,
            y: self.y / w.y,
            z: self.z / w.z,
            major: self.major.clone()
        }
    }

    /// Subtract two 3D vectors
    #[inline]
    pub fn sub(&self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.x - w.x,
            y: self.y - w.y,
            z: self.z - w.z,
            major: self.major.clone()
        }
    }

    /** Dot two 3D vectors, Example

    ```rust
    let v1 = Vector3d::new(1, 2, 3);
    let v2 = Vector3d::new(4, 5, 6);
    let res = v1 * v2;
    ```

    output res:
    ```
    32
    ```
     */
    #[inline]
    pub fn dot(&self, w: Vector3d<T>) -> T {
        self.x * w.x + self.y * w.y + self.z * w.z
    }

    /** Cross product of two 3D vectors, Example
    ```rust
    let v1 = Vector3d::new(1, 2, 3);
    let v2 = Vector3d::new(4, 5, 6);
    let v = v1.cross(v2);
    ```

    output v:
    ```
    [-3, 6, -3]
    ```
    */
    #[inline]
    pub fn cross(&self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.y * w.z - w.y * self.z,
            y: self.z * w.x - w.z * self.x,
            z: self.x * w.y - w.x * self.y,
            major: self.major.clone()
        }
    }

    /// Add a variable of the same type to each component of the 3-dimensional vector
    #[inline]
    pub fn add_item(v: Vector3d<T>, item: T) -> Vector3d<T> {
        Vector3d {
            x: v.x + item,
            y: v.y + item,
            z: v.z + item,
            major: v.major.clone()
        }
    }

    /// Subtract a variable of the same type to each component of the 3-dimensional vector
    #[inline]
    pub fn sub_item(v: Vector3d<T>, item: T) -> Vector3d<T> {
        Vector3d {
            x: v.x - item,
            y: v.y - item,
            z: v.z - item,
            major: v.major.clone()
        }
    }

    /// Multiply a variable of the same type to each component of the 3-dimensional vector
    #[inline]
    pub fn mul_item(v: Vector3d<T>, item: T) -> Vector3d<T> {
        Vector3d {
            x: v.x * item,
            y: v.y * item,
            z: v.z * item,
            major: v.major.clone()
        }
    }

    /// Divide a variable of the same type to each component of the 3-dimensional vector
    #[inline]
    pub fn div_item(v: Vector3d<T>, item: T) -> Vector3d<T> {
        Vector3d {
            x: v.x / item,
            y: v.y / item,
            z: v.z / item,
            major: v.major.clone()
        }
    }

    /// Find the norm length of a 3-dimensional vector
    #[inline]
    pub fn norm(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /** Normalize a 2D vector, Example
    ```rust
    let mut v = Vector3d::new(3.0_f32, 5.0_f32, 2.0_f32);
    v.normalized();
    ```

    result v:
    ```
    [0.07894736842105263, 0.13157894736842105, 0.05263157894736842]
    ```
     */
    #[inline]
    pub fn normalized(&mut self) {
        let n = self.norm();
        self.x = self.x / n;
        self.y = self.y / n;
        self.z = self.z / n;
    }

    /** Multiply a 3-dimensional vector left by a 3x3-dimensional matrix to return a 3-dimensional vector, Example
    ```rust
    let v1 = Vector3d::new(3, 5, 6);
    let m = Matrix3d::new(Vector3d::new(1, 2, 3), Vector3d::new(4, 5, 6), Vector3d::new(7, 8, 9));
    let v = v1.product_with_matrix2d(m);
    ```

    output v:
    ```
    [65, 79, 93]
    ```
     */
    #[inline]
    pub fn product_with_matrix3d(&self, m: Matrix3d<T>) -> Vector3d<T> {
        let mut mat2 = m.clone();
        mat2.transpose();
        Vector3d::new(
            self.dot(mat2.items[0]),
            self.dot(mat2.items[1]),
            self.dot(mat2.items[2])
        )
    }

    #[inline]
    pub fn to_vector4d(&self, add_item: T) -> Vector4d<T> {
        Vector4d::new(self.x, self.y, self.z, add_item)
    }

    #[inline]
    pub fn head2(&self) -> Vector2d<T> {
        Vector2d::new(self.x, self.y)
    }
}

impl<T> fmt::Display for Vector3d<T>
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
                write!(f, "[{:?} {:?} {:?}]", self.x, self.y, self.z)
            },
            Major::Col => {
                write!(f, "[{:?}]\n[{:?}]\n[{:?}]", self.x, self.y, self.z)
            }
        }
    }
}

impl<T> ops::Add for Vector3d<T>
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
    type Output = Vector3d<T>;

    /// Add two 2D vectors
    #[inline]
    fn add(self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.x + w.x,
            y: self.y + w.y,
            z: self.z + w.z,
            major: self.major
        }
    }
}

impl<T> ops::Sub for Vector3d<T>
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
    type Output = Vector3d<T>;

    /// Subtract two 2D vectors
    #[inline]
    fn sub(self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.x - w.x,
            y: self.y - w.y,
            z: self.z - w.z,
            major: self.major
        }
    }
}

impl<T> ops::Mul for Vector3d<T>
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
    #[inline]
    fn mul(self, w: Vector3d<T>) -> T {
        self.x * w.x + self.y * w.y + self.z * w.z
    }
}

impl<T> ops::Div for Vector3d<T>
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
    type Output = Vector3d<T>;

    /// Divide two 2D vectors
    #[inline]
    fn div(self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.x / w.x,
            y: self.y / w.y,
            z: self.z / w.z,
            major: self.major
        }
    }
}