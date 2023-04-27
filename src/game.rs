use std::num::Wrapping;

use sdl2::{
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{TextureCreator, TextureQuery, WindowCanvas},
    ttf::Font,
    video::WindowContext,
};

const PADDLE_LENGTH: usize = 5;

pub struct Game {
    pub pixel_width: usize,
    pub pixel_height: usize,
    pub pixel_size: usize,

    keys: u8,
    left_paddle_pos: usize,
    right_paddle_pos: usize,
    ball_pos: (usize, usize),
    ball_vel: (bool, bool),
    score: (usize, usize),
}

impl Game {
    pub fn new(pixel_width: usize, pixel_height: usize, pixel_size: usize) -> Game {
        Game {
            pixel_width,
            pixel_height,
            pixel_size,
            keys: 0,
            left_paddle_pos: pixel_height / 2,
            right_paddle_pos: pixel_height / 2,
            score: (0, 0),
            ball_pos: (pixel_width / 2, pixel_height / 2),
            ball_vel: (false, false),
        }
    }

    pub fn dispatch_key(&mut self, keycode: Keycode, pressed: bool) {
        let mask = match keycode {
            Keycode::W => 1 << 0,
            Keycode::S => 1 << 1,
            Keycode::Up => 1 << 2,
            Keycode::Down => 1 << 3,
            _ => 0b0,
        };
        if pressed {
            self.keys |= mask;
        } else {
            self.keys &= !mask;
        }
    }

    pub fn update(&mut self) {
        // Paddles
        if self.keys & (1 << 0) != 0 && self.left_paddle_pos > PADDLE_LENGTH / 2 {
            self.left_paddle_pos -= 1;
        } else if self.keys & (1 << 1) != 0
            && self.left_paddle_pos < self.pixel_height - PADDLE_LENGTH / 2 - 1
        {
            self.left_paddle_pos += 1;
        }
        if self.keys & (1 << 2) != 0 && self.right_paddle_pos > PADDLE_LENGTH / 2 {
            self.right_paddle_pos -= 1;
        } else if self.keys & (1 << 3) != 0
            && self.right_paddle_pos < self.pixel_height - PADDLE_LENGTH / 2 - 1
        {
            self.right_paddle_pos += 1;
        }

        // Ball
        if self.ball_vel.0 {
            if self.ball_pos.0 > 0 {
                self.ball_pos.0 -= 1;
            } else {
                self.ball_vel.0 = false;
            }
        } else {
            if self.ball_pos.0 < self.pixel_width - 1 {
                self.ball_pos.0 += 1;
            } else {
                self.ball_vel.0 = true;
            }
        }

        if self.ball_vel.1 {
            if self.ball_pos.1 > 0 {
                self.ball_pos.1 -= 1;
            } else {
                self.ball_vel.1 = false;
            }
        } else {
            if self.ball_pos.1 < self.pixel_height - 1 {
                self.ball_pos.1 += 1;
            } else {
                self.ball_vel.1 = true;
            }
        }

        // Collision
        if self.ball_pos.0 == 1
            && self.ball_pos.1 >= self.left_paddle_pos - PADDLE_LENGTH / 2
            && self.ball_pos.1 <= self.left_paddle_pos + PADDLE_LENGTH / 2
        {
            self.ball_vel.0 = false;
        } else if self.ball_pos.0 == self.pixel_width - 2
            && self.ball_pos.1 >= self.right_paddle_pos - PADDLE_LENGTH / 2
            && self.ball_pos.1 <= self.right_paddle_pos + PADDLE_LENGTH / 2
        {
            self.ball_vel.0 = true;
        }

        // Score
        if self.ball_pos.0 == 0 && self.ball_vel.0 {
            self.score.1 += 1;
        } else if self.ball_pos.0 == self.pixel_width - 1 && !self.ball_vel.0 {
            self.score.0 += 1;
        }
    }

    pub fn draw(
        &self,
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
    ) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        self.draw_score(canvas, texture_creator, font, self.score.0, self.score.1);
        self.draw_paddle(canvas, 1, self.left_paddle_pos);
        self.draw_paddle(canvas, self.pixel_width - 2, self.right_paddle_pos);
        self.draw_ball(canvas, self.ball_pos.0, self.ball_pos.1);
        canvas.present();
    }

    fn draw_ball(&self, canvas: &mut WindowCanvas, x: usize, y: usize) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .fill_rect(Rect::new(
                x as i32 * self.pixel_size as i32,
                y as i32 * self.pixel_size as i32,
                self.pixel_size as u32,
                self.pixel_size as u32,
            ))
            .unwrap();
    }

    fn draw_paddle(&self, canvas: &mut WindowCanvas, x: usize, y: usize) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .fill_rect(Rect::new(
                x as i32 * self.pixel_size as i32,
                (y - 2) as i32 * self.pixel_size as i32,
                self.pixel_size as u32,
                (self.pixel_size * 5) as u32,
            ))
            .unwrap();
    }

    fn draw_score(
        &self,
        canvas: &mut WindowCanvas,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
        left_score: usize,
        right_score: usize,
    ) {
        let surface = font
            .render(&format!("{}-{}", left_score, right_score).to_owned())
            .blended(Color::RGB(255, 255, 255))
            .unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let hw = Wrapping::<i32>((self.pixel_width as i32 / 2i32) * self.pixel_size as i32)
            - Wrapping::<i32>(width as i32 / 2i32);
        let target = Rect::new(hw.0, 10, width as u32, height as u32);

        canvas.copy(&texture, None, Some(target)).unwrap();
    }
}
