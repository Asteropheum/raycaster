mod frame_buffer;
mod map;
mod player;
mod texture;
mod utils;

use crate::frame_buffer::FrameBuffer;
use crate::map::Map;
use crate::player::Player;
use crate::texture::Texture;
use std::f32::consts::PI;
use std::fs::{create_dir_all};
use crate::utils::{drop_ppm_image, pack_color};

fn main() {
    let window_width = 1024;
    let window_height = 512;

    let map_width = 16;
    let map_height = 16;
    let map = Map::new(map_width, map_height);

    let player = Player {
        x: 3.456,
        y: 2.345,
        a: 90.0 * PI / 180.0,
        fov: 60.0 * PI / 180.0,
    };

    let mut frame_buffer = FrameBuffer::new(
        window_width,
        window_height,
        vec![pack_color(255, 255, 255, None); (window_width * window_height) as usize],
    );

    let wall_texture_struct =
        Texture::new("./resources/walltext.png").expect("can not load texture");

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

            let texture_id = map.get(i, j);
            assert!(texture_id < wall_texture_struct.count);

            frame_buffer.draw_rectangle(
                rect_x,
                rect_y,
                rect_w,
                rect_h,
                wall_texture_struct
                    .get(0, 0, texture_id)
                    .expect("can not get pixel"),
            );
        }
    }

    // Draw rays
    let rays_number = window_width / 2;
    let starting_angle = player.a - player.fov / 2.0;
    for i in 0..rays_number {
        let offset = player.fov * i as f32 / rays_number as f32;
        let angle = starting_angle + offset;

        for t in 0..1500 {
            // Slide point along the ray by varying t value
            let t = t as f32 * 0.01;
            let cx = player.x + t * angle.cos();
            let cy = player.y + t * angle.sin();

            let mut pix_x = (cx * rect_w as f32) as i32;
            let mut pix_y = (cy * rect_h as f32) as i32;

            frame_buffer.set_pixel(pix_x, pix_y, pack_color(160, 160, 160, None));

            if !map.is_empty(cx as i32, cy as i32) {
                let texture_id = map.get(cx as i32, cy as i32);
                assert!(texture_id < wall_texture_struct.count);

                let column_height =
                    (window_height as f32 / (t * (angle - player.a).cos())) as usize;

                let hit_x = cx - (cx + 0.5).floor(); // take fractional part of x
                let hit_y = cy - (cy + 0.5).floor(); // take fractional part of y

                // If we multiply fractional part by texture size,
                // we'll get corresponding column in the texture image.
                let mut x_texture_coord: i64 = (hit_x * wall_texture_struct.size as f32) as i64;
                if hit_y.abs() > hit_x.abs() {
                    // We also have vertical walls, i.e. the walls where
                    // the hit_x will be close to zero. The texture column
                    // is defined by hit_y in this case.
                    x_texture_coord = (hit_y * wall_texture_struct.size as f32) as i64;
                }

                if x_texture_coord < 0 {
                    x_texture_coord += wall_texture_struct.size as i64;
                }
                assert!(x_texture_coord >= 0 && x_texture_coord < wall_texture_struct.size as i64);

                let column = wall_texture_struct
                    .get_scaled_column(texture_id, x_texture_coord as u32, column_height as u32)
                    .expect("can not get column");

                pix_x = window_width as i32 / 2 + i as i32;

                for j in 0..column_height {
                    let pix_y = j as i32 + frame_buffer.height / 2 - (column_height / 2) as i32;
                    if pix_y < frame_buffer.height {
                        frame_buffer.set_pixel(pix_x, pix_y as i32, column[j as usize]);
                    }
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
