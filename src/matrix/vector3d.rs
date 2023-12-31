use std::{fmt, ops};
use crate::matrix::utils::Major;
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
    pub fn new(x: T, y: T, z: T) -> Vector3d<T> {
        Vector3d{x, y, z, major: Major::Row}
    }

    /// Create a vector from a Vec
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
    pub fn identity(type_item: T) -> Vector3d<T> {
        Vector3d {
            x: type_item,
            y: type_item,
            z: type_item,
            major: Major::Row
        }
    }

    /// Transpose the 3-dimensional vector. Note that here we simply use markers to distinguish
    pub fn transpose(&mut self) {
        match self.major {
            Major::Col => self.major = Major::Row,
            Major::Row => self.major = Major::Col
        }
    }

    pub fn from_transpose(v: Vector3d<T>) -> Vector3d<T> {
        let mut res = Vector3d::identity(v.x);
        match v.major {
            Major::Col => res.major = Major::Row,
            Major::Row => res.major = Major::Col
        }
        res
    }

    /// Add two 3D vectors
    pub fn add(&self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.x + w.x,
            y: self.y + w.y,
            z: self.z + w.z,
            major: self.major.clone()
        }
    }

    /// Multiply two 3-dimensional vectors. Note that it is not a dot.
    pub fn mul(&self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.x * w.x,
            y: self.y * w.y,
            z: self.z * w.z,
            major: self.major.clone()
        }
    }

    /// Divide two 3D vectors
    pub fn div(&self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.x / w.x,
            y: self.y / w.y,
            z: self.z / w.z,
            major: self.major.clone()
        }
    }

    /// Subtract two 3D vectors
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
    pub fn cross(&self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.y * w.z - w.y * self.z,
            y: self.z * w.x - w.z * self.x,
            z: self.x * w.y - w.x * self.y,
            major: self.major.clone()
        }
    }

    /// Add a variable of the same type to each component of the 3-dimensional vector
    pub fn add_item(v: Vector3d<T>, item: T) -> Vector3d<T> {
        Vector3d {
            x: v.x + item,
            y: v.y + item,
            z: v.z + item,
            major: v.major.clone()
        }
    }

    /// Subtract a variable of the same type to each component of the 3-dimensional vector
    pub fn sub_item(v: Vector3d<T>, item: T) -> Vector3d<T> {
        Vector3d {
            x: v.x - item,
            y: v.y - item,
            z: v.z - item,
            major: v.major.clone()
        }
    }

    /// Multiply a variable of the same type to each component of the 3-dimensional vector
    pub fn mul_item(v: Vector3d<T>, item: T) -> Vector3d<T> {
        Vector3d {
            x: v.x * item,
            y: v.y * item,
            z: v.z * item,
            major: v.major.clone()
        }
    }

    /// Divide a variable of the same type to each component of the 3-dimensional vector
    pub fn div_item(v: Vector3d<T>, item: T) -> Vector3d<T> {
        Vector3d {
            x: v.x / item,
            y: v.y / item,
            z: v.z / item,
            major: v.major.clone()
        }
    }

    /// Find the norm length of a 3-dimensional vector
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
    pub fn normalized(&mut self) {
        let n = self.norm();
        self.x = self.x / n;
        self.y = self.y / n;
        self.z = self.z / n;
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
    fn div(self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.x / w.x,
            y: self.y / w.y,
            z: self.z / w.z,
            major: self.major
        }
    }
}