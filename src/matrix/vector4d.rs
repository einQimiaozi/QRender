use std::{fmt, ops};
use crate::matrix::utils::Major;
use crate::matrix::vector_errors::{VectorError, VectorErrorType};

#[derive(Debug, Clone)]
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
    pub fn identity(type_item: T) -> Vector4d<T> {
        Vector4d {
            x: type_item,
            y: type_item,
            z: type_item,
            w: type_item,
            major: Major::Row
        }
    }
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
    pub fn add(&self, w: Vector4d<T>) -> Vector4d<T> {
        Vector4d {
            x: self.x + w.x,
            y: self.y + w.y,
            z: self.z + w.z,
            w: self.w + w.w,
            major: self.major.clone()
        }
    }
    pub fn mul(&self, w: Vector4d<T>) -> Vector4d<T> {
        Vector4d {
            x: self.x * w.x,
            y: self.y * w.y,
            z: self.z * w.z,
            w: self.w * w.w,
            major: self.major.clone()
        }
    }
    pub fn sub(&self, w: Vector4d<T>) -> Vector4d<T> {
        Vector4d {
            x: self.x - w.x,
            y: self.y - w.y,
            z: self.z - w.z,
            w: self.w - w.w,
            major: self.major.clone()
        }
    }
    pub fn div(&self, w: Vector4d<T>) -> Vector4d<T> {
        Vector4d {
            x: self.x / w.x,
            y: self.y / w.y,
            z: self.z / w.z,
            w: self.w / w.w,
            major: self.major.clone()
        }
    }
    pub fn dot(&self, w: Vector4d<T>) -> T {
        self.x * w.x + self.y * w.y + self.z *w.z + self.w * w.w
    }
    pub fn add_item(v: Vector4d<T>, item: T) -> Vector4d<T> {
        Vector4d {
            x: v.x + item,
            y: v.x + item,
            z: v.x + item,
            w: v.x + item,
            major: v.major.clone()
        }
    }
    pub fn sub_item(v: Vector4d<T>, item: T) -> Vector4d<T> {
        Vector4d {
            x: v.x - item,
            y: v.x - item,
            z: v.x - item,
            w: v.x - item,
            major: v.major.clone()
        }
    }
    pub fn mul_item(v: Vector4d<T>, item: T) -> Vector4d<T> {
        Vector4d {
            x: v.x * item,
            y: v.x * item,
            z: v.x * item,
            w: v.x * item,
            major: v.major.clone()
        }
    }
    pub fn div_item(v: Vector4d<T>, item: T) -> Vector4d<T> {
        Vector4d {
            x: v.x / item,
            y: v.x / item,
            z: v.x / item,
            w: v.x / item,
            major: v.major.clone()
        }
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