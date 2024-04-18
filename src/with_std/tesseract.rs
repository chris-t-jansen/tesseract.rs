use std::{
    f64::consts::PI,
    thread::sleep,
    time::Duration,
};

const SCREEN_WIDTH: i16 = 80;
const SCREEN_HEIGHT: i16 = 40;
const SCREEN_SIZE: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

const FROM4: [f64; 4] = [5.0, 0.0, 0.0, 0.0];
const TO4: [f64; 4] =  [0.0, 0.0, 0.0, 0.0];
const UP4: [f64; 4] = [0.0, 1.0, 0.0, 0.0];
const OVER4: [f64; 4] = [0.0, 0.0, 1.0, 0.0];

const FROM3: [f64; 3] = [3.00, 0.99, 1.82];
const TO3: [f64; 3] = [0.0, 0.0, 0.0];
const UP3: [f64; 3] = [0.0, -1.0, 0.0];

#[derive(Clone, Copy)]
struct Coord {
    pub x: i16,
    pub y: i16,
}

impl Coord {

    pub fn default() -> Self {
        Coord { x: 0, y: 0 }
    }

    pub fn from_f64(x_f64: f64, y_f64: f64) -> Self {
        Coord { 
            x: x_f64 as i16,
            y: y_f64 as i16,
        }
    }

    pub fn to_linear_index(&self) -> usize {
        (self.y * SCREEN_WIDTH + self.x) as usize
    }
}

struct Verts {
    v4: [[f64; 4]; 16],
    v3: [[f64; 3]; 16],
    v2: [[f64; 2]; 16],

    indices: [[usize; 2]; 32],
}

impl Verts {
    pub fn default() -> Self {
        Verts {
            v4: [
                [-1.0, -1.0, -1.0, -1.0],
                [ 1.0, -1.0, -1.0, -1.0],
                [-1.0,  1.0, -1.0, -1.0],
                [ 1.0,  1.0, -1.0, -1.0],
                [-1.0, -1.0,  1.0, -1.0],
                [ 1.0, -1.0,  1.0, -1.0],
                [-1.0,  1.0,  1.0, -1.0],
                [ 1.0,  1.0,  1.0, -1.0],
                [-1.0, -1.0, -1.0,  1.0],
                [ 1.0, -1.0, -1.0,  1.0],
                [-1.0,  1.0, -1.0,  1.0],
                [ 1.0,  1.0, -1.0,  1.0],
                [-1.0, -1.0,  1.0,  1.0],
                [ 1.0, -1.0,  1.0,  1.0],
                [-1.0,  1.0,  1.0,  1.0],
                [ 1.0,  1.0,  1.0,  1.0]
            ],

            v3: [[0.0; 3]; 16],
            v2: [[0.0; 2]; 16],

            indices: [
                [0, 1],
                [0, 2],
                [0, 4],
                [1, 3],
                [1, 5],
                [2, 3],
                [2, 6],
                [3, 7],
                [4, 5],
                [4, 6],
                [5, 7],
                [6, 7],

                // in-between lines
                [0,	8],
                [1,	9],
                [2,	10],
                [3,	11],
                [4,	12],
                [5,	13],
                [6,	14],
                [7,	15],

                // cube #2
                [8, 9],
                [8, 10],
                [8, 12],
                [9, 11],
                [9, 13],
                [10, 11],
                [10, 14],
                [11, 15],
                [12, 13],
                [12, 14],
                [13, 15],
                [14, 15],
            ]
        }
    }
}

fn clear_buffer(char_arr: &mut [char; SCREEN_SIZE]) {
    for character in char_arr {
        *character = ' ';
    }
}

fn set_buffer_char(char_arr: &mut [char; SCREEN_SIZE], point: &Coord, character: char) {
    char_arr[point.to_linear_index()] = character;
}

fn calc_coord_char(points: &[Coord; 3], err: f64) -> char {
    if (points[0].y - points[2].y).abs() < 2 {
        if err > 0.5 {
            return '-';
        }
        return '_';
    }

    if (points[0].x - points[2].x).abs() < 2 &&
        (points[0].x >= points[2].x || points[1].x != points[2].x) &&
        (points[0].x <= points[2].x || points[1].x != points[0].x) {
        return '|';
    }

    let m_x = if points[0].y < points[2].y { points[0].x } else { points[2].x };
    if m_x < points[1].x {
        return '\\';
    } else {
        return '/';
    }
}

fn ln(char_arr: &mut [char; SCREEN_SIZE], mut a: Coord, b: Coord) {
    set_buffer_char(char_arr, &a, '@');
    set_buffer_char(char_arr, &b, '@');

    let diff_x: i32 = (b.x - a.x).abs() as i32;
    let diff_y: i32 = (b.y - a.y).abs() as i32;

    let dir_x: i16 = if a.x < b.x { 1 } else { -1 };
    let dir_y: i16 = if a.y < b.y { 1 } else { -1 };

    let mut err: i32 = if diff_x > diff_y { diff_x } else { -diff_y } / 2;
    let mut err_check: i32;

    let mut points = [Coord::default(); 3];
    let mut ers = [f64::default(); 3];

    for i in 0..3 {
        points[i] = a;
        ers[i] = (err - diff_x) as f64 / (diff_y - diff_x) as f64;
        ers[i] = if dir_y == 1 { 1.0 - ers[i] } else { ers[i] };

        if a.x == b.x && a.y == b.y {
            return;
        }

        err_check = err;
        if err_check > -diff_x {
            err -= diff_y;
            a.x += dir_x;
        }

        if err_check < diff_y {
            err += diff_x;
            a.y += dir_y;
        }
    }

    loop {
        set_buffer_char(char_arr, &points[1], calc_coord_char(&points, ers[1]));

        points[0] = points[1];
        points[1] = points[2];
        points[2] = a;

        ers[0] = ers[1];
        ers[1] = ers[2];
        ers[2] = (err - diff_x) as f64 / (diff_y - diff_x) as f64;
        ers[2] = if dir_y == 1 { 1.0 - ers[2] } else { ers[2] };

        if a.x == b.x && a.y == b.y {
            break;
        }

        err_check = err;
        if err_check > -diff_x {
            err -= diff_y;
            a.x += dir_x;
        }
        if err_check < diff_y {
            err += diff_x;
            a.y += dir_y;
        }
    }

    set_buffer_char(char_arr, &points[1], calc_coord_char(&points, ers[1]));
}

fn dot_4(v: [f64; 4], u: [f64; 4]) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..4 {
        result += v[i] * u[i];
    }
    result
}

fn norm_4(v: [f64; 4]) -> f64 {
    dot_4(v, v).sqrt()
}

fn cross_4(result: &mut [f64; 4], u: [f64; 4], v: [f64; 4], w: [f64; 4]) {
    let a: f64 = (v[0] * w[1]) - (v[1] * w[0]);
    let b: f64 = (v[0] * w[2]) - (v[2] * w[0]);
    let c: f64 = (v[0] * w[3]) - (v[3] * w[0]);
    let d: f64 = (v[1] * w[2]) - (v[2] * w[1]);
    let e: f64 = (v[1] * w[3]) - (v[3] * w[1]);
    let f: f64 = (v[2] * w[3]) - (v[3] * w[2]);

    result[0] = (u[1] * f) - (u[2] * e) + (u[3] * d);
    result[1] = -(u[0] * f) + (u[2] * c) - (u[3] * b);
    result[2] = (u[0] * e) - (u[1] * c) + (u[3] * a);
    result[3] = -(u[0] * d) + (u[1] * b) - (u[2] * a);
}

fn vec_sub_4(result: &mut [f64; 4], a: [f64; 4], b: [f64; 4]) {
    for i in 0..4 {
        result[i] = a[i] - b[i];
    }
}

fn vec_scale_4(vec: &mut [f64; 4], m: f64) {
    for i in 0..4 {
        vec[i] *= m;
    }
}

fn mat_vec_mul_4(result: &mut [f64; 4], mat: [f64; 16], vec: [f64; 4]) {
    for row in 0..4 {
        result[row] = 0.0;

        for col in 0..4 {
            result[row] += mat[col * 4 + row] * vec[col];
        }
    }
}

fn rot_x_w_4(result: &mut [f64; 16], t: f64) {
    result[0] = t.cos();
    result[1] = 0.0;
    result[2] = 0.0;
    result[3] = -t.sin();

    result[4] = 0.0;
    result[5] = 1.0;
    result[6] = 0.0;
    result[7] = 0.0;

    result[8] = 0.0;
    result[9] = 0.0;
    result[10] = 1.0;
    result[11] = 0.0;

    result[12] = t.sin();
    result[13] = 0.0;
    result[14] = 0.0;
    result[15] = t.cos();
}

fn view_4(result: &mut [f64; 16]) {
    // column vectors
    let mut w_a: [f64; 4] = [0.0; 4];
    let mut w_b: [f64; 4] = [0.0; 4];
    let mut w_c: [f64; 4] = [0.0; 4];
    let mut w_d: [f64; 4] = [0.0; 4];

    w_a.copy_from_slice(&result[..4]);
    w_b.copy_from_slice(&result[4..(4 + 4)]);
    w_c.copy_from_slice(&result[8..(4 + 8)]);
    w_d.copy_from_slice(&result[12..(4 + 12)]);

    let mut norm: f64;

    // get the normalized Wd column-vector.
    vec_sub_4(&mut w_d, TO4, FROM4);
    norm = norm_4(w_d);
    vec_scale_4(&mut w_d, 1.0 / norm);

    // calculate the normalized Wa column-vector.
    cross_4(&mut w_a, UP4, OVER4, w_d);
    norm = norm_4(w_a);
    vec_scale_4(&mut w_a, 1.0 / norm);

    // calculate the normalized Wb column-vector.
    cross_4(&mut w_b, OVER4, w_d, w_a);
    norm = norm_4(w_b);
    vec_scale_4(&mut w_b, 1.0 / norm);

    // calculate the Wc column-vector.
    cross_4(&mut w_c, w_d, w_a, w_b);

    result[..4].copy_from_slice(&w_a);
    result[4..(4 + 4)].copy_from_slice(&w_b);
    result[8..(4 + 8)].copy_from_slice(&w_c);
    result[12..(4 + 12)].copy_from_slice(&w_d);
}

fn project_to_3d(v_angle: f64, mat_view: [f64; 16], mat_rotation: [f64; 16], verts: &mut Verts) {
    let mut w_a: [f64; 4] = [0.0; 4];
    let mut w_b: [f64; 4] = [0.0; 4];
    let mut w_c: [f64; 4] = [0.0; 4];
    let mut w_d: [f64; 4] = [0.0; 4];

    w_a.copy_from_slice(&mat_view[..4]);
    w_b.copy_from_slice(&mat_view[4..(4 + 4)]);
    w_c.copy_from_slice(&mat_view[8..(4 + 8)]);
    w_d.copy_from_slice(&mat_view[12..(4 + 12)]);

    let mut s: f64;
    let t: f64 = 1.0 / (v_angle / 2.0).tan();

    for i in 0..16 {
        let mut v:[f64; 4] = [0.0; 4];
        mat_vec_mul_4(&mut v, mat_rotation, verts.v4[i]);

        let mut vf: [f64; 4] = [0.0; 4];
        vec_sub_4(&mut vf, v, FROM4);

        s = t / dot_4(vf, w_d);

        verts.v3[i][0] = s * dot_4(vf, w_a);
        verts.v3[i][1] = s * dot_4(vf, w_b);
        verts.v3[i][2] = s * dot_4(vf, w_c);
    }
}

fn dot_3(v: [f64; 3], u: [f64; 3]) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..3 {
        result += v[i] * u[i];
    }
    result
}

fn norm_3(v: [f64; 3]) -> f64 {
    dot_3(v, v).sqrt()
}

fn cross_3(result: &mut [f64; 3], u: [f64; 3], v: [f64; 3]) {
    result[0] = (u[1] * v[2]) - (u[2] * v[1]);
    result[1] = (u[2] * v[0]) - (u[0] * v[2]);
	result[2] = (u[0] * v[1]) - (u[1] * v[0]);
}

fn vec_sub_3(result: &mut [f64; 3], a: [f64; 3], b: [f64; 3]) {
    for i in 0..3 {
        result[i] = a[i] - b[i];
    }
}

fn vec_scale_3(vec: &mut [f64; 3], m: f64) {
    for i in 0..3 {
        vec[i] *= m;
    }
}

fn mat_vec_mul_3(result: &mut [f64; 3], mat: [f64; 9], vec: [f64; 3]) {
    for row in 0..3 {
        result[row] = 0.0;
        for col in 0..3 {
            result[row] += mat[col * 3 + row] * vec[col];
        }
    }
}

fn rot_x_z_3(result: &mut [f64; 9], t: f64) {
    result[0] = t.cos();
    result[1] = 0.0;
    result[2] = -t.sin();

    result[3] = 0.0;
    result[4] = 1.0;
    result[5] = 0.0;

    result[6] = t.sin();
    result[7] = 0.0;
    result[8] = t.cos();
}

fn view_3(result: &mut [f64; 9]) {
    let mut v_a: [f64; 3] = [0.0; 3];
    let mut v_b: [f64; 3] = [0.0; 3];
    let mut v_c: [f64; 3] = [0.0; 3];

    v_a.copy_from_slice(&result[..3]);
    v_b.copy_from_slice(&result[3..(3 + 3)]);
    v_c.copy_from_slice(&result[6..(3 + 6)]);

    let mut norm: f64;

    vec_sub_3(&mut v_c, TO3, FROM3);
    norm = norm_3(v_c);
    vec_scale_3(&mut v_c, 1.0 / norm);

    cross_3(&mut v_a, v_c, UP3);
    norm = norm_3(v_a);
    vec_scale_3(&mut v_a, 1.0 / norm);

    cross_3(&mut v_b, v_a, v_c);

    result[..3].copy_from_slice(&v_a);
    result[3..(3 + 3)].copy_from_slice(&v_b);
    result[6..(3 + 6)].copy_from_slice(&v_c);
}

fn project_to_2d(v_angle: f64, mat_view: [f64; 9], mat_rotation: [f64; 9], verts: &mut Verts) {
    let mut v_a: [f64; 3] = [0.0; 3];
    let mut v_b: [f64; 3] = [0.0; 3];
    let mut v_c: [f64; 3] = [0.0; 3];

    v_a.copy_from_slice(&mat_view[..3]);
    v_b.copy_from_slice(&mat_view[3..(3 + 3)]);
    v_c.copy_from_slice(&mat_view[6..(3 + 6)]);

    let mut s: f64;
    let t: f64 = 1.0 / (v_angle / 2.0).tan();

    for i in 0..16 {
        let mut v: [f64; 3] = [0.0; 3];
        mat_vec_mul_3(&mut v, mat_rotation, verts.v3[i]);

        let mut vf: [f64; 3] = [0.0; 3];
        vec_sub_3(&mut vf, v, FROM3);

        s = t / dot_3(vf, v_c);

        verts.v2[i][0] = f64::from(SCREEN_WIDTH / 2) + (f64::from(SCREEN_WIDTH) * s * dot_3(vf, v_a));
        verts.v2[i][1] = f64::from(SCREEN_HEIGHT / 2) + (f64::from(SCREEN_HEIGHT) * s * dot_3(vf, v_b));
    }
}

pub fn run_animation() {
    let mut text_buffer: [char; SCREEN_SIZE] = [' '; SCREEN_SIZE];
    let mut main_verts: Verts = Verts::default();

    let mut view_mat_4: [f64; 16] = [0.0; 16];
    view_4(&mut view_mat_4);
    let mut rot_4: [f64; 16] = [0.0; 16];

    let mut view_mat_3: [f64; 9] = [0.0; 9];
    view_3(&mut view_mat_3);
    let mut rot_3: [f64; 9] = [0.0; 9];

    let mut rotation: f64 = 0.0;

    print!("\x1b[2J");
    loop {
        rotation += 0.01;

        // Rotates the cube in 4-dimensional space
        rot_x_w_4(&mut rot_4, rotation);
        project_to_3d(PI / 3.0, view_mat_4, rot_4, &mut main_verts);

        // Rotates the cube in 3-dimensional space
        rot_x_z_3(&mut rot_3, rotation * 0.3);
        project_to_2d(PI / 4.0, view_mat_3, rot_3, &mut main_verts);

        clear_buffer(&mut text_buffer);

        for i in 0..32 {
            let first_ind: usize = main_verts.indices[i][0];
            let second_ind: usize = main_verts.indices[i][1];

            let first_coord = Coord::from_f64(main_verts.v2[first_ind][0], main_verts.v2[first_ind][1]);
            let second_coord = Coord::from_f64(main_verts.v2[second_ind][0], main_verts.v2[second_ind][1]);

            ln(&mut text_buffer, first_coord, second_coord);
        }

        print!("\x1b[H");

        for k in 0..SCREEN_SIZE {
            // The buffers are linear, so linebreaks are inserted when the frame is printed.
            match k % SCREEN_WIDTH as usize {
                0 => println!(),
                1.. => print!("{}", text_buffer[k]),
            }
        }

        sleep(Duration::from_millis(10));
    }
}