// Work in progress! Does not work!

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::{
    f64::consts::PI,
    thread::sleep,
    time::Duration,
};

const ww: usize = 100;
const wh: usize = 50;

const from4: [f64; 4] = [5.0, 0.0, 0.0, 0.0];
const to4: [f64; 4] =  [0.0, 0.0, 0.0, 0.0];
const up4: [f64; 4] = [0.0, 1.0, 0.0, 0.0];
const over4: [f64; 4] = [0.0, 0.0, 1.0, 0.0];

const from3: [f64; 3] = [3.00, 0.99, 1.82];
const to3: [f64; 3] = [0.0, 0.0, 0.0];
const up3: [f64; 3] = [0.0, -1.0, 0.0];

#[derive(Clone, Copy)]
struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn default() -> Self {
        Coord{
            x: 0,
            y: 0,
        }
    }

    pub fn build(x_coord: f64, y_coord: f64) -> Self {
        let x_trunc: f64 = x_coord.trunc();
        let mut x_short: i16 = 0;
        if x_trunc > f64::from(i16::MAX) {
            x_short = i16::MAX;
        } else if x_trunc < f64::from(i16::MIN) {
            x_short = i16::MIN;
        } else {
            x_short = x_trunc as i16;
        }

        let y_trunc: f64 = y_coord.trunc();
        let mut y_short: i16 = 0;
        if y_trunc > f64::from(i16::MAX) {
            y_short = i16::MAX;
        } else if y_trunc < f64::from(i16::MIN) {
            y_short = i16::MIN;
        } else {
            y_short = y_trunc as i16;
        }

        let x_usize: usize = usize::try_from(x_short).expect("y_short was negative! Conversion to usize failed!");
        let y_usize: usize = usize::try_from(y_short).expect("y_short was negative! Conversion to usize failed!");

        Coord {
            x: x_usize,
            y: y_usize,
        }
    }

    pub fn x_i32(&self) -> i32 {
        self.x as i32
    }

    pub fn y_i32(&self) -> i32 {
        self.y as i32
    }
}

fn clr(char_arr: &mut [char; ww * wh]) {
    for character in char_arr {
        *character = ' ';
    }
}

fn set(char_arr: &mut [char; ww * wh], pt: &Coord, character: char) {
    println!("X: {} | Y: {}", pt.x, pt.y);
    char_arr[pt.y * ww + pt.x] = character;
}

fn getp(char_arr: &mut [char; ww * wh], pts: &[Coord; 3], err: f64) -> char {
    if (pts[0].y_i32() - pts[2].y_i32()).abs() < 2 {
        if err > 0.5 {
            return '-'
        }
        return '_'
    }

    if (pts[0].x_i32() - pts[2].x_i32()).abs() < 2 && (pts[0].x >= pts[2].x || pts[1].x != pts[2].x) && (pts[0].x <= pts[2].x || pts[1].x != pts[0].x) {
        return '|'
    }

    let mX: i32 = if pts[0].y < pts[2].y { pts[0].x_i32() } else { pts[2].x_i32() };
    if mX < pts[1].x_i32() { '\\' } else { '/' }
}

fn ln(char_arr: &mut [char; ww * wh], a: &mut Coord, b: Coord) {
    set(char_arr, &a, '@');
    set(char_arr, &b, '@');

    let dx: i32 = (b.x_i32() - a.x_i32()).abs();
    let sx: i32 = if a.x < b.x { 1 } else { -1 };

    let dy: i32 = (b.y_i32() - a.y_i32()).abs();
    let sy: i32 = if a.y < b.y { 1 } else { -1 };

    let mut err: i32 = (if dx > dy { dx } else { -dy }) / 2;
    let mut e2: i32;

    let mut pts: [Coord; 3] = [Coord::default(); 3];
    let mut ers: [f64; 3] = [0.0; 3];

    for i in 0..3 {
        pts[i] = *a;
        ers[i] = f64::from(err - dx) / f64::from(dy - dx);
        ers[i] = if sy == 1 { 1.0 - ers[i] } else { ers[i] };

        if a.x == b.x && a.y == b.y {
            return;
        }

        e2 = err;

        if e2 > -dx {
            err -= dy;
            a.x += usize::try_from(sx).expect("Couldn't convert `sx` to usize!");
        }

        if e2 < dy {
            err += dx;
            a.y += usize::try_from(sy).unwrap_or(0);
        }
    }

    let mut getp_char: char;

    loop {
        getp_char = getp(char_arr, &pts, ers[1]);
        set(char_arr, &pts[1], getp_char);

        pts[0] = pts[1];
        pts[1] = pts[2];
        pts[2] = *a;

        ers[0] = ers[1];
        ers[1] = ers[2];
        ers[2] = f64::from(err - dx) / f64::from(dy - dx);
        ers[2] = if sy == 1 { 1.0 - ers[2] } else { ers[2] };

        if a.x == b.x && a.y == b.y {
            break;
        }

        e2 = err;
        if e2 > -dx {
            err -= dy;
            a.x += usize::try_from(sx).expect("Couldn't convert `sx` to usize!");
        }

        if e2 < dy {
            err += dx;
            a.y += usize::try_from(sy).unwrap_or(0);
        }
    }

    getp_char = getp(char_arr, &pts, ers[1]);
    set(char_arr, &pts[1], getp_char);
}

struct Verts {
    v4: [[f64; 4]; 16],
    v3: [[f64; 3]; 16],
    v2: [[f64; 2]; 16],

    indices: [[i32; 2]; 32],
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

fn dot4(v: [f64; 4], u: [f64; 4]) -> f64 {
    (v[0] * u[0]) + (v[1] * u[1]) + (v[2] * u[2]) + (v[3] * u[3])
}

fn norm4(v: [f64; 4]) -> f64 {
    dot4(v, v).sqrt()
}

fn cross4(result: &mut [f64; 4], u: [f64; 4], v: [f64; 4], w: [f64; 4]) {
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

fn vecSub4(result: &mut [f64; 4], a: [f64; 4], b: [f64; 4]) {
    result[0] = a[0] - b[0];
	result[1] = a[1] - b[1];
	result[2] = a[2] - b[2];
	result[3] = a[3] - b[3];
}

fn vecScale4(vec: &mut [f64; 4], m: f64) {
    vec[0] *= m;
	vec[1] *= m;
	vec[2] *= m;
	vec[3] *= m;
}

fn matVecMul4(result: &mut [f64; 4], mat: [f64; 16], vec: [f64; 4]) {
    for row in 0..4 {
        result[row] = 0.0;

        for col in 0..4 {
            result[row] += mat[col * 4 + row] * vec[row];
        }
    }
}

fn rotXW4(result: &mut [f64; 16], t: f64) {
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

fn view4(result: &mut [f64; 16]) {
    // column vectors
    let mut Wa: [f64; 4] = [0.0; 4];
    let mut Wb: [f64; 4] = [0.0; 4];
    let mut Wc: [f64; 4] = [0.0; 4];
    let mut Wd: [f64; 4] = [0.0; 4];

    for i in 0..4 {
        Wa[i] = result[i + 0];
        Wb[i] = result[i + 4];
        Wc[i] = result[i + 8];
        Wd[i] = result[i + 12];
    }

    let mut norm: f64;

    // get the normalized Wd column-vector.
    vecSub4(&mut Wd, to4, from4);
    norm = norm4(Wd);
    vecScale4(&mut Wd, 1.0 / norm);

    // calculate the normalized Wa column-vector.
    cross4(&mut Wa, up4, over4, Wd);
    norm = norm4(Wa);
    vecScale4(&mut Wa, 1.0 / norm);

    // calculate the normalized Wb column-vector.
    cross4(&mut Wb, over4, Wd, Wa);
    norm = norm4(Wb);
    vecScale4(&mut Wb, 1.0 / norm);

    // calculate the Wc column-vector.
    cross4(&mut Wc, Wd, Wa, Wb);

    for i in 0..4 {
        result[i + 0] = Wa[i];
        result[i + 4] = Wb[i];
        result[i + 8] = Wc[i];
        result[i + 12] = Wd[i];
    }
}

fn projectTo3D(vAngle: f64, matView: [f64; 16], matRotation: [f64; 16], verts: &mut Verts) {
    let mut Wa: [f64; 4] = [0.0; 4];
    let mut Wb: [f64; 4] = [0.0; 4];
    let mut Wc: [f64; 4] = [0.0; 4];
    let mut Wd: [f64; 4] = [0.0; 4];

    for i in 0..4 {
        Wa[i] = matView[i + 0];
        Wb[i] = matView[i + 4];
        Wc[i] = matView[i + 8];
        Wd[i] = matView[i + 12];
    }

    let mut s: f64;
    let t: f64 = 1.0 / (vAngle / 2.0).tan();

    for i in 0..16 {
        let mut v:[f64; 4] = [0.0; 4];
        matVecMul4(&mut v, matRotation, verts.v4[i]);

        let mut vf: [f64; 4] = [0.0; 4];
        vecSub4(&mut vf, v, from4);

        s = t / dot4(vf, Wd);

        verts.v3[i][0] = s * dot4(vf, Wa);
    }
}

fn dot3(v: [f64; 3], u: [f64; 3]) -> f64 {
    (v[0] * u[0]) + (v[1] * u[1]) + (v[2] * u[2])
}

fn norm3(v: [f64; 3]) -> f64 {
    dot3(v, v).sqrt()
}

fn cross3(result: &mut [f64; 3], u: [f64; 3], v: [f64; 3]) {
    result[0] = (u[1] * v[2]) - (u[2] * v[1]);
    result[1] = (u[2] * v[0]) - (u[0] * v[2]);
	result[2] = (u[0] * v[1]) - (u[1] * v[0]);
}

fn vecSub3(result: &mut [f64; 3], a: [f64; 3], b: [f64; 3]) {
    result[0] = a[0] - b[0];
	result[1] = a[1] - b[1];
	result[2] = a[2] - b[2];
}

fn vecScale3(vec: &mut [f64; 3], m: f64) {
    vec[0] *= m;
	vec[1] *= m;
	vec[2] *= m;
}

fn matVecMul3(result: &mut [f64; 3], mat: [f64; 9], vec: [f64; 3]) {
    for row in 0..3 {
        result[row] = 0.0;
        for col in 0..3 {
            result[row] += mat[col * 3 + row] * vec[row];
        }
    }
}

fn rotXZ3(result: &mut [f64; 9], t: f64) {
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

fn view3(result: &mut [f64; 9]) {
    let mut Va: [f64; 3] = [0.0; 3];
    let mut Vb: [f64; 3] = [0.0; 3];
    let mut Vc: [f64; 3] = [0.0; 3];

    for i in 0..3 {
        Va[i] = result[i + 0];
        Vb[i] = result[i + 3];
        Vc[i] = result[i + 6];
    }

    let mut norm: f64;

    vecSub3(&mut Vc, to3, from3);
    norm = norm3(Vc);
    vecScale3(&mut Vc, 1.0 / norm);

    cross3(&mut Va, Vc, up3);
    norm = norm3(Va);
    vecScale3(&mut Va, 1.0 / norm);

    cross3(&mut Vb, Va, Vc);

    for i in 0..3 {
        result[i + 0] = Va[i];
        result[i + 3] = Vb[i];
        result[i + 6] = Vc[i];
    }
}

fn projectTo2D(vAngle: f64, matView: [f64; 9], matRotation: [f64; 9], verts: &mut Verts) {
    let mut Va: [f64; 3] = [0.0; 3];
    let mut Vb: [f64; 3] = [0.0; 3];
    let mut Vc: [f64; 3] = [0.0; 3];

    for i in 0..3 {
        Va[i] = matView[i + 0];
        Vb[i] = matView[i + 3];
        Vc[i] = matView[i + 6];
    }

    let mut s: f64;
    let t: f64 = 1.0 / (vAngle / 2.0).tan();

    for i in 0..16 {
        let mut v: [f64; 3] = [0.0; 3];
        matVecMul3(&mut v, matRotation, verts.v3[i]);

        let mut vf: [f64; 3] = [0.0; 3];
        vecSub3(&mut vf, v, from3);

        s = t / dot3(vf, Vc);

        verts.v2[i][0] = ((ww / 2) as f64) + ((ww as f64) * s * dot3(vf, Va));
        verts.v2[i][1] = ((wh / 2) as f64) + ((wh as f64) * s * dot3(vf, Vb));
    }
}

fn main() {
    let mut d: [char; wh * ww] = [' '; wh * ww];

    let mut main_verts: Verts = Verts::default();

    let mut viewMat4: [f64; 16] = [0.0; 16];
    view4(&mut viewMat4);
    let mut rot4: [f64; 16] = [0.0; 16];

    let mut viewMat3: [f64; 9] = [0.0; 9];
    view3(&mut viewMat3);
    let mut rot3: [f64; 9] = [0.0; 9];

    let mut rotation: f64 = 0.0;

    loop {
        rotation += 0.01;

        rotXW4(&mut rot4, rotation);
        projectTo3D(PI / 3.0, viewMat4, rot4, &mut main_verts);

        rotXZ3(&mut rot3, rotation * 0.3);
        projectTo2D(PI / 4.0, viewMat3, rot3, &mut main_verts);

        clr(&mut d);

        for i in 0..32 {
            let a: i32 = main_verts.indices[i][0];
            let b: i32 = main_verts.indices[i][1];
            let mut c1: Coord = Coord::build(main_verts.v2[a as usize][0], main_verts.v2[a as usize][1]);
            let c2: Coord = Coord::build(main_verts.v2[b as usize][0], main_verts.v2[b as usize][1]);
            ln(&mut d, &mut c1, c2);
        }

        print!("\x1b[H");

        for k in 0..(ww * wh) {
            // The buffers are linear, so linebreaks are inserted when the frame is printed.
            match k % ww {
                0 => print!("\n"),
                1.. => print!("{}", d[k])
            }
        }

        sleep(Duration::from_secs(1));
    }
}