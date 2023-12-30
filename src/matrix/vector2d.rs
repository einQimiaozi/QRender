use std::{fmt, ops};
use crate::matrix::utils::Major;
use crate::matrix::vector_errors::{VectorError, VectorErrorType};

#[derive(Debug, Clone)]
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
    pub fn identity(type_item: T) -> Vector2d<T> {
        Vector2d {
            x: type_item,
            y: type_item,
            major: Major::Row
        }
    }
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
    pub fn add(&self, w: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x + w.x,
            y: self.y + w.y,
            major: self.major.clone()
        }
    }
    pub fn mul(&self, w: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x * w.x,
            y: self.y * w.y,
            major: self.major.clone()
        }
    }
    pub fn div(&self, w: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x / w.x,
            y: self.y / w.y,
            major: self.major.clone()
        }
    }
    pub fn sub(&self, w: Vector2d<T>) -> Vector2d<T> {
        Vector2d {
            x: self.x - w.x,
            y: self.y - w.y,
            major: self.major.clone()
        }
    }
    pub fn dot(&self, w: Vector2d<T>) -> T {
        self.x * w.x + self.y * w.y
    }
    pub fn add_item(v: Vector2d<T>, item: T) -> Vector2d<T> {
        Vector2d {
            x: v.x + item,
            y: v.y + item,
            major: v.major.clone()
        }
    }
    pub fn sub_item(v: Vector2d<T>, item: T) -> Vector2d<T> {
        Vector2d {
            x: v.x - item,
            y: v.y - item,
            major: v.major.clone()
        }
    }
    pub fn mul_item(v: Vector2d<T>, item: T) -> Vector2d<T> {
        Vector2d {
            x: v.x * item,
            y: v.y * item,
            major: v.major.clone()
        }
    }
    pub fn div_item(v: Vector2d<T>, item: T) -> Vector2d<T> {
        Vector2d {
            x: v.x / item,
            y: v.y / item,
            major: v.major.clone()
        }
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