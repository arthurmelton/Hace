use enigo::*;
use rand::Rng;
use std::{thread, time::Duration};

extern crate captrs;

use captrs::*;
// game width, game height, game offset x, game offset y, screen number
static mut GAME: (i32, i32, i32, i32, i32) = (1920, 1080, 0, 0, 0);
static mut MOUSE: (f32, f32) = (0.0, 0.0);
// x and y are floats bc it is the % from top left so 1,1 is top right, 0.5,0.5 middle of screen

const MOUSE_MOVE_COUNT_OFFSET: f32 = 13.0;

pub fn move_mouse(x: f32, y: f32, change_availability_x: f32, change_availability_y: f32) {
    unsafe {
        let mut rng = rand::thread_rng();
        let mut enigo = Enigo::new();
        let new_pos = (
            (GAME.0 as f32 * x) as i32
                + GAME.2
                + rng.gen_range(
                    (GAME.0 as f32 * change_availability_x * -1.0).round() as i32
                        ..(GAME.0 as f32 * change_availability_x).round() as i32 + 1,
                ),
            GAME.1 - (GAME.1 as f32 * y) as i32
                + GAME.3
                + rng.gen_range(
                    (GAME.0 as f32 * change_availability_y * -1.0).round() as i32
                        ..(GAME.0 as f32 * change_availability_y).round() as i32 + 1,
                ),
        );
        let old_pos = (
            (GAME.0 as f32 * MOUSE.0) as i32 + GAME.2,
            GAME.1 - (GAME.1 as f32 * MOUSE.1) as i32 + GAME.3,
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
        for i in 0..(distance.round() / MOUSE_MOVE_COUNT_OFFSET).round() as i32 {
            let mut distance_from_center =
                i - ((distance.round() / MOUSE_MOVE_COUNT_OFFSET).round() as i32 / 2);
            if distance_from_center < 0 {
                distance_from_center *= -1;
            }
            last += (1.0
                - (distance_from_center as f32
                    / (distance.round() / MOUSE_MOVE_COUNT_OFFSET / 2.0)))
                * 2.0;
            enigo.mouse_move_to(
                ((new_old_x / (distance.round() / MOUSE_MOVE_COUNT_OFFSET).round() as i32) as f32
                    * last)
                    .round() as i32
                    + rng.gen_range(-1..2)
                    + old_pos.0,
                ((new_old_y / (distance.round() / MOUSE_MOVE_COUNT_OFFSET).round() as i32) as f32
                    * last)
                    .round() as i32
                    + rng.gen_range(-1..2)
                    + old_pos.1,
            );
            thread::sleep(Duration::from_millis(8));
        }
        enigo.mouse_move_to(new_pos.0, new_pos.1);
        MOUSE = (x, y);
    }
}

pub fn write(text: &str) {
    let mut enigo = Enigo::new();
    let mut rng = rand::thread_rng();
    for i in text.chars() {
        enigo.key_down(Key::Layout(i));
        thread::sleep(Duration::from_millis(rng.gen_range(60..101)));
        // Doing it this way will not work on other keyboard layouts so idk how to fix it without the user providing the right way
        enigo.key_up(Key::Layout(i));
        thread::sleep(Duration::from_millis(rng.gen_range(60..101)));
    }
}

pub fn down_key(character:char) {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Layout(character));
}

pub fn up_key(character:char) {
    let mut enigo = Enigo::new();
    enigo.key_up(Key::Layout(character));
}

pub fn init(width: Option<i32>, height: Option<i32>, offset_x: Option<i32>, offset_y: Option<i32>, screen: Option<i32>) {
    unsafe {
        let mut enigo = Enigo::new();
        if width.is_some() {GAME.0 = width.unwrap();}
        if height.is_some() {GAME.1 = height.unwrap();}
        if offset_x.is_some() {GAME.2 = offset_x.unwrap();}
        if offset_y.is_some() {GAME.3 = offset_y.unwrap();}
        if screen.is_some() {GAME.4 = screen.unwrap();}
        enigo.mouse_move_to(GAME.2, GAME.3+GAME.1);
    }
}

pub fn up_right_click() {
    let mut enigo = Enigo::new();
    enigo.mouse_up(MouseButton::Right);
}

pub fn up_left_click() {
    let mut enigo = Enigo::new();
    enigo.mouse_up(MouseButton::Left);
}

pub fn down_left_click() {
    let mut enigo = Enigo::new();
    enigo.mouse_down(MouseButton::Left);
}

pub fn down_right_click() {
    let mut enigo = Enigo::new();
    enigo.mouse_down(MouseButton::Right);
}

pub fn left_click() {
    let mut enigo = Enigo::new();
    let mut rng = rand::thread_rng();
    enigo.mouse_down(MouseButton::Left);
    thread::sleep(Duration::from_millis(rng.gen_range(60..101)));
    enigo.mouse_up(MouseButton::Left);
}

pub fn right_click() {
    let mut enigo = Enigo::new();
    let mut rng = rand::thread_rng();
    enigo.mouse_down(MouseButton::Right);
    thread::sleep(Duration::from_millis(rng.gen_range(60..101)));
    enigo.mouse_up(MouseButton::Right);
}

pub fn get_color(x:f32, y:f32) -> (u8,u8,u8) {
    unsafe{
        let pos = (
            (GAME.0 as f32 * x) as i32
                + GAME.2,
            GAME.1 - (GAME.1 as f32 * y) as i32
                + GAME.3,
        );
        let mut capturer = Capturer::new(GAME.4.try_into().unwrap()).unwrap();
        let h = capturer.geometry().1;
        let ps = capturer.capture_frame().unwrap();
    
        let Bgr8 { r, g, b, .. } = ps.into_iter().nth(((pos.1 as u32*h)+pos.0 as u32).try_into().unwrap()).unwrap();
        return (r,g,b);
    }
}
