// LIGHT:
//       N        iτtn
// p =   Σ     c e
//  t n=-(N/2)  n
// .
//   p = final point
//   t = time (0..=1)
//   c_n = normalized DFT coeff
//   n ∈ R = frequency

use num_complex::*;
use raylib::prelude::*;

const TRAIL_SIZE: usize = 5000;
const TRAIL_SCALE: f32 = 0.25;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("DFT my beloved")
        .msaa_4x()
        .build();
    rl.set_exit_key(None);

    let font = rl.load_font_ex(&thread, "font.ttf", 20, None).unwrap();
    let mut trail = rl.load_render_texture(&thread, TRAIL_SIZE as _, TRAIL_SIZE as _).unwrap();
    let mut trail2 = rl.load_render_texture(&thread, TRAIL_SIZE as _, TRAIL_SIZE as _).unwrap();

    let mut camera = camera::Camera2D::default();
    camera.zoom = 1.0;
    camera.offset = Vector2::new(rl.get_screen_width() as f32 / 2.0, rl.get_screen_height() as f32 / 2.0);
    let mut lock_on = None;

    let mut t = 0.0;

    let img = load_img();
    let mut coeff = vec![Complex64::default(); img.len()];
    smoldft::compute_dft_to(&mut coeff, &img);
    coeff.iter_mut().for_each(|i| *i /= img.len() as f64);

    let mut last_sum = img[0];

    while !rl.window_should_close() {
        camera.zoom = (camera.zoom + rl.get_mouse_wheel_move_v().y).max(0.1).min(50.0);

        if rl.is_key_pressed(KeyboardKey::KEY_L) {
            lock_on = match lock_on {
                Some(n) if n == img.len() - 1 => None,
                _ => Some(img.len() - 1),
            };
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) && lock_on.is_none() {
            camera.target -= rl.get_mouse_delta() / camera.zoom;
        }

        let mouse_s = rl.get_mouse_position();
        let mouse_w = rl.get_screen_to_world2D(mouse_s, camera);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(36, 39, 58, 255));

        let mut sum = Complex64::default();
        let mut pts = Vec::with_capacity(coeff.len());
        compute_pts(&coeff, t, &mut sum, &mut pts);

        // update camera and focus
        let mut cdst = f32::INFINITY;
        for (i, p) in pts.iter().enumerate() {
            let p = cmplx_to_vec(*p);

            if lock_on == Some(i) {
                camera.target = p;
            }

            if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                let dp = mouse_w - p;
                let dst = dp.x * dp.x + dp.y * dp.y;
                if cdst > dst && dst < 10.0 {
                    lock_on = Some(i);
                    cdst = dst;
                }
            }
        }

        // update trail
        {
            let mut d = d.begin_texture_mode(&thread, &mut trail2);
            d.clear_background(Color::new(166, 218, 149, 0));
            d.draw_texture(&trail, 0, 0, Color::new(255, 255, 255, (d.get_frame_time() * 2000.0).min(255.0) as u8));

            let c = (TRAIL_SIZE / 2) as f32;
            let to_v = |s: Complex64| Vector2::new(s.re as f32 / TRAIL_SCALE + c, -(s.im as f32 / TRAIL_SCALE) + c);
            d.draw_line_ex(to_v(last_sum), to_v(sum), 2.0 / TRAIL_SCALE, Color::new(166, 218, 149, 255));
            last_sum = sum;
        }

        // copy trail from temp texture
        {
            let mut d = d.begin_texture_mode(&thread, &mut trail);
            d.draw_texture(&trail2, 0, 0, Color::new(255, 255, 255, 255));
        }

        // draw world elements
        {
            let mut d = d.begin_mode2D(camera);
            d.draw_line(i32::MIN, 0, i32::MAX, 0, Color::new(73, 77, 100, 255));
            d.draw_line(0, i32::MIN, 0, i32::MAX, Color::new(73, 77, 100, 255));
            let o = -(TRAIL_SIZE as i32 / 2) as f32 * TRAIL_SCALE;
            d.draw_texture_ex(&trail, Vector2::new(o, o), 0.0, TRAIL_SCALE, Color::WHITE);

            if d.is_key_down(KeyboardKey::KEY_S) {
                for p in img.iter() {
                    d.draw_circle_v(cmplx_to_vec(*p), 5.0, Color::new(166, 218, 149, 255));
                }
            }

            let mut last = Vector2::default();
            for (p, c) in ::core::iter::once(&c64(0.0, 0.0)).chain(pts.iter()).zip(coeff.iter()) {
                let dst = (c.re * c.re + c.im * c.im).sqrt();
                unsafe {
                    raylib::ffi::DrawCircleLinesV(
                        cmplx_to_vec(*p).into(),
                        dst as _,
                        Color::new(128, 135, 162, 255).into(),
                    );
                }
            }

            for p in pts.iter() {
                let p = cmplx_to_vec(*p);
                d.draw_line_ex(last, p, 1.5, Color::new(147, 154, 183, 255));
                last = p;
            }

            d.draw_circle_v(cmplx_to_vec(sum), 2.5, Color::new(237, 135, 150, 255));
        }

        // draw ui
        let stat = if let Some(lock) = lock_on {
            format!("t = {t:.02} (camera locked on #{lock})")
        } else {
            format!("t = {t:.02}")
        };
        d.draw_text_ex(&font, &stat, Vector2::new(12.0, 12.0), 20.0, 0.0, Color::new(202, 211, 245, 255));

        let mtip_text = format!("{:.02} + {:.02}i", mouse_w.x, -mouse_w.y);
        let mtip_pos = mouse_s + Vector2::new(15.0, 15.0);
        let mtip_size = font.measure_text(&mtip_text, 20.0, 0.0);
        d.draw_rectangle_v(mtip_pos, mtip_size, Color::new(24, 25, 38, 127));
        d.draw_text_ex(&font, &mtip_text, mtip_pos, 20.0, 0.0, Color::new(147, 154, 183, 255));

        if d.is_key_up(KeyboardKey::KEY_P) {
            t += d.get_frame_time() as f64 * 0.2;
            if t >= 1.0 { t -= 1.0; }
        }
    }
}

fn cmplx_to_vec(c: Complex64) -> Vector2 {
    Vector2::new(c.re as f32, -c.im as f32)
}

fn load_img() -> Vec<Complex64> {
    let f = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    f.lines()
        .map(|l| {
            let (x, y) = l.split_once(' ').unwrap();
            c64(x.parse::<f64>().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn compute_pts(coeff: &[Complex64], t: f64, sum: &mut Complex64, pts: &mut Vec<Complex64>) {
    // let mut cdst = f32::INFINITY;
    for (n, c) in coeff.iter().enumerate() {
        // let psum = cmplx_to_vec(*sum);
        let rn = if n <= coeff.len() / 2 { n as isize } else { n as isize - coeff.len() as isize };
        *sum += c * (Complex64::i() * std::f64::consts::TAU * t * rn as f64).exp();
        // let sum = cmplx_to_vec(*sum);

        // d.draw_line_ex(psum, sum, 1.5, Color::new(54, 58, 79, 255));
        // d.draw_circle_v(sum, 2.5, if lock_on.map_or(false, |i| i == n) {
        //     Color::new(237, 135, 150, 255)
        // } else {
        //     Color::new(73, 77, 100, 255)
        // });

        // if lock_on.map_or(false, |i| i == n) {
        //     camera.target = sum;
        // }

        // if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        //     let dst = mouse_w.distance_to(sum);
        //     if cdst > dst && dst < 10.0 {
        //         lock_on = Some(n);
        //         cdst = dst;
        //     }
        // }

        pts.push(*sum);
    }
}
