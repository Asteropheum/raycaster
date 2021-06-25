use rand::Rng;
use std::f32::consts::PI;
use std::fs::{File, create_dir_all};
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

fn draw_rectangle(
    image: &mut Vec<u32>,
    img_width: usize,
    img_height: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    color: u32,
) {
    assert_eq!(image.len(), img_width * img_height);
    for i in 0..w {
        for j in 0..h {
            let cx = x + i;
            let cy = y + j;
            if (cx >= img_width) || (cy >= img_height) {
                continue;
            }
            image[cx + cy * img_width] = color;
        }
    }
}

fn main() {
    let window_width = 1024;
    let window_height = 512;

    let map_width = 16;
    let map_height = 16;
    let map: Vec<char> = "0000222222220000\
                          1              0\
                          1      11111   0\
                          1     0        0\
                          0     0  1110000\
                          0     3        0\
                          0   10000      0\
                          0   0   11100  0\
                          0   0   0      0\
                          0   0   1  00000\
                          0       1      0\
                          2       1      0\
                          0       0      0\
                          0 0000000      0\
                          0              0\
                          0002222222200000"
        .chars()
        .collect();

    assert_eq!(map.len(), map_width * map_width);

    let player_x: f32 = 3.456;
    let player_y: f32 = 2.345;
    let player_a_degree: f32 = 90.0;
    let mut player_a: f32 = player_a_degree * PI / 180.0; // angle between view direction and x axis
    let player_fov_degree: f32 = 60.0;
    let player_fov: f32 = player_fov_degree * PI / 180.0;

    let mut frame_buffer: Vec<u32> =
        vec![(window_width * window_height) as u32; window_width * window_height];

    let n_colors: usize = 10;
    let mut colors: Vec<u32> = vec![0; n_colors];
    let mut rng = rand::thread_rng();
    for i in colors.iter_mut().take(n_colors) {
        *i = pack_color(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>(), None);
    }

    let rect_w = window_width / (map_width * 2);
    let rect_h = window_height / map_height;

    let output_dir = "./out/";
    create_dir_all(output_dir).expect("can not create output directory");
    for frame in 0..360 {
        player_a += 2. * PI / 360.;
        frame_buffer = vec![pack_color(255, 255, 255, None); window_width * window_height];

        // Draw map
        for j in 0..map_height {
            for i in 0..map_width {
                if map[i + j * map_width] == ' ' {
                    continue;
                }
                let rect_x = i * rect_w;
                let rect_y = j * rect_h;
                let r_color: usize = map[i + j * map_width].to_digit(10).unwrap() as usize;
                assert!(r_color < n_colors);
                draw_rectangle(
                    &mut frame_buffer,
                    window_width,
                    window_height,
                    rect_x,
                    rect_y,
                    rect_w,
                    rect_h,
                    colors[r_color],
                );
            }
        }

        // Draw rays
        let rays_number = window_width / 2;
        let starting_angle = player_a - player_fov / 2.0;
        for i in 0..rays_number {
            let offset = player_fov * i as f32 / rays_number as f32;
            let angle = starting_angle + offset;

            for t in 0..4000 {
                // Slide point along the ray by varying t value
                let t = t as f32 * 0.01;
                let cx = player_x + t * angle.cos();
                let cy = player_y + t * angle.sin();

                let pix_x = cx * rect_w as f32;
                let pix_y = cy * rect_h as f32;
                frame_buffer[pix_x as usize + pix_y as usize * window_width] =
                    pack_color(160, 160, 160, None);

                if map[cx as usize + cy as usize * map_width] != ' ' {
                    let r_color = map[cx as usize + cy as usize * map_width]
                        .to_digit(10)
                        .unwrap() as usize;
                    assert!(r_color < n_colors);
                    let column_height =
                        (window_height as f32 / (t * (angle - player_a).cos())) as usize;
                    draw_rectangle(
                        &mut frame_buffer,
                        window_width,
                        window_height,
                        window_width / 2 + i,
                        window_height / 2 - column_height / 2,
                        1,
                        column_height,
                        colors[r_color],
                    );
                    break;
                }
            }
        }

        let filename = format!("{}frame{:04}.ppm", output_dir, frame);
        drop_ppm_image(
            filename.as_str(),
            &frame_buffer,
            window_width,
            window_height,
        );
    }
}
