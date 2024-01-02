use crate::matrix::vector2d::Vector2d;
use crate::matrix::vector3d::Vector3d;
use crate::geometry::geometry_errors::{TriangleError};
use crate::rasterizer::rasterizer::Primitive;

pub struct Triangle {
    pub vertex: [Vector3d<f32>; 3],
    pub tex_coords: [Vector2d<f32>; 3],
    pub normal:     [Vector3d<f32>; 3],
    pub color:      [Vector3d<f32>; 3],
    pub primitive:  Primitive
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            vertex: [Vector3d::fill(0.0_f32); 3],
            tex_coords: [Vector2d::fill(0.0_f32); 3],
            normal: [Vector3d::fill(0.0_f32); 3],
            color: [Vector3d::fill(0.0_f32); 3],
            primitive: Primitive::Triangle,
        }
    }

    pub fn from_vertex(vertex: [Vector3d<f32>; 3]) -> Triangle {
        Triangle {
            vertex,
            tex_coords: [Vector2d::fill(0.0_f32); 3],
            normal: [Vector3d::fill(0.0_f32); 3],
            color: [Vector3d::fill(0.0_f32); 3],
            primitive: Primitive::Triangle,
        }
    }

    pub fn set_vertex(&mut self, index: usize, vertex: Vector3d<f32>) -> Result<(), TriangleError> {
        if index >= 3 {
            return Err(
                TriangleError {
                    err_code: 1003,
                    message: "".to_string()
                }
            )
        }
        self.vertex[index] = vertex;
        Ok(())
    }

    pub fn set_normal(&mut self, index: usize, normal: Vector3d<f32>) -> Result<(), TriangleError> {
        if index >= 3 {
            return Err(
                TriangleError {
                    err_code: 1003,
                    message: "".to_string()
                }
            )
        }
        self.normal[index] = normal;
        Ok(())
    }

    pub fn set_tex_coords(&mut self, index: usize, tex: Vector2d<f32>) -> Result<(), TriangleError> {
        if index >= 2 {
            return Err(
                TriangleError {
                    err_code: 1002,
                    message: "".to_string()
                }
            )
        }
        self.tex_coords[index] = tex;
        Ok(())
    }

    pub fn set_color(&mut self, index: usize, r: f32, g: f32, b:f32) -> Result<(), TriangleError> {
        if index >= 3 {
            return Err(
                TriangleError {
                    err_code: 1003,
                    message: "".to_string()
                }
            )
        }
        if r < 0.0_f32 || r > 255.0_f32 || g < 0.0_f32 || g > 255.0_f32 || b < 0.0_f32 || b > 255.0_f32 {
            return Err(
                TriangleError {
                    err_code: 2001,
                    message: "".to_string()
                }
            )
        }
        self.color[index] = Vector3d::div_item(Vector3d::new(r, g, b), 255.0_f32);
        Ok(())
    }

    pub fn get_vertex_a(&self) -> Vector3d<f32> {
        self.vertex[0]
    }

    pub fn get_vertex_b(&self) -> Vector3d<f32> {
        self.vertex[1]
    }

    pub fn get_vertex_c(&self) -> Vector3d<f32> {
        self.vertex[2]
    }
}