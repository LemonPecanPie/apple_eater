use macroquad::prelude::*;
use std::{
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

enum TreeVariety {
    Apple,
    Banana,
}

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

    // let mut box_x: f32 = 0.;
    // let mut box_y: f32 = 0.;
    let mut box_vec = vec3(0., 0., 0.);
    let mut apple_x: f32 = rand::gen_range(-BORDER_Z, BORDER_Z);
    let mut apple_y: f32 = rand::gen_range(-BORDER_X, BORDER_X);
    const SPEED: f32 = 0.1;
    const BORDER_Z: f32 = 200.0;
    const BORDER_X: f32 = 100.0;
    let mut size: f32 = 2.0;

    let mut most_recent_left_or_right = -1;
    let mut most_recent_up_or_down = -1;

    let mut mr_lr = -1;

    let mut score: i32 = 0;

    const CAMERA_RANGE: f32 = 50.0;
    let mut cam_position = vec3(0., 0., 0.);
    cam_position.x = box_vec.x - 30.;
    cam_position.z = box_vec.z - 1.;
    cam_position.y = 15.;
    // update_camera_position(&mut cam_position, CAMERA_RANGE);

    let mut cam_angle_vec: Vec2 = vec2(0., 0.);
    const CAM_ANGLE_SPEED: f32 = 0.01;

    const SCORE_THRESHOLD: i32 = 5;

    const APPLE_SIZE: f32 = 1.0;

    let mut tree_positions = vec![];
    for i in 0..10 {
        tree_positions.push((
            rand::gen_range(-BORDER_X, BORDER_X),
            rand::gen_range(-BORDER_Z, BORDER_Z),
        ));
    }

    let mut projectiles: Vec<(Vec2, Vec3, u32)> = vec![];
    const PROJECTILE_SPEED: f32 = 0.4;

    loop {
        // if is_key_down(KeyCode::Left) {
        //     rect_x -= 1.0;
        // }
        // if is_key_down(KeyCode::Right)

        clear_background(SKYBLUE);

        if is_key_pressed(KeyCode::Left) {
            mr_lr = 0;
        } else if is_key_pressed(KeyCode::Right) {
            mr_lr = 1;
        } else if is_key_down(KeyCode::Left) && !is_key_down(KeyCode::Right) {
            mr_lr = 0;
        } else if is_key_down(KeyCode::Right) && !is_key_down(KeyCode::Left) {
            mr_lr = 1;
        } else if !is_key_down(KeyCode::Left) && !is_key_down(KeyCode::Right) {
            mr_lr = -1;
        }

        let mut cam_angle = cam_angle_vec.to_angle();

        if mr_lr == 0 {
            // mov_vec.x -= 1.;
            cam_angle -= CAM_ANGLE_SPEED;

            // println!("{:?}", cam_angle);
            // println!("{:?}", cam_angle);
            // println!("{:?}", Vec2::from_angle(cam_angle));
            // cam_position.x = box_vec.x - (Vec2::from_angle(cam_angle).x * cam_position.x) * CAMERA_RANGE;
        } else if mr_lr == 1 {
            cam_angle += CAM_ANGLE_SPEED;
            // mov_vec.x += 1.;
            // cam_position.x = Vec2::from_angle(cam_angle).rotate(vec2(cam_position.x, cam_position.z)).x;
            // cam_position.z = box_vec.z - (Vec2::from_angle(cam_angle).y) * CAMERA_RANGE;
        }

        cam_angle_vec = Vec2::from_angle(cam_angle);
        cam_position.x = box_vec.x - (cam_angle_vec.x) * CAMERA_RANGE;
        cam_position.z = box_vec.z - (cam_angle_vec.y) * CAMERA_RANGE;

        set_camera(&Camera3D {
            position: vec3(cam_position.x, cam_position.y, cam_position.z),
            up: vec3(0.0, 1.0, 0.0),
            // target: vec3(0.0, 0.0, 0.0),
            target: box_vec,
            ..Default::default()
        });

        // draw_grid((BORDER as u32) * 2, 1.0, BLACK, GRAY);
        draw_plane(
            vec3(0., 0., 0.),
            vec2(BORDER_X * 1.1, BORDER_Z * 1.1),
            None,
            DARKGREEN,
        );

        draw_cube(
            vec3(apple_y, 1., apple_x),
            vec3(APPLE_SIZE, APPLE_SIZE, APPLE_SIZE),
            None,
            RED,
        );

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

        // println!("c: {:?}", cam_angle_vec);

        if !(mov_vec.x == 0. && mov_vec.y == 0.) {
            mov_vec = mov_vec.normalize();
        }

        // mov_vec = cam_angle_vec.rotate(mov_vec) * SPEED;

        // println!("m: {:?}", mov_vec);

        let dist_vec = cam_position - box_vec;
        let dist_vec_2d = vec2(dist_vec.z, dist_vec.x);
        let ang = dist_vec_2d.normalize();

        // println!("{:?}", ang);

        mov_vec = ang.rotate(mov_vec).perp();

        for elem in &mut projectiles {
            draw_cube(elem.1, vec3(1., 1., 1.), None, YELLOW);
            elem.1.z += elem.0.x;
            elem.1.x += elem.0.y;
            elem.2 += 1;
        }

        for i in (0..projectiles.len()).rev() {
            if projectiles[i].2 == 300 {
                projectiles.remove(i);
            }
        }

        let facing_vec = vec2(1., 0.);
        let projectile_mov_vec = ang.rotate(facing_vec).perp().perp();

        if is_key_pressed(KeyCode::Space) {
            projectiles.push((
                projectile_mov_vec * PROJECTILE_SPEED,
                vec3(box_vec.x, 1.0 / 2., box_vec.z),
                0,
            ));
        }
        mov_vec *= SPEED;

        box_vec.z += mov_vec.x;
        box_vec.x += mov_vec.y;

        box_vec.z = box_vec.z.clamp(-BORDER_Z, BORDER_Z);
        box_vec.x = box_vec.x.clamp(-BORDER_X, BORDER_X);

        // println!("{}", box_x);

        draw_cube(
            vec3(box_vec.x, size / 2., box_vec.z),
            vec3(size, size, size),
            None,
            ORANGE,
        );

        if is_touching(
            box_vec,
            vec3(size, size, size),
            vec3(apple_y, 0., apple_x),
            vec3(APPLE_SIZE, APPLE_SIZE, APPLE_SIZE),
        ) {
            apple_x = rand::gen_range(-BORDER_Z, BORDER_Z);
            apple_y = rand::gen_range(-BORDER_X, BORDER_X);
            size += 0.25;
            score += 1;
            if score % SCORE_THRESHOLD == 0 {
                // update_camera_position(&mut cam_position, CAMERA_RANGE);
            }
        }

        if is_key_pressed(KeyCode::R) {
            size = 2.0;
        }

        // draw_tree(0., 0.);
        // draw_tree(10., 40.);
        for elem in &tree_positions {
            draw_tree(elem.0, elem.1, None, None, None, Some(TreeVariety::Banana));
        }

        // draw_farmer(0.,0.);
        draw_farmer(10.0, 50.0);

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

// fn is_touching(box_x: f32, box_y: f32, apple_x: f32, apple_y: f32, size: f32) -> bool {
//     (box_x - apple_x).abs() < (size / 2. + 0.5) && (box_y - apple_y).abs() < (size / 2. + 0.5)
// }

fn is_touching(pos1: Vec3, siz1: Vec3, pos2: Vec3, siz2: Vec3) -> bool {
    (pos1.x - pos2.x).abs() < (siz1.x / 2. + siz2.x / 2.)
        && (pos1.y - pos2.y).abs() < (siz1.y / 2. + siz2.y / 2.)
        && (pos1.z - pos2.z).abs() < (siz1.z / 2. + siz2.z / 2.)
}

fn update_camera_position(cam_position: &mut Vec3, camera_range: f32) {
    cam_position.x = rand::gen_range(-camera_range, -10.);
    cam_position.y = rand::gen_range(10., camera_range);
    cam_position.z = rand::gen_range(-2., 2.);
}

fn draw_tree(
    x: f32,
    z: f32,
    tree_height: Option<f32>,
    tree_width: Option<f32>,
    leaf_width: Option<f32>,
    variety: Option<TreeVariety>,
) {
    let tree_height = tree_height.unwrap_or(7.0);
    let tree_width = tree_width.unwrap_or(3.0);
    let leaf_width = leaf_width.unwrap_or(5.0);
    let v = variety.unwrap_or(TreeVariety::Apple);
    let trunk_color;
    let leaf_color;
    match v {
        TreeVariety::Apple => {
            trunk_color = BROWN;
            leaf_color = GREEN;
        }
        TreeVariety::Banana => {
            trunk_color = Color::new(0.95, 0.72, 0.31, 1.);
            leaf_color = LIME;
        }
    }
    draw_cube(
        vec3(x, tree_height / 2., z),
        vec3(tree_width, tree_height, tree_width),
        None,
        trunk_color,
    );
    draw_cube(
        vec3(x, tree_height + (leaf_width / 2.), z),
        vec3(leaf_width, leaf_width, leaf_width),
        None,
        leaf_color,
    );
}

fn draw_farmer(x: f32, z: f32) {
    const FARMER_SIZE: f32 = 3.0;
    const HAT_WIDTH: f32 = 5.0;
    const HAT_HEIGHT: f32 = 1.0;
    draw_cube(
        vec3(x, FARMER_SIZE / 2., z),
        vec3(FARMER_SIZE, FARMER_SIZE, FARMER_SIZE),
        None,
        MAROON,
    );
    draw_cube(
        vec3(x, FARMER_SIZE + HAT_HEIGHT / 2., z),
        vec3(HAT_WIDTH, HAT_HEIGHT, HAT_WIDTH),
        None,
        BEIGE,
    );
}

// fn is_touching(pos_1: Vec3, siz_1: Vec3, pos_2: Vec3, siz_2: Vec3) {

// }
