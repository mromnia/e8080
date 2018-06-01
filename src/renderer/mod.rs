extern crate image as im;
extern crate piston;
extern crate piston_window;

use renderer::piston::event_loop::*;
use renderer::piston::input::*;
use renderer::piston::window::WindowSettings;
use renderer::piston_window::{
    clear, image, G2dTexture, Key, OpenGL, PistonWindow, Texture, TextureSettings,
};

use emulator;

const SIZE_X: u32 = 224;
const SIZE_Y: u32 = 256;

fn calc_addr_in_buffer(x: u32, y: u32) -> (usize, u32) {
    let x_addr = x * (SIZE_Y / 8);

    let y_addr = (SIZE_Y - y) / 8;
    let y_bit = (SIZE_Y - y) % 8;

    let addr = x_addr as usize + y_addr as usize;

    (addr, y_bit)
}

fn get_bit(val: u8, bit: u32) -> u8 {
    (val >> bit) & 0x01
}

pub fn run(emulator: &mut emulator::ArcadeMachine) {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("e8080", [SIZE_X, SIZE_Y])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut canvas = im::ImageBuffer::new(SIZE_X, SIZE_Y);

    for x in 0..SIZE_X {
        for y in 0..SIZE_Y {
            canvas.put_pixel(x, y, im::Rgba([0, 0, 0, 255]));
        }
    }

    let mut texture: G2dTexture =
        Texture::from_image(&mut window.factory, &canvas, &TextureSettings::new()).unwrap();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(Key::C) => emulator.coin_key_toggle(true),
                Button::Keyboard(Key::Left) => emulator.left_p1_key_toggle(true),
                Button::Keyboard(Key::Right) => emulator.right_p1_key_toggle(true),
                Button::Keyboard(Key::Space) => emulator.fire_p1_key_toggle(true),
                Button::Keyboard(Key::S) => emulator.start_p1_key_toggle(true),
                _ => (),
            }
        };

        if let Some(button) = e.release_args() {
            match button {
                Button::Keyboard(Key::C) => emulator.coin_key_toggle(false),
                Button::Keyboard(Key::Left) => emulator.left_p1_key_toggle(false),
                Button::Keyboard(Key::Right) => emulator.right_p1_key_toggle(false),
                Button::Keyboard(Key::Space) => emulator.fire_p1_key_toggle(false),
                Button::Keyboard(Key::S) => emulator.start_p1_key_toggle(false),
                _ => (),
            }
        };

        if let Some(_) = e.render_args() {
            let half_time = 1.0 / 60.0 / 2.0;

            emulator.run(half_time);

            {
                let buff = emulator.get_render_buffer();

                for y in 0..(SIZE_Y / 2) {
                    for x in 0..SIZE_X {
                        let (addr, bit) = calc_addr_in_buffer(x, y);
                        let val = get_bit(buff[addr], bit) * 255;

                        canvas.put_pixel(x, y, im::Rgba([val; 4]));
                    }
                }
            }

            emulator.signal_half_render();
            emulator.run(half_time);

            {
                let buff = emulator.get_render_buffer();

                for y in (SIZE_Y / 2)..SIZE_Y {
                    for x in 0..SIZE_X {
                        let (addr, bit) = calc_addr_in_buffer(x, y);
                        let val = get_bit(buff[addr], bit) * 255;

                        canvas.put_pixel(x, y, im::Rgba([val; 4]));
                    }
                }
            }

            emulator.signal_finish_render();

            texture.update(&mut window.encoder, &canvas).unwrap();
            window.draw_2d(&e, |c, gl| {
                clear([0.0; 4], gl);
                image(&texture, c.transform, gl);
            });
        }
    }
}
