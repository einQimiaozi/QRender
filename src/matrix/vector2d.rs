use std::{fmt, ops};
use crate::matrix::utils::Major;
use crate::matrix::vector_errors::{VectorError, VectorErrorType};

/** 2D vector
    Supports regular vector calculations
    Supports comparing vectors for equality
    Each item type in the vector must support the Copy trait and alternate addition, subtraction, multiplication, division and Dot operations.
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2d<T>
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
    pub major: Major
}

impl<T> Vector2d<T>
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
    pub fn new(x: T, y: T) -> Vector2d<T> {
        Vector2d{x, y, major: Major::Row}
    }

    /// Create a vector from a Vec
    pub fn from_vector(v: Vec<T>) -> Result<Vector2d<T>, VectorError> {
        if v.len() != 2 {
            return Err(
                VectorError {
                    err_msg: String::from("The dimension of the vector must be 2 dimensions"),
                    err_type: VectorErrorType::InvalidInitSizeError,
                }
            )
        }
        Ok (
            Vector2d{x: v[0], y: v[1], major: Major::Row}
        )
    }

    /// Create a 2-dimensional vector with an initial value of item_type
    pub fn identity(type_item: T) -> Vector2d<T> {
        Vector2d {
            x: type_item,
            y: type_item,
            major: Major::Row
        }
    }

    /// Transpose the 2-dimensional vector. Note that here we simply use markers to distinguish
    pub fn transpose(&mut self) {
        match self.major {
            Major::Col => self.major = Major::Row,
            Major::Row => self.major = Major::Col
        }
    }

    pub fn from_transpose(v: Vector2d<T>) -> Vector2d<T> {
        let mut res = Vector2d::identity(v.x);
        match v.major {
            Major::Col => res.major = Major::Row,
            Major::Row => res.major = Major::Col
        }
        res
    }

    /// Add two 2D vectors
    pub fn add(&self, w: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x + w.x,
            y: self.y + w.y,
            major: self.major
        }
    }

    /// Multiply two 2-dimensional vectors. Note that it is not a dot.
    pub fn mul(&self, w: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x * w.x,
            y: self.y * w.y,
            major: self.major
        }
    }

    /// Divide two 2D vectors
    pub fn div(&self, w: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x / w.x,
            y: self.y / w.y,
            major: self.major
        }
    }

    /// Subtract two 2D vectors
    pub fn sub(&self, w: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x - w.x,
            y: self.y - w.y,
            major: self.major
        }
    }

    /** Dot two 2D vectors, Example

    ```rust
    let v1 = Vector2d::new(1, 2);
    let v2 = Vector2d::new(3, 4);
    let res = v1 * v2;
    ```

    output res:
    ```
    11
    ```
    */
    pub fn dot(&self, w: Vector2d<T>) -> T {
        self.x * w.x + self.y * w.y
    }

    /// Find the norm length of a 2-dimensional vector
    pub fn norm(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    /// Add a variable of the same type to each component of the 2-dimensional vector
    pub fn add_item(v: Vector2d<T>, item: T) -> Vector2d<T> {
        Vector2d {
            x: v.x + item,
            y: v.y + item,
            major: v.major
        }
    }

    /// Divide a variable of the same type to each component of the 2-dimensional vector
    pub fn sub_item(v: Vector2d<T>, item: T) -> Vector2d<T> {
        Vector2d {
            x: v.x - item,
            y: v.y - item,
            major: v.major
        }
    }

    /// Multiply a variable of the same type to each component of the 2-dimensional vector
    pub fn mul_item(v: Vector2d<T>, item: T) -> Vector2d<T> {
        Vector2d {
            x: v.x * item,
            y: v.y * item,
            major: v.major
        }
    }

    /// Subtract a variable of the same type to each component of the 2-dimensional vector
    pub fn div_item(v: Vector2d<T>, item: T) -> Vector2d<T> {
        Vector2d {
            x: v.x / item,
            y: v.y / item,
            major: v.major
        }
    }

    /** Normalize a 2D vector, Example
    ```rust
    let mut v = Vector2d::new(3.0_f32, 5.0_f32);
    v.normalized();
    ```

    result v:
    ```
    [0.08823529411764706, 0.14705882352941177]
    ```
    */
    pub fn normalized(&mut self) {
        let n = self.norm();
        self.x = self.x / n;
        self.y = self.y / n;
    }
}

impl<T> fmt::Display for Vector2d<T>
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
                write!(f, "[{:?} {:?}]", self.x, self.y)
            },
            Major::Col => {
                write!(f, "[{:?}]\n[{:?}]", self.x, self.y)
            }
        }
    }
}

impl<T> ops::Add for Vector2d<T>
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
    type Output = Vector2d<T>;

    /// Add two 2D vectors
    fn add(self, w: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x + w.x,
            y: self.y + w.y,
            major: self.major
        }
    }
}

impl<T> ops::Sub for Vector2d<T>
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
    type Output = Vector2d<T>;

    /// Subtract two 2D vectors
    fn sub(self, w: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x - w.x,
            y: self.y - w.y,
            major: self.major
        }
    }
}

impl<T> ops::Mul for Vector2d<T>
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
    fn mul(self, w: Vector2d<T>) -> T {
        self.x * w.x + self.y * w.y
    }
}

impl<T> ops::Div for Vector2d<T>
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
    type Output = Vector2d<T>;

    /// Divide two 2D vectors
    fn div(self, w: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x / w.x,
            y: self.y / w.y,
            major: self.major
        }
    }
}