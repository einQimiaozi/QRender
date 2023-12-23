use std::{fmt, ops};
use crate::matrix::matrix_errors::{MatrixError, MatrixErrorType};
use crate::matrix::utils::Major;
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
    pub major: Major
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
    pub fn new(v: Vec<Vector2d<T>>) -> Result<Matrix2d<T>, MatrixError> {
        let mut major = Major::Row;
        if v.len() != 2 {
            return Err(
                MatrixError {
                    err_type: MatrixErrorType::InvalidInitSizeError,
                    err_msg: "A 2D matrix can only be composed of two vectors 2d".to_string()
                }
            )
        }
        for (i, &ref item) in v.iter().enumerate() {
            if i == 0 {
                major = item.major.clone();
            }else {
                if major != item.major.clone() {
                    return Err(
                        MatrixError {
                            err_type: MatrixErrorType::InvalidInitSizeError,
                            err_msg: format!("The row and column types of the input vector must be consistent in line: {}, expect: {:?}, actual: {:?}", i, major, item.major)
                        }
                    )
                }
            }
        }
        Ok (
            Matrix2d {
                items: v.clone(),
                rows: 2,
                cols: 2,
                major: Major::Row
            }
        )
    }
    pub fn identity(item_type: T) -> Matrix2d<T> {
        let mut items = Vec::with_capacity(2);
        items.resize(2, Vector2d::new(item_type, item_type));
        Matrix2d {
            items,
            rows: 2,
            cols: 2,
            major: Major::Row
        }
    }

    pub fn transpose(&mut self) {
        for item in self.items.iter_mut() {
            item.transpose();
        }
        match self.major {
            Major::Row => self.major = Major::Col,
            Major::Col => self.major = Major::Row
        }
    }

    pub fn product(&mut self, mut mat: Matrix2d<T>) -> Matrix2d<T> {
        let mut res = Self::identity(mat.items[0].x - mat.items[0].x);
        let mut mat1 = self.clone();
        let mut mat2 = mat.clone();

        // 保证乘号左边为行存储，右边为列存储
        if mat1.major == Major::Col {
            mat1.transpose();
        }
        if mat2.major == Major::Row {
            mat2.transpose();
        }

        res.items[0].x = mat.items[0].dot(mat2.items[0].clone());
        res.items[0].y = mat.items[0].dot(mat2.items[1].clone());
        res.items[1].x = mat.items[1].dot(mat2.items[0].clone());
        res.items[1].y = mat.items[1].dot(mat2.items[1].clone());

        res
    }

    pub fn add(&mut self, mat: Matrix2d<T>) -> Result<Matrix2d<T>, MatrixError> {
        if self.major != mat.major {
            return Err (
                MatrixError {
                    err_msg: format!("Matrix calculation must be stored in the same way, self: {:?}, params: {:?}", self.major, mat.major),
                    err_type: MatrixErrorType::InvalidCalcError
                }
            )
        }
        Ok (
            Matrix2d {
                items: vec![self.items[0].add(mat.items[0].clone()), self.items[1].add(mat.items[1].clone())],
                rows: 2,
                cols: 2,
                major: self.major.clone()
            }
        )
    }


    pub fn hadamard(&mut self, mat: Matrix2d<T>) -> Result<Matrix2d<T>, MatrixError> {
        if self.major != mat.major {
            return Err (
                MatrixError {
                    err_msg: format!("Matrix calculation must be stored in the same way, self: {:?}, params: {:?}", self.major, mat.major),
                    err_type: MatrixErrorType::InvalidCalcError
                }
            )
        }
        Ok (
            Matrix2d {
                items: vec![self.items[0].mul(mat.items[0].clone()), self.items[1].mul(mat.items[1].clone())],
                rows: 2,
                cols: 2,
                major: self.major.clone()
            }
        )
    }

    pub fn sub(&mut self, mat: Matrix2d<T>) -> Result<Matrix2d<T>, MatrixError> {
        if self.major != mat.major {
            return Err (
                MatrixError {
                    err_msg: format!("Matrix calculation must be stored in the same way, self: {:?}, params: {:?}", self.major, mat.major),
                    err_type: MatrixErrorType::InvalidCalcError
                }
            )
        }
        Ok (
            Matrix2d {
                items: vec![self.items[0].sub(mat.items[0].clone()), self.items[1].sub(mat.items[1].clone())],
                rows: 2,
                cols: 2,
                major: self.major.clone()
            }
        )
    }

    pub fn div(&mut self, mat: Matrix2d<T>) -> Result<Matrix2d<T>, MatrixError> {
        if self.major != mat.major {
            return Err (
                MatrixError {
                    err_msg: format!("Matrix calculation must be stored in the same way, self: {:?}, params: {:?}", self.major, mat.major),
                    err_type: MatrixErrorType::InvalidCalcError
                }
            )
        }
        Ok (
            Matrix2d {
                items: vec![self.items[0].div(mat.items[0].clone()), self.items[1].div(mat.items[1].clone())],
                rows: 2,
                cols: 2,
                major: self.major.clone()
            }
        )
    }

    pub fn add_item(&mut self, item: T) -> Matrix2d<T> {
        Matrix2d {
            items: vec![
                Vector2d::new(self.items[0].x + item, self.items[0].y + item),
                Vector2d::new(self.items[1].x + item, self.items[1].y + item)
            ],
            rows: 2,
            cols: 2,
            major: self.major.clone()
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
            major: self.major.clone()
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
            major: self.major.clone()
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
            major: self.major.clone()
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
        match self.major {
            Major::Row => {
                write!(f, "|[{:?} {:?}]|\n|[{:?} {:?}]|", self.items[0].x, self.items[0].y, self.items[1].x, self.items[1].y)
            },
            Major::Col => {
                write!(f, "|[{:?}] [{:?}]|\n|[{:?}] [{:?}]|", self.items[0].x, self.items[1].x, self.items[0].y, self.items[1].y)
            }
        }
    }
}