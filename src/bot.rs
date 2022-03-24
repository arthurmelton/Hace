use enigo::*;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::{thread, time::Duration};

extern crate captrs;

use captrs::*;

pub struct Bot {
    enigo: enigo::Enigo,
    // x and y are floats bc it is the % from top left so 1,1 is top right, 0.5,0.5 middle of screen
    mouse: (f32, f32),
    // game width, game height, game offset x, game offset y, screen number
    game: (i32, i32, i32, i32, i32),
    move_count_offset: f32,
    rng: ThreadRng,
    capturer: Capturer,
}

impl Bot {
    #[must_use]
    pub fn new() -> Bot {
        Bot {
            enigo: Enigo::new(),
            mouse: (0.0, 0.0),
            game: (1920, 1080, 0, 0, 0),
            move_count_offset: 13.0,
            rng: rand::thread_rng(),
            capturer: Capturer::new(0).unwrap(),
        }
    }

    pub fn set_game_size(&mut self, x: i32, y: i32) {
        self.game.0 = x;
        self.game.1 = y;
    }

    pub fn set_game_offset(&mut self, x: i32, y: i32) {
        self.game.2 = x;
        self.game.3 = y;
    }

    pub fn set_game_screen(&mut self, i: i32) {
        self.game.4 = i;
        self.capturer = Capturer::new(i.try_into().unwrap()).unwrap();
    }

    pub fn start(&mut self) {
        self.enigo
            .mouse_move_to(self.game.2, self.game.3 + self.game.1);
    }

    pub fn write(&mut self, text: &str) {
        for i in text.chars() {
            self.enigo.key_down(Key::Layout(i));
            thread::sleep(Duration::from_millis(self.rng.gen_range(60..101)));
            // Doing it this way will not work on other keyboard layouts so idk how to fix it without the user providing the right way
            self.enigo.key_up(Key::Layout(i));
            thread::sleep(Duration::from_millis(self.rng.gen_range(60..101)));
        }
    }

    pub fn get_color(&mut self, x: f32, y: f32) -> (u8, u8, u8) {
        let pos = (
            (self.game.0 as f32 * x) as i32 + self.game.2,
            self.game.1 - (self.game.1 as f32 * y) as i32 + self.game.3,
        );
        let h = self.capturer.geometry().1;
        let ps = self.capturer.capture_frame().unwrap();

        let Bgr8 { r, g, b, .. } = ps
            .into_iter()
            .nth(((pos.1 as u32 * h) + pos.0 as u32).try_into().unwrap())
            .unwrap();
        (r, g, b)
    }

    pub fn move_mouse(
        &mut self,
        x: f32,
        y: f32,
        change_availability_x: f32,
        change_availability_y: f32,
    ) {
        let new_pos = (
            (self.game.0 as f32 * x) as i32
                + self.game.2
                + self.rng.gen_range(
                    (self.game.0 as f32 * change_availability_x * -1.0).round() as i32
                        ..(self.game.0 as f32 * change_availability_x).round() as i32 + 1,
                ),
            self.game.1 - (self.game.1 as f32 * y) as i32
                + self.game.3
                + self.rng.gen_range(
                    (self.game.0 as f32 * change_availability_y * -1.0).round() as i32
                        ..(self.game.0 as f32 * change_availability_y).round() as i32 + 1,
                ),
        );
        let old_pos = (
            (self.game.0 as f32 * self.mouse.0) as i32 + self.game.2,
            self.game.1 - (self.game.1 as f32 * self.mouse.1) as i32 + self.game.3,
        );
        let mut distance_x = new_pos.0 - old_pos.0;
        if distance_x < 0 {
            distance_x *= -1;
        }
        let mut distance_y = new_pos.1 - old_pos.1;
        if distance_y < 0 {
            distance_y *= -1;
        }
        let distance = (((distance_x * distance_x) + (distance_y * distance_y)) as f32).sqrt();
        let new_old_x = new_pos.0 - old_pos.0;
        let new_old_y = new_pos.1 - old_pos.1;
        let mut last = 0.0;
        for i in 0..(distance.round() / self.move_count_offset).round() as i32 {
            let mut distance_from_center =
                i - ((distance.round() / self.move_count_offset).round() as i32 / 2);
            if distance_from_center < 0 {
                distance_from_center *= -1;
            }
            last += (1.0
                - (distance_from_center as f32
                    / (distance.round() / self.move_count_offset / 2.0)))
                * 2.0;
            self.enigo.mouse_move_to(
                ((new_old_x / (distance.round() / self.move_count_offset).round() as i32) as f32
                    * last)
                    .round() as i32
                    + self.rng.gen_range(-1..2)
                    + old_pos.0,
                ((new_old_y / (distance.round() / self.move_count_offset).round() as i32) as f32
                    * last)
                    .round() as i32
                    + self.rng.gen_range(-1..2)
                    + old_pos.1,
            );
            thread::sleep(Duration::from_millis(8));
        }
        self.enigo.mouse_move_to(new_pos.0, new_pos.1);
        self.mouse = (x, y);
    }

    pub fn down_key(&mut self, character: char) {
        self.enigo.key_down(Key::Layout(character));
    }

    pub fn up_key(&mut self, character: char) {
        self.enigo.key_up(Key::Layout(character));
    }

    pub fn up_right_click(&mut self) {
        self.enigo.mouse_up(MouseButton::Right);
    }

    pub fn up_left_click(&mut self) {
        self.enigo.mouse_up(MouseButton::Left);
    }

    pub fn down_left_click(&mut self) {
        self.enigo.mouse_down(MouseButton::Left);
    }

    pub fn down_right_click(&mut self) {
        self.enigo.mouse_down(MouseButton::Right);
    }

    pub fn left_click(&mut self) {
        self.enigo.mouse_down(MouseButton::Left);
        thread::sleep(Duration::from_millis(self.rng.gen_range(60..101)));
        self.enigo.mouse_up(MouseButton::Left);
    }

    pub fn right_click(&mut self) {
        self.enigo.mouse_down(MouseButton::Right);
        thread::sleep(Duration::from_millis(self.rng.gen_range(60..101)));
        self.enigo.mouse_up(MouseButton::Right);
    }
}
