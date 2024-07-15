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

const CIRCLES: usize = 8;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("DFT my beloved")
        .build();
    let font = rl.load_font_ex(&thread, "font.ttf", 20, None).unwrap();

    let mut camera = camera::Camera2D::default();
    camera.zoom = 1.0;
    camera.offset = Vector2::new(rl.get_screen_width() as f32 / 2.0, rl.get_screen_height() as f32 / 2.0);
    let mut lock_on = None;

    let mut t = 0.0;

    let img = &[
        c64(-100.0, -100.0),
        c64(0.0, -100.0),
        c64(100.0, -100.0),
        c64(100.0, 0.0),
        c64(100.0, 100.0),
        c64(0.0, 100.0),
        c64(-100.0, 100.0),
        c64(-100.0, 0.0),
        // c64(100.0 * (0.0 * std::f64::consts::FRAC_PI_3).cos(), 100.0 * (0.0 * std::f64::consts::FRAC_PI_3).sin()),
        // c64(100.0 * (2.0 * std::f64::consts::FRAC_PI_3).cos(), 100.0 * (2.0 * std::f64::consts::FRAC_PI_3).sin()),
        // c64(100.0 * (4.0 * std::f64::consts::FRAC_PI_3).cos(), 100.0 * (4.0 * std::f64::consts::FRAC_PI_3).sin()),
    ];
    let mut coeff = [Complex64::default(); CIRCLES];
    smoldft::compute_dft_to(&mut coeff, img);
    coeff.iter_mut().for_each(|i| *i /= CIRCLES as f64);

    println!("{coeff:?}");

    while !rl.window_should_close() {
        camera.zoom = (camera.zoom + rl.get_mouse_wheel_move_v().y).max(0.1).min(50.0);

        if rl.is_key_pressed(KeyboardKey::KEY_L) {
            lock_on = match lock_on {
                Some(_) => None,
                None => Some(CIRCLES - 1),
            };
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            camera.target -= rl.get_mouse_delta() / camera.zoom;
        }

        let mouse_s = rl.get_mouse_position();
        let mouse_w = rl.get_screen_to_world2D(mouse_s, camera);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(36, 39, 58, 255));
        d.draw_text_ex(&font, &format!("{t:.02} {lock_on:?}"), Vector2::new(12.0, 12.0), 20.0, 0.0, Color::new(202, 211, 245, 255));

        {
            let mut d = d.begin_mode2D(camera);
            d.draw_line(i32::MIN, 0, i32::MAX, 0, Color::new(110, 115, 141, 255));
            d.draw_line(0, i32::MIN, 0, i32::MAX, Color::new(110, 115, 141, 255));

            for p in img.iter() {
                d.draw_circle_v(cmplx_to_vec(*p), 5.0, Color::new(166, 218, 149, 255));
            }

            let mut sum = Complex64::default();
            let mut cdst = f32::INFINITY;

            for (n, c) in coeff.iter().enumerate() {
                let psum = cmplx_to_vec(sum);
                let rn = if n <= CIRCLES / 2 { n as isize } else { n as isize - CIRCLES as isize };
                sum += c * (Complex64::i() * std::f64::consts::TAU * t * rn as f64).exp();
                let sum = cmplx_to_vec(sum);

                d.draw_line_ex(psum, sum, 1.5, Color::new(128, 135, 162, 255));
                d.draw_circle_v(sum, 2.5, if lock_on.map_or(false, |i| i == n) {
                    Color::new(237, 135, 150, 255)
                } else {
                    Color::new(147, 154, 183, 255)
                });

                if lock_on.map_or(false, |i| i == n) {
                    camera.target = sum;
                }

                if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                    let dst = mouse_w.distance_to(sum);
                    if cdst > dst && dst < 10.0 {
                        lock_on = Some(n);
                        cdst = dst;
                    }
                }
            }
        }

        let mtip_text = format!("{:.02} + {:.02}i", mouse_w.x, -mouse_w.y);
        let mtip_pos = mouse_s + Vector2::new(15.0, 15.0);
        let mtip_size = font.measure_text(&mtip_text, 20.0, 0.0);
        d.draw_rectangle_v(mtip_pos, mtip_size, Color::new(24, 25, 38, 127));
        d.draw_text_ex(&font, &mtip_text, mtip_pos, 20.0, 0.0, Color::new(147, 154, 183, 255));

        t += d.get_frame_time() as f64 * 0.2;
        if t >= 1.0 { t -= 1.0; }
    }
}

fn cmplx_to_vec(c: Complex64) -> Vector2 {
    Vector2::new(c.re as f32, -c.im as f32)
}
