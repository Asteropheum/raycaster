mod frame_buffer;
mod map;
mod player;
mod texture;
mod utils;

use crate::frame_buffer::FrameBuffer;
use crate::map::Map;
use crate::player::Player;
use crate::texture::Texture;
use crate::utils::{drop_ppm_image, pack_color};
use std::f32::consts::PI;
use std::fs::create_dir_all;

fn wall_x_texcoord(x: f32, y: f32, tex_walls: Texture) -> i32 {
    let hit_x = (x - (x + 0.5).floor()) as i32; // hit_x and hit_y contain (signed) fractional parts of x and y,
    let hit_y = (y - (y + 0.5).floor()) as i32; // they vary between -0.5 and +0.5, and one of them is supposed to be very close to 0
    let mut tex = hit_x * tex_walls.size as i32;
    if hit_y.abs() > hit_x.abs() {
        // we need to determine whether we hit a "vertical" or a "horizontal" wall (w.r.t the map)
        tex = hit_y * tex_walls.size as i32;
    }
    if tex < 0 {
        // do not forget x_texcoord can be negative, fix that
        tex += hit_y * tex_walls.size as i32;
    }
    assert!(tex >= 0 && tex < tex_walls.size as i32);
    tex
}

fn render(frame_buffer: &mut FrameBuffer, map: &Map, player: &Player, wall_texture: &Texture) {
    frame_buffer.clear(pack_color(255, 255, 255, None));

    let rect_w = frame_buffer.width / (map.width * 2) as i32;
    let rect_h = frame_buffer.height / map.height;

    // Draw MAP
    for j in 0..map.height {
        for i in 0..map.width {
            if map.is_empty(i, j) {
                continue;
            }
            let rect_x = i as i32 * rect_w;
            let rect_y = j as i32 * rect_h;

            let texture_id = map.get(i, j);
            assert!(texture_id < wall_texture.count);

            frame_buffer.draw_rectangle(
                rect_x,
                rect_y,
                rect_w,
                rect_h,
                wall_texture
                    .get(0, 0, texture_id)
                    .expect("can not get pixel"),
            );
        }
    }

    // Draw rays
    let rays_number = frame_buffer.width / 2;
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
                assert!(texture_id < wall_texture.count);

                let column_height =
                    (frame_buffer.height as f32 / (t * (angle - player.a).cos())) as usize;

                let hit_x = cx - (cx + 0.5).floor(); // take fractional part of x
                let hit_y = cy - (cy + 0.5).floor(); // take fractional part of y

                // If we multiply fractional part by texture size,
                // we'll get corresponding column in the texture image.
                let mut x_texture_coord: i64 = (hit_x * wall_texture.size as f32) as i64;
                if hit_y.abs() > hit_x.abs() {
                    // We also have vertical walls, i.e. the walls where
                    // the hit_x will be close to zero. The texture column
                    // is defined by hit_y in this case.
                    x_texture_coord = (hit_y * wall_texture.size as f32) as i64;
                }

                if x_texture_coord < 0 {
                    x_texture_coord += wall_texture.size as i64;
                }
                assert!(x_texture_coord >= 0 && x_texture_coord < wall_texture.size as i64);

                let column = wall_texture
                    .get_scaled_column(texture_id, x_texture_coord as u32, column_height as u32)
                    .expect("can not get column");

                pix_x = frame_buffer.width as i32 / 2 + i as i32;

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
}

fn main() {
    let map = Map::new(16, 16);

    let player = Player {
        x: 3.456,
        y: 2.345,
        a: 90.0 * PI / 180.0,
        fov: 60.0 * PI / 180.0,
    };

    let mut frame_buffer = FrameBuffer::new(
        1024,
        512,
        vec![pack_color(255, 255, 255, None); (1024 * 512) as usize],
    );

    let wall_texture = Texture::new("./resources/walltext.png").expect("can not load texture");

    render(&mut frame_buffer, &map, &player, &wall_texture);

    let output_dir = "./out/";
    create_dir_all(output_dir).expect("can not create output directory");

    drop_ppm_image(
        "./out/out.ppm",
        &*frame_buffer.image,
        frame_buffer.width as usize,
        frame_buffer.height as usize,
    );
}
