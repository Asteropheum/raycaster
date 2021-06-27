mod frame_buffer;
mod map;

use crate::frame_buffer::FrameBuffer;
use crate::map::Map;
use std::f32::consts::PI;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};

/// Store each RGBA u8 component in one u32 integer.
fn pack_color(r: u8, g: u8, b: u8, a: Option<u8>) -> u32 {
    (u32::from(a.unwrap_or(255)) << 24) + (u32::from(b) << 16) + (u32::from(g) << 8) + u32::from(r)
}

fn unpack_color(color: &u32, r: &mut u8, g: &mut u8, b: &mut u8, a: &mut u8) {
    *r = (color & 255) as u8;
    *g = ((color >> 8) & 255) as u8;
    *b = ((color >> 16) & 255) as u8;
    *a = ((color >> 24) & 255) as u8;
}

fn drop_ppm_image(filename: &str, image: &[u32], width: usize, height: usize) {
    assert_eq!(image.len(), (width * height) as usize);
    let file = File::create(filename).expect("can not open file");
    let mut writer = BufWriter::new(file);

    write!(writer, "P6\n{} {}\n255\n", width, height).expect("can not write to file");

    for color in image.iter().take(width * height) {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut a = 0;
        unpack_color(color, &mut r, &mut g, &mut b, &mut a);
        writer.write_all(&[r, g, b]).expect("can not write to file");
    }
}

fn load_texture(
    filename: &str,
    texture: &mut Vec<u32>,
    texture_size: &mut usize,
    texture_count: &mut usize,
) -> Result<Vec<u32>, image::ImageError> {
    let rgba_img = image::open(filename)?
        .as_rgba8()
        .expect("Cannot open texture")
        .to_owned();

    let width = rgba_img.width() as usize;
    let height = rgba_img.height() as usize;

    *texture_count = (width / height) as usize;
    *texture_size = width as usize / *texture_count;

    if width != height * *texture_count {
        return Err(image::ImageError::FormatError(String::from(
            "Texture file doesn't contain enough packed square textures",
        )));
    }

    let mut texture = vec![0; width * height];
    let rgba_img = rgba_img.to_vec();

    for j in 0..height {
        for i in 0..width {
            let r = rgba_img[(i + j * width) * 4];
            let g = rgba_img[(i + j * width) * 4 + 1];
            let b = rgba_img[(i + j * width) * 4 + 2];
            let a = rgba_img[(i + j * width) * 4 + 3];
            texture[i + j * width] = pack_color(r, g, b, Some(a));
        }
    }

    Ok(texture)
}

fn texture_column(
    img: &[u32],
    texture_size: usize,
    n_textures: usize,
    texture_id: usize,
    texture_coord: usize,
    column_height: usize,
) -> Vec<u32> {
    let img_w = texture_size * n_textures;
    let img_h = texture_size;
    assert!(img.len() == img_w * img_h && texture_coord < texture_size && texture_id < n_textures);

    let mut column = vec![0; column_height];

    for (y, item) in column.iter_mut().enumerate().take(column_height) {
        let pix_x = texture_id * texture_size + texture_coord;
        let pix_y = (y * texture_size) / column_height;
        *item = img[pix_x + pix_y * img_w];
    }

    column
}

fn main() {
    let window_width = 1024;
    let window_height = 512;

    let map_width = 16;
    let map_height = 16;
    let map = Map::new(map_width, map_height);

    let player_x: f32 = 3.456;
    let player_y: f32 = 2.345;
    let player_a_degree: f32 = 90.0;
    let player_a: f32 = player_a_degree * PI / 180.0; // angle between view direction and x axis
    let player_fov_degree: f32 = 60.0;
    let player_fov: f32 = player_fov_degree * PI / 180.0;

    let mut frame_buffer = FrameBuffer::new(
        window_width,
        window_height,
        vec![pack_color(255, 255, 255, None); (window_width * window_height) as usize],
    );

    let mut wall_texture: Vec<u32> = Vec::new();
    let mut wall_texture_size: usize = 0;
    let mut wall_texture_count: usize = 0;

    wall_texture = match load_texture(
        "./resources/walltext.png",
        &mut wall_texture,
        &mut wall_texture_size,
        &mut wall_texture_count,
    ) {
        Ok(texture) => texture,
        Err(error) => {
            println!("error when loading texture: {}", error);
            vec![pack_color(100, 100, 100, None), 64 * 64 * 6]
        }
    };

    let rect_w = window_width / (map_width * 2) as i32;
    let rect_h = window_height / map_height;

    let output_dir = "./out/";
    create_dir_all(output_dir).expect("can not create output directory");

    // Draw MAP
    for j in 0..map_height {
        for i in 0..map_width {
            if map.is_empty(i, j) {
                continue;
            }
            let rect_x = i as i32 * rect_w;
            let rect_y = j as i32 * rect_h;

            let texture_id: usize = map.get(i, j);
            assert!(texture_id < wall_texture_count);

            frame_buffer.draw_rectangle(
                rect_x,
                rect_y,
                rect_w,
                rect_h,
                wall_texture[texture_id * wall_texture_size],
            );
        }
    }

    // Draw rays
    let rays_number = window_width / 2;
    let starting_angle = player_a - player_fov / 2.0;
    for i in 0..rays_number {
        let offset = player_fov * i as f32 / rays_number as f32;
        let angle = starting_angle + offset;

        for t in 0..1500 {
            // Slide point along the ray by varying t value
            let t = t as f32 * 0.01;
            let cx = player_x + t * angle.cos();
            let cy = player_y + t * angle.sin();

            let mut pix_x = (cx * rect_w as f32) as i32;
            let mut pix_y = (cy * rect_h as f32) as i32;

            frame_buffer.set_pixel(pix_x, pix_y, pack_color(160, 160, 160, None));

            if !map.is_empty(cx as i32, cy as i32) {
                let texture_id = map.get(cx as i32, cy as i32);
                assert!(texture_id < wall_texture_count);

                let column_height =
                    (window_height as f32 / (t * (angle - player_a).cos())) as usize;

                let hit_x = cx - (cx + 0.5).floor(); // take fractional part of x
                let hit_y = cy - (cy + 0.5).floor(); // take fractional part of y

                // If we multiply fractional part by texture size,
                // we'll get corresponding column in the texture image.
                let mut x_texture_coord: i64 = (hit_x * wall_texture_size as f32) as i64;
                if hit_y.abs() > hit_x.abs() {
                    // We also have vertical walls, i.e. the walls where
                    // the hit_x will be close to zero. The texture column
                    // is defined by hit_y in this case.
                    x_texture_coord = (hit_y * wall_texture_size as f32) as i64;
                }

                if x_texture_coord < 0 {
                    x_texture_coord += wall_texture_size as i64;
                }
                assert!(x_texture_coord >= 0 && x_texture_coord < wall_texture_size as i64);

                let column = texture_column(
                    &wall_texture,
                    wall_texture_size,
                    wall_texture_count,
                    texture_id,
                    x_texture_coord as usize,
                    column_height,
                );

                pix_x = window_width as i32 / 2 + i as i32;

                for (j, item) in column.iter().enumerate().take(column_height) {
                    pix_y = j as i32 + window_height as i32 / 2 - column_height as i32 / 2;

                    if pix_y < 0 || pix_y >= window_height as i32 {
                        continue;
                    }

                    frame_buffer.set_pixel(pix_x, pix_y, *item);
                }
                break;
            }
        }
    }

    drop_ppm_image(
        "./out/out.ppm",
        &*frame_buffer.image,
        window_width as usize,
        window_height as usize,
    );
}
