use std::{fmt, ops};
use crate::matrix::utils::Major;
use crate::matrix::vector_errors::{VectorError, VectorErrorType};

#[derive(Debug, Clone)]
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
    pub fn identity(type_item: T) -> Vector3d<T> {
        Vector3d {
            x: type_item,
            y: type_item,
            z: type_item,
            major: Major::Row
        }
    }
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
    pub fn add(&mut self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.x + w.x,
            y: self.y + w.y,
            z: self.z + w.z,
            major: self.major.clone()
        }
    }
    pub fn mul(&mut self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.x * w.x,
            y: self.y * w.y,
            z: self.z * w.z,
            major: self.major.clone()
        }
    }
    pub fn div(&mut self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.x / w.x,
            y: self.y / w.y,
            z: self.z / w.z,
            major: self.major.clone()
        }
    }
    pub fn sub(&mut self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.x - w.x,
            y: self.y - w.y,
            z: self.z - w.z,
            major: self.major.clone()
        }
    }
    pub fn dot(&mut self, w: Vector3d<T>) -> T {
        self.x * w.x + self.y * w.y + self.z *w.z
    }
    pub fn cross(&self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.y * w.z - w.y - self.z,
            y: self.z * w.x - w.z * self.x,
            z: self.x * w.y - w.x * self.z,
            major: self.major.clone()
        }
    }
    pub fn add_item(mut v: Vector3d<T>, item: T) -> Vector3d<T> {
        Vector3d {
            x: v.x + item,
            y: v.y + item,
            z: v.z + item,
            major: v.major.clone()
        }
    }
    pub fn sub_item(mut v: Vector3d<T>, item: T) -> Vector3d<T> {
        Vector3d {
            x: v.x - item,
            y: v.y - item,
            z: v.z - item,
            major: v.major.clone()
        }
    }
    pub fn mul_item(mut v: Vector3d<T>, item: T) -> Vector3d<T> {
        Vector3d {
            x: v.x * item,
            y: v.y * item,
            z: v.z * item,
            major: v.major.clone()
        }
    }
    pub fn div_item(mut v: Vector3d<T>, item: T) -> Vector3d<T> {
        Vector3d {
            x: v.x / item,
            y: v.y / item,
            z: v.z / item,
            major: v.major.clone()
        }
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