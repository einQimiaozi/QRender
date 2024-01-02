use crate::matrix::vector3d::Vector3d;
use crate::rasterizer::rasterizer::{BasicRasterizer, Rasterizer};
use image::{RgbImage, Rgb};

pub fn draw_green_triangle(args: Vec<&str>) {
    let mut angle = 0.0_f32;
    let mut command_line = false;
    let mut filename = "output.png";

    if args.len() >= 3 {
        command_line = true;
        if let Ok(input_angle) = args[2].parse::<f32>() {
            angle = input_angle;
        } else {
            println!("无法将字符串转换为浮点数");
        }
        if args.len() == 4 {
            filename = args[3];
        }
    }

    let mut raster = Rasterizer::new(700, 700);
    let eye_pos = Vector3d::new(0_f32, 0_f32, 5_f32);

    // 这里的参数控制三角形的三个顶点的位置
    // 如果希望看自转，直接把z轴位置设置为0即可
    // std::vector<Eigen::Vector3f> pos{{2, 0, -2}, {0, 2, -2}, {-2, 0, -2}};
    let pos = vec![
        Vector3d::new(1_f32, 0_f32, 0_f32),
        Vector3d::new(0_f32, 1_f32, 0_f32),
        Vector3d::new(-1_f32, 0_f32, 0_f32),
    ];
    let ind = vec![
        Vector3d::new(0, 1, 2)
    ];
    let axis = Vector3d::new(1_f32, 0_f32, 0_f32);

    let pos_id = raster.load_positions(pos);
    let ind_id = raster.load_indices(ind);

    // let (key, frame_count) = (0, 0);

    if command_line {
        raster.clear_buf();

        raster.set_model(Rasterizer::model_matrix(axis, angle));
        raster.set_view(Rasterizer::view_matrix(eye_pos));
        raster.set_projection(Rasterizer::projection_matrix(45_f32, 1_f32, 0.1_f32, 50_f32));

        raster.draw_triangle(pos_id, ind_id, Vector3d::new(0_f32, 255_f32, 0_f32)).unwrap();

        let mut image = RgbImage::new(700, 700);
        for x in 1 .. 700 {
            for y in 1 .. 700 {
                let color = raster.frame_buf[raster.get_index(x as usize, y as usize)];
                image.put_pixel(x, y, Rgb([color.x as u8, color.y as u8, color.z as u8]));
            }
        }
        image.save(filename).unwrap();
        return;
    }
}