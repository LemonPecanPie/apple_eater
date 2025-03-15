use macroquad::prelude::*;
use std::{
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

#[macroquad::main("Apple Eater")]
async fn main() {
    // let mut rect_x = screen_width() / 2.0 - 60.0;
    // let mut rect_y = 100.0;

    // const SPEED: f64 = 1.0;
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    rand::srand(since_the_epoch.as_micros() as u64);

    let mut box_x: f32 = 0.;
    let mut box_y: f32 = 0.;
    let mut apple_x: f32 = rand::gen_range(-BORDER, BORDER);
    let mut apple_y: f32 = rand::gen_range(-BORDER, BORDER);
    const SPEED: f32 = 0.1;
    const BORDER: f32 = 10.0;
    let mut size: f32 = 2.0;

    let mut most_recent_left_or_right = -1;
    let mut most_recent_up_or_down = -1;

    let mut score: i32 = 0;

    const CAMERA_RANGE: f32 = 50.0;
    let mut cam_position = vec3(0.,0.,0.);
    update_camera_position(&mut cam_position, CAMERA_RANGE);

    const SCORE_THRESHOLD: i32 = 5;

    loop {
        // if is_key_down(KeyCode::Left) {
        //     rect_x -= 1.0;
        // }
        // if is_key_down(KeyCode::Right)

        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: vec3(cam_position.x, cam_position.y, cam_position.z),
            up: vec3(0.0, 1.0, 0.0),
            target: vec3(0.0, 0.0, 0.0),
            ..Default::default()
        });

        draw_grid(20, 1.0, BLACK, GRAY);

        draw_cube(vec3(apple_y, 1., apple_x), vec3(1., 1., 1.), None, RED);

        // draw_cube_wires(vec3(0., 1.0, -6.0), vec3(2., 2., 2.), DARKGREEN);
        // draw_cube_wires(vec3(0., 1., 6.), vec3(3., 2., 5.), DARKBLUE);
        // draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), YELLOW);

        // draw_plane(vec3(-8., 0., -8.), vec2(5., 5.), None, WHITE);

        let mut mov_vec = vec2(0., 0.);

        // let keys_pressed = get_keys_pressed();
        // let keys_down = get_keys_down();

        // if keys_pressed != HashSet::new() {
        //     println!("{:?}", keys_pressed);

        // }

        if is_key_pressed(KeyCode::A) {
            most_recent_left_or_right = 0;
        } else if is_key_pressed(KeyCode::D) {
            most_recent_left_or_right = 1;
        } else if is_key_down(KeyCode::A) && !is_key_down(KeyCode::D) {
            most_recent_left_or_right = 0;
        } else if is_key_down(KeyCode::D) && !is_key_down(KeyCode::A) {
            most_recent_left_or_right = 1;
        } else if !is_key_down(KeyCode::A) && !is_key_down(KeyCode::D) {
            most_recent_left_or_right = -1;
        }

        // println!("POTATO");
        if is_key_pressed(KeyCode::S) {
            // println!("POTATO");
            most_recent_up_or_down = 0;
        } else if is_key_pressed(KeyCode::W) {
            most_recent_up_or_down = 1;
            // println!("hi");
        } else if is_key_down(KeyCode::S) && !is_key_down(KeyCode::W) {
            most_recent_up_or_down = 0;
        } else if is_key_down(KeyCode::W) && !is_key_down(KeyCode::S) {
            most_recent_up_or_down = 1;
        } else if !is_key_down(KeyCode::S) && !is_key_down(KeyCode::W) {
            most_recent_up_or_down = -1;
        }

        if most_recent_left_or_right == 0 {
            mov_vec.x -= 1.;
        } else if most_recent_left_or_right == 1 {
            mov_vec.x += 1.;
        }
        if most_recent_up_or_down == 0 {
            mov_vec.y -= 1.;
        } else if most_recent_up_or_down == 1 {
            mov_vec.y += 1.;
        }

        if !(mov_vec.x == 0. && mov_vec.y == 0.) {
            mov_vec = mov_vec.normalize() * SPEED;
        }
        box_x += mov_vec.x;
        box_y += mov_vec.y;

        box_x = box_x.clamp(-BORDER, BORDER);
        box_y = box_y.clamp(-BORDER, BORDER);

        // println!("{}", box_x);

        draw_cube(vec3(box_y, 1., box_x), vec3(size, size, size), None, ORANGE);

        if is_touching(box_x, box_y, apple_x, apple_y, size) {
            apple_x = rand::gen_range(-BORDER, BORDER);
            apple_y = rand::gen_range(-BORDER, BORDER);
            size += 0.25;
            score += 1;
            if score % SCORE_THRESHOLD == 0 {
                update_camera_position(&mut cam_position, CAMERA_RANGE);
            }
        }

        if is_key_pressed(KeyCode::R) {
            size = 2.0;
        }

        // draw_rectangle(rect_x, rect_y, 40.0, 40.0, GREEN);

        set_default_camera();

        draw_text("HELLO", 10.0, 25.0, 30.0, BLACK);
        draw_text("WASD TO MOVE", 10.0, 50.0, 30.0, BLACK);
        draw_text("R TO RESET SIZE", 10.0, 75.0, 30.0, BLACK);
        draw_text(
            &format!("SCORE: {score}")[..],
            screen_width() - 150.,
            25.0,
            30.0,
            BLACK,
        );

        next_frame().await
    }
}

fn is_touching(box_x: f32, box_y: f32, apple_x: f32, apple_y: f32, size: f32) -> bool {
    (box_x - apple_x).abs() < (size / 2. + 0.5)
        && (box_y - apple_y).abs() < (size / 2. + 0.5)
}

fn update_camera_position(cam_position: &mut Vec3, camera_range: f32) {
    cam_position.x = rand::gen_range(-camera_range, -10.);
    cam_position.y = rand::gen_range(10., camera_range);
    cam_position.z = rand::gen_range(-2., 2.);
}