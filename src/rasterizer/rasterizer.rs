use crate::matrix::{vector3d::Vector3d, matrix4d::Matrix4d};
use std::collections::HashMap;
use crate::geometry::geometry_errors::TriangleError;
use crate::geometry::triangle::Triangle;
use crate::matrix::vector4d::Vector4d;
use std::f32::consts::PI;
use crate::matrix::matrix3d::Matrix3d;

pub enum Primitive {
    Line,
    Triangle,
}

pub trait BasicRasterizer {
    fn view_matrix(eye_pos: Vector3d<f32>) -> Matrix4d<f32>;
    fn model_matrix(norm: Vector3d<f32>, angle: f32) -> Matrix4d<f32>;
    fn projection_matrix(eye_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Matrix4d<f32>;
}

pub struct Rasterizer {
    pub model:         Matrix4d<f32>,
    pub view:          Matrix4d<f32>,
    pub projection:    Matrix4d<f32>,

    pos_buf:       HashMap<usize, Vec<Vector3d<f32>>>,
    ind_buf:       HashMap<usize, Vec<Vector3d<i32>>>,

    pub frame_buf:     Vec<Vector3d<f32>>,
    pub depth_buf:     Vec<f32>,

    pub width:     usize,
    pub height:    usize,
    next_id:       u32
}

impl Rasterizer {
    /// To initialize a raster renderer, the model view and projection matrix need to be filled using function a, function b and function c.
    pub fn new(width: usize, height: usize) -> Rasterizer {
        let pos_buf: HashMap<usize, Vec<Vector3d<f32>>> = HashMap::new();
        let ind_buf: HashMap<usize, Vec<Vector3d<i32>>> = HashMap::new();
        let mut frame_buf: Vec<Vector3d<f32>> = Vec::new();
        frame_buf.resize(width * height, Vector3d::fill(0.0_f32));
        let mut depth_buf: Vec<f32> = Vec::new();
        depth_buf.resize(width * height, 0.0_f32);

        Rasterizer {
            model: Matrix4d::fill(0.0_f32),
            view: Matrix4d::fill(0.0_f32),
            projection: Matrix4d::fill(0.0_f32),
            pos_buf,
            ind_buf,
            frame_buf,
            depth_buf,
            width,
            height,
            next_id: 0
        }
    }

    fn get_next_id(&mut self) -> u32 {
        let res = self.next_id;
        self.next_id += 1;
        res
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        (self.height - y) * self.width + x
    }

    pub fn load_positions(&mut self, positions: Vec<Vector3d<f32>>) -> usize {
        let id = self.get_next_id() as usize;
        self.pos_buf.insert(id, positions);
        id
    }

    pub fn load_indices(&mut self, indices: Vec<Vector3d<i32>>) -> usize {
        let id = self.get_next_id() as usize;
        self.ind_buf.insert(id, indices);
        id
    }

    pub fn set_model(&mut self, m: Matrix4d<f32>) {
        self.model = m;
    }

    pub fn set_view(&mut self, m: Matrix4d<f32>) {
        self.view = m;
    }

    pub fn set_projection(&mut self, m: Matrix4d<f32>) {
        self.projection = m;
    }

    pub fn clear_frame_buf(&mut self) {
        let length = self.frame_buf.len();
        self.frame_buf = vec![Vector3d::fill(0.0_f32); length]
    }

    pub fn clear_depth_buf(&mut self) {
        let length = self.depth_buf.len();
        self.depth_buf = vec![0.0_f32; length]
    }

    pub fn clear_buf(&mut self) {
        self.clear_frame_buf();
        self.clear_depth_buf();
    }

    pub fn set_pixel(&mut self, point: Vector3d<f32>, color: Vector3d<f32>) {
        if point.x < 0.0_f32 || point.x >= self.width as f32 || point.y < 0.0_f32 || point.y >= self.height as f32 {
            return;
        }
        let ind = (self.height as f32 - point.y) * self.width as f32 + point.x;
        if ind >= (self.width * self.height) as f32 {
            return;
        }
        self.frame_buf[ind as usize] = color;
    }

    pub fn draw_line(&mut self, begin: Vector3d<f32>, end: Vector3d<f32>, line_color: Vector3d<f32>) {
        let mut point1 = Vector3d::new(0.0_f32, 0.0_f32, 0.0_f32);
        let mut point2 = Vector3d::new(0.0_f32, 0.0_f32, 0.0_f32);
        let distance = end.sub(begin);
        let (mut dx, mut dy) = (f32::abs(distance.x), f32::abs(distance.y));
        let (mut px, mut py) = (2.0_f32 * dy - dx, 2.0_f32 * dx - dy);

        if dy <= dx {
            if distance.x >= 0.0_f32 {
                point1.x = begin.x;
                point1.y = begin.y;
                point2.x = end.x;
            }else {
                point1.x = end.x;
                point1.y = end.y;
                point2.x = begin.x;
            }
            self.set_pixel(point1, line_color);
            for _ in point1.x as usize .. point2.x as usize {
                point1.x = point1.x + 1.0_f32;
                if px < 0.0_f32 {
                    px = px + 2.0_f32 * dy;
                }else {
                    if (distance.x < 0.0_f32 && distance.y < 0.0_f32) || (distance.x > 0.0_f32 && distance.y > 0.0_f32)
                    {
                        point1.y = point1.y + 1.0_f32;
                    }else {
                        point1.y = point1.y - 1.0_f32;
                    }
                    px = px + 2.0_f32 * (distance.y - distance.x);
                }
                self.set_pixel(point1, line_color);
            }
        }else {
            if dy >= 0.0_f32 {
                point1.x = begin.x;
                point1.y = begin.y;
                point2.y = end.y;
            }else {
                point1.x = end.x;
                point1.y = end.y;
                point2.y = begin.y;
            }
            self.set_pixel(point1,line_color);
            for _ in point1.y as usize .. point2.y as usize {
                point1.y = point1.y + 1.0_f32;
                if py <= 0.0_f32 {
                    py = py + 2.0_f32 * dx;
                }else {
                    if (distance.x < 0.0_f32 && distance.y < 0.0_f32) || (distance.x > 0.0_f32 && distance.y > 0.0_f32) {
                        point1.x = point1.x + 1.0_f32;
                    }else {
                        point1.x = point1.x - 1.0_f32;
                    }
                    py = py + 2.0_f32 * (dx - dy);
                }
                self.set_pixel(point1,line_color);
            }
        }
    }

    pub fn rasterizer_wireframe(&mut self, triangle: Triangle, line_color: Vector3d<f32>) {
        self.draw_line(triangle.get_vertex_c(), triangle.get_vertex_a(), line_color);
        self.draw_line(triangle.get_vertex_c(), triangle.get_vertex_b(), line_color);
        self.draw_line(triangle.get_vertex_b(), triangle.get_vertex_a(), line_color);
    }

    pub fn draw_triangle(&mut self, pos_id: usize, ind_id: usize, color: Vector3d<f32>) -> Result<(), TriangleError>{
        let buf: Vec<Vector3d<f32>> = match self.pos_buf.get(&pos_id) {
            None => vec![],
            Some(v) => v.to_vec()
        };
        let ind: Vec<Vector3d<i32>> = match self.ind_buf.get(&ind_id) {
            None => vec![],
            Some(v) => v.to_vec()
        };

        let f1 = (100.0_f32 - 0.1_f32) / 2.0_f32;
        let f2 = (100.0_f32 + 0.1_f32) / 2.0_f32;

        let mvp = self.projection * self.view * self.model;

        for ind_vec in ind.iter() {
            let mut t = Triangle::new();
            let v0 = match buf.get(ind_vec.x as usize) {
                None => {
                    Err(
                        TriangleError {
                            err_code: 3001,
                            message: format!("index: {}, length: {}", ind_vec.x, buf.len())
                        }
                    )
                },
                Some(buf_vec) => {
                    let v = mvp.product_with_vector4d(buf_vec.to_vector4d(1.0_f32));
                    let mut v = Vector4d::div_item(v, v.w);
                    v.x = 0.5_f32 * self.width as f32 * (v.x + 1.0_f32);
                    v.y = 0.5_f32 * self.height as f32 * (v.y + 1.0_f32);
                    v.z = v.z * f1 + f2;
                    Ok(v)
                }
            }?;
            let v1 = match buf.get(ind_vec.y as usize) {
                None => {
                    Err(
                        TriangleError {
                            err_code: 3001,
                            message: format!("index: {}, length: {}", ind_vec.x, buf.len())
                        }
                    )
                },
                Some(buf_vec) => {
                    let v = mvp.product_with_vector4d(buf_vec.to_vector4d(1.0_f32));
                    let mut v = Vector4d::div_item(v, v.w);
                    v.x = 0.5_f32 * self.width as f32 * (v.x + 1.0_f32);
                    v.y = 0.5_f32 * self.height as f32 * (v.y + 1.0_f32);
                    v.z = v.z * f1 + f2;
                    Ok(v)
                }
            }?;
            let v2 = match buf.get(ind_vec.z as usize) {
                None => {
                    Err(
                        TriangleError {
                            err_code: 3001,
                            message: format!("index: {}, length: {}", ind_vec.x, buf.len())
                        }
                    )
                },
                Some(buf_vec) => {
                    let v = mvp.product_with_vector4d(buf_vec.to_vector4d(1.0_f32));
                    let mut v = Vector4d::div_item(v, v.w);
                    v.x = 0.5_f32 * self.width as f32 * (v.x + 1.0_f32);
                    v.y = 0.5_f32 * self.height as f32 * (v.y + 1.0_f32);
                    v.z = v.z * f1 + f2;
                    Ok(v)
                }
            }?;

            t.set_vertex(0, v0.head3()).expect("draw triangle panic\n");
            t.set_vertex(1, v1.head3()).expect("draw triangle panic\n");
            t.set_vertex(2, v2.head3()).expect("draw triangle panic\n");

            t.set_color(0, color.x, 0.0_f32, 0.0_f32).expect("draw triangle panic\n");
            t.set_color(1, 0.0_f32, color.y, 0.0_f32).expect("draw triangle panic\n");
            t.set_color(2, 0.0_f32, 0.0_f32, color.z).expect("draw triangle panic\n");

            self.rasterizer_wireframe(t, color);
        }
        Ok(())
    }
}

impl BasicRasterizer for Rasterizer {
    fn view_matrix(eye_pos: Vector3d<f32>) -> Matrix4d<f32> {
        Matrix4d::new(
            Vector4d::new(1.0_f32, 0.0_f32, 0.0_f32, eye_pos.x * -1.0_f32),
            Vector4d::new(0.0_f32, 1.0_f32, 0.0_f32, eye_pos.y * -1.0_f32),
            Vector4d::new(0.0_f32, 0.0_f32, 1.0_f32, eye_pos.z * -1.0_f32),
            Vector4d::new(0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32),
        ) * Matrix4d::identity(1.0_f32)
    }

    fn model_matrix(norm: Vector3d<f32>, angle: f32) -> Matrix4d<f32> {
        let r_angle = angle / 180.0_f32 * PI;
        let r_model = Matrix3d::new(
            Vector3d::new(0.0_f32, -1.0_f32 * norm.z, norm.y),
            Vector3d::new(norm.z, 0.0_f32, -1.0_f32 * norm.x),
            Vector3d::new(-1.0_f32 * norm.y, norm.x, 0.0_f32),
        );
        let rod = r_model.mul_item((1.0_f32 - f32::cos(r_angle)) * norm.dot(norm) + f32::sin(r_angle))
            .add(Matrix3d::identity(1.0_f32).mul_item(f32::cos(r_angle)));

        Matrix4d::new(
            rod.items[0].to_vector4d(0.0_f32),
            rod.items[1].to_vector4d(0.0_f32),
            rod.items[2].to_vector4d(0.0_f32),
            Vector4d::new(0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32)
        )
    }

    fn projection_matrix(eye_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Matrix4d<f32> {
        let t = f32::tan(eye_fov / 180.0_f32 * f32::acos(-1.0_f32) / 2.0_f32) * z_near;
        let b = -t;
        let r = aspect_ratio * t;
        let l = -r;

        let persp = Matrix4d::new(
            Vector4d::new(z_near, 0.0_f32, 0.0_f32, 0.0_f32),
            Vector4d::new(0.0_f32, z_near, 0.0_f32, 0.0_f32),
            Vector4d::new(0.0_f32, 0.0_f32, z_near + z_far, -1.0_f32 * z_near * z_far),
            Vector4d::new(0.0_f32, 0.0_f32, 1.0_f32, 0.0_f32),
        );

        let trans = Matrix4d::new(
            Vector4d::new(2.0_f32 / (r - l), 0.0_f32, 0.0_f32, 0.0_f32),
            Vector4d::new(0.0_f32, 2.0_f32 / (t - b), 0.0_f32, 0.0_f32),
            Vector4d::new(0.0_f32, 0.0_f32, 2.0_f32/(z_near-z_far), 0.0_f32),
            Vector4d::new(0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32),
        );

        let scale = Matrix4d::new(
            Vector4d::new(1.0_f32, 0.0_f32, 0.0_f32, -1.0_f32 * (r + l) / 2.0_f32),
            Vector4d::new(0.0_f32, 1.0_f32, 0.0_f32, -1.0_f32 * (t + b) / 2.0_f32),
            Vector4d::new(0.0_f32, 0.0_f32, 1.0_f32, -1.0_f32 * (z_near + z_far) / 2.0_f32),
            Vector4d::new(0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32),
        );

        trans * scale * persp
    }
}