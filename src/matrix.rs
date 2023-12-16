use std::{error, fmt, ops};

const MATRIX2D_SIZE_INVALID_ERROR_MSG: &str = "The length of matrix2d is illegal. The legal length is: 'n' == 4";
const MATRIX3D_SIZE_INVALID_ERROR_MSG: &str = "The length of matrix3d is illegal. The legal length is: 'n' == 9";
const MATRIX4D_SIZE_INVALID_ERROR_MSG: &str = "The length of matrix4d is illegal. The legal length is: 'n' == 16";

#[derive(Debug, Clone)]
pub struct Matrix2dSizeInvalidError;

#[derive(Debug, Clone)]
pub struct Matrix3dSizeInvalidError;

#[derive(Debug, Clone)]
pub struct Matrix4dSizeInvalidError;

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
    x: T,
    y: T
}

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
    x: T,
    y: T,
    z: T
}

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
    x: T,
    y: T,
    z: T,
    w: T
}

#[derive(Debug, Clone)]
pub struct Matrix2d<T>([[T; 2]; 2])
    where T:
    fmt::Debug +
    Copy +
    PartialOrd +
    PartialEq +
    ops::Add<Output=T> +
    ops::Mul<Output=T> +
    ops::Div<Output=T> +
    ops::Sub<Output=T>;

#[derive(Debug, Clone)]
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
    x: [T; 3],
    y: [T; 3],
    z: [T; 3]
}

#[derive(Debug, Clone)]
pub struct Matrix4d<T>
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
    x: [T; 4],
    y: [T; 4],
    z: [T; 4],
    w: [T; 4]
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
        Vector2d{x, y}
    }
    pub fn add(&mut self, w: Vector2d<T>) {
        self.x = self.x + w.x;
        self.y = self.y + w.y;
    }
    pub fn mul(&mut self, w: Vector2d<T>) {
        self.x = self.x * w.x;
        self.y = self.y * w.y;
    }
    pub fn div(&mut self, w: Vector2d<T>) {
        self.x = self.x / w.x;
        self.y = self.y / w.y;
    }
    pub fn sub(&mut self, w: Vector2d<T>) {
        self.x = self.x - w.x;
        self.y = self.y - w.y;
    }
    pub fn dot(&mut self, w: Vector2d<T>) -> T {
        self.x * w.x + self.y * w.y
    }
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
        Vector3d{x, y, z}
    }
    pub fn add(&mut self, w: Vector3d<T>) {
        self.x = self.x + w.x;
        self.y = self.y + w.y;
        self.z = self.z + w.z;
    }
    pub fn mul(&mut self, w: Vector3d<T>) {
        self.x = self.x * w.x;
        self.y = self.y * w.y;
        self.z = self.z * w.z;
    }
    pub fn div(&mut self, w: Vector3d<T>) {
        self.x = self.x / w.x;
        self.y = self.y / w.y;
        self.z = self.z / w.z;
    }
    pub fn sub(&mut self, w: Vector3d<T>) {
        self.x = self.x - w.x;
        self.y = self.y - w.y;
        self.z = self.z - w.z;
    }
    pub fn dot(&mut self, w: Vector3d<T>) -> T {
        self.x * w.x + self.y * w.y + self.z *w.z
    }
    pub fn cross(&self, w: Vector3d<T>) -> Vector3d<T> {
        Vector3d {
            x: self.y * w.z - w.y - self.z,
            y: self.z * w.x - w.z * self.x,
            z: self.x * w.y - w.x * self.z
        }
    }
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
        Vector4d{x, y, z, w}
    }
    pub fn add(&mut self, w: Vector4d<T>) {
        self.x = self.x + w.x;
        self.y = self.y + w.y;
        self.z = self.z + w.z;
        self.w = self.w + w.w;
    }
    pub fn mul(&mut self, w: Vector4d<T>) {
        self.x = self.x * w.x;
        self.y = self.y * w.y;
        self.z = self.z * w.z;
        self.w = self.w * w.w;
    }
    pub fn div(&mut self, w: Vector4d<T>) {
        self.x = self.x / w.x;
        self.y = self.y / w.y;
        self.z = self.z / w.z;
        self.w = self.w / w.w;
    }
    pub fn sub(&mut self, w: Vector4d<T>) {
        self.x = self.x - w.x;
        self.y = self.y - w.y;
        self.z = self.z - w.z;
        self.w = self.w - w.w;
    }
    pub fn dot(&mut self, w: Vector4d<T>) -> T {
        self.x * w.x + self.y * w.y + self.z *w.z + self.w * w.w
    }
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
    pub fn new(v: Vec<T>) -> Result<Matrix2d<T>, Matrix2dSizeInvalidError> {
        if v.len() > 4 {
            return Err(Matrix2dSizeInvalidError);
        }
        Ok(
            Matrix2d{
                0: [
                    [v[0].clone(), v[1].clone()],
                    [v[2].clone(), v[3].clone()]
                ]
            }
        )
    }
    pub fn add(&mut self, m: Matrix2d<T>) {
        self.0[0][0] = self.0[0][0] + m.0[0][0];
        self.0[0][1] = self.0[0][1] + m.0[0][1];
        self.0[1][0] = self.0[1][0] + m.0[1][0];
        self.0[1][1] = self.0[1][1] + m.0[1][1];
    }
    pub fn sub(&mut self, m: Matrix2d<T>) {
        self.0[0][0] = self.0[0][0] - m.0[0][0];
        self.0[0][1] = self.0[0][1] - m.0[0][1];
        self.0[1][0] = self.0[1][0] - m.0[1][0];
        self.0[1][1] = self.0[1][1] - m.0[1][1];
    }
    pub fn product(&mut self, m: Matrix2d<T>) {
        self.0[0][0] = self.0[0][0] * m.0[0][0];
        self.0[0][1] = self.0[0][1] * m.0[0][1];
        self.0[1][0] = self.0[1][0] * m.0[1][0];
        self.0[1][1] = self.0[1][1] * m.0[1][1];
    }
    pub fn div(&mut self, m: Matrix2d<T>) {
        self.0[0][0] = self.0[0][0] / m.0[0][0];
        self.0[0][1] = self.0[0][1] / m.0[0][1];
        self.0[1][0] = self.0[1][0] / m.0[1][0];
        self.0[1][1] = self.0[1][1] / m.0[1][1];
    }
    pub fn cross(&self, m: Matrix2d<T>) -> Matrix2d<T> {
        Matrix2d {
            0: [
                [self.0[0][0] * m.0[0][0], self.0[0][1] * m.0[1][0]],
                [self.0[1][0] * m.0[0][0], self.0[0][1] * m.0[1][0]],
            ]
        }
    }
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
    pub fn new(v: Vec<T>) -> Result<Matrix3d<T>, Matrix3dSizeInvalidError> {
        if v.len() > 9 {
            return Err(Matrix3dSizeInvalidError);
        }
        Ok(Matrix3d{
            x: [v[0].clone(), v[1].clone(), v[2].clone()],
            y: [v[3].clone(), v[4].clone(), v[5].clone()],
            z: [v[6].clone(), v[7].clone(), v[8].clone()],
        })
    }
    pub fn add(&mut self, m: Matrix3d<T>) {
        self.x[0] = self.x[0] + m.x[0];
        self.x[1] = self.x[1] + m.x[1];
        self.y[0] = self.y[0] + m.y[0];
        self.y[1] = self.y[1] + m.y[1];
        self.z[0] = self.z[0] + m.z[0];
        self.z[1] = self.z[1] + m.z[1];
    }
    pub fn sub(&mut self, m: Matrix3d<T>) {
        self.x[0] = self.x[0] - m.x[0];
        self.x[1] = self.x[1] - m.x[1];
        self.y[0] = self.y[0] - m.y[0];
        self.y[1] = self.y[1] - m.y[1];
        self.z[0] = self.z[0] - m.z[0];
        self.z[1] = self.z[1] - m.z[1];
    }
    pub fn product(&mut self, m: Matrix3d<T>) {
        self.x[0] = self.x[0] * m.x[0];
        self.x[1] = self.x[1] * m.x[1];
        self.y[0] = self.y[0] * m.y[0];
        self.y[1] = self.y[1] * m.y[1];
        self.z[0] = self.z[0] * m.z[0];
        self.z[1] = self.z[1] * m.z[1];
    }
    pub fn div(&mut self, m: Matrix3d<T>) {
        self.x[0] = self.x[0] / m.x[0];
        self.x[1] = self.x[1] / m.x[1];
        self.y[0] = self.y[0] / m.y[0];
        self.y[1] = self.y[1] / m.y[1];
        self.z[0] = self.z[0] / m.z[0];
        self.z[1] = self.z[1] / m.z[1];
    }
}

impl<T> Matrix4d<T>
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
    pub fn new(v: Vec<T>) -> Result<Matrix4d<T>, Matrix4dSizeInvalidError> {
        if v.len() > 16 {
            return Err(Matrix4dSizeInvalidError);
        }
        Ok(Matrix4d{
            x: [v[0].clone(), v[1].clone(), v[2].clone(), v[3].clone()],
            y: [v[4].clone(), v[5].clone(), v[6].clone(), v[7].clone()],
            z: [v[8].clone(), v[9].clone(), v[10].clone(), v[11].clone()],
            w: [v[12].clone(), v[13].clone(), v[14].clone(), v[15].clone()]
        })
    }
    pub fn add(&mut self, m: Matrix4d<T>) {
        self.x[0] = self.x[0] + m.x[0];
        self.x[1] = self.x[1] + m.x[1];
        self.y[0] = self.y[0] + m.y[0];
        self.y[1] = self.y[1] + m.y[1];
        self.z[0] = self.z[0] + m.z[0];
        self.z[1] = self.z[1] + m.z[1];
        self.w[0] = self.w[0] + m.w[0];
        self.w[1] = self.w[1] + m.w[1];
    }
    pub fn sub(&mut self, m: Matrix4d<T>) {
        self.x[0] = self.x[0] - m.x[0];
        self.x[1] = self.x[1] - m.x[1];
        self.y[0] = self.y[0] - m.y[0];
        self.y[1] = self.y[1] - m.y[1];
        self.z[0] = self.z[0] - m.z[0];
        self.z[1] = self.z[1] - m.z[1];
        self.w[0] = self.w[0] - m.w[0];
        self.w[1] = self.w[1] - m.w[1];
    }
    pub fn product(&mut self, m: Matrix4d<T>) {
        self.x[0] = self.x[0] * m.x[0];
        self.x[1] = self.x[1] * m.x[1];
        self.y[0] = self.y[0] * m.y[0];
        self.y[1] = self.y[1] * m.y[1];
        self.z[0] = self.z[0] * m.z[0];
        self.z[1] = self.z[1] * m.z[1];
        self.w[0] = self.w[0] * m.w[0];
        self.w[1] = self.w[1] * m.w[1];
    }
    pub fn div(&mut self, m: Matrix4d<T>) {
        self.x[0] = self.x[0] / m.x[0];
        self.x[1] = self.x[1] / m.x[1];
        self.y[0] = self.y[0] / m.y[0];
        self.y[1] = self.y[1] / m.y[1];
        self.z[0] = self.z[0] / m.z[0];
        self.z[1] = self.z[1] / m.z[1];
        self.w[0] = self.w[0] / m.w[0];
        self.w[1] = self.w[1] / m.w[1];
    }
}

impl fmt::Display for Matrix2dSizeInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", MATRIX2D_SIZE_INVALID_ERROR_MSG)
    }
}

impl error::Error for Matrix2dSizeInvalidError {}

impl fmt::Display for Matrix3dSizeInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", MATRIX3D_SIZE_INVALID_ERROR_MSG)
    }
}

impl error::Error for Matrix3dSizeInvalidError {}

impl fmt::Display for Matrix4dSizeInvalidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", MATRIX4D_SIZE_INVALID_ERROR_MSG)
    }
}

impl error::Error for Matrix4dSizeInvalidError {}