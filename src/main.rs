// Work in progress! Does not work!

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

const ww: usize = 100;
const wh: usize = 50;

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
            a.y += usize::try_from(sy).expect("Couldn't convert `sy` to usize!");
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
            a.y += usize::try_from(sy).expect("Couldn't convert `sy` to usize!");
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

fn matVecMul4(result: &mut [f64; 4], mat: [f64; 4], vec: [f64; 4]) {
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

struct Conversions {
    from4: [f64; 4],
    to4: [f64; 4],
    up4: [f64; 4],
    over4: [f64; 4],
}

impl Conversions {
    pub fn default() -> Self {
        Conversions {
            from4: [5.0, 0.0, 0.0, 0.0],
            to4: [0.0, 0.0, 0.0, 0.0],
            up4: [0.0, 1.0, 0.0, 0.0],
            over4: [0.0, 0.0, 1.0, 0.0],
        }
    }
}

fn main() {
    println!("Hello, world!");
}