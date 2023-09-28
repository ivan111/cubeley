//! 3x3x3のキューブパズルをシミュレーションする

pub mod moves;

use std::ops;
use std::collections::HashMap;

use colored::*;

/// キューブの状態を表す。
/// キューブの動きも状態で表す。
#[derive(Debug, Clone, PartialEq)]
pub struct State {
    p: [u8; 54],  // インデックス => 値の置換[上9, 前9, 右9, 下9, 後9, 左9]。pは値を変えてはいけない。
}

/// キューブの色
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Green,
    Red,
    Yellow,
    Blue,
    Orange,
}

/// キューブの面
pub enum Face {
    Up,
    Front,
    Right,
    Down,
    Back,
    Left,
}

// 色への変換マップ
static COLOR_MAP: [Color; 54] = [
    Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White, Color::White,
    Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green, Color::Green,
    Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red, Color::Red,
    Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow, Color::Yellow,
    Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue, Color::Blue,
    Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange, Color::Orange,
];

// キューブの色から色付きの文字列をつくる。
macro_rules! color_str {
    ($color:expr) => {
        match $color {
            Color::White => "W".white(),
            Color::Green => "G".truecolor(0, 155, 72),
            Color::Red => "R".truecolor(183, 18, 52),
            Color::Yellow => "Y".truecolor(255, 213, 0),
            Color::Blue => "B".truecolor(0, 70, 173),
            Color::Orange => "O".truecolor(255, 88, 0),
        }
    };
}

impl State {
    /// 上が白で、前が緑の状態のキューブ。
    pub const SOLVED: State = State {
        p: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
            20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
            30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
            40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
            50, 51, 52, 53],
    };

    /// 新しいキューブを作る。
    pub fn new(p: [u8; 54]) -> State {
        State { p }
    }

    /// 巡回置換から新しいキューブを作る。
    pub fn cycles(cp: &Vec<u8>) -> State {
        let mut p = Self::SOLVED.p.clone();

        if cp.len() != 0 {
            let first = cp[0];
            let last = cp.last().unwrap();

            for v in cp.windows(2) {
                p[v[0] as usize] = v[1];
            }

            p[*last as usize] = first;
        }

        State { p }
    }

    /// 巡回置換の積から新しいキューブを作る。
    pub fn product_of_cycles(pcp: &Vec<Vec<u8>>) -> State {
        let mut st = Self::SOLVED.clone();

        for cp in pcp {
            st = st.apply(&Self::cycles(cp));
        }

        st
    }

    /// pを取得する。
    pub fn get_p(&self) -> [u8; 54] {
        self.p
    }

    /// キューブがそろっているならtrueを返す。
    /// 回転記号x, y, z, E, M, Sなどセンターキューブを動かしていない場合は、
    /// SOLVEDと==で比較したほうが速い。
    pub fn is_solved(&self) -> bool {
        let p = &self.get_prime().p;

        p[1..9].iter().all(|c| COLOR_MAP[*c as usize] == COLOR_MAP[p[0] as usize]) &&
        p[10..18].iter().all(|c| COLOR_MAP[*c as usize] == COLOR_MAP[p[9] as usize]) &&
        p[19..27].iter().all(|c| COLOR_MAP[*c as usize] == COLOR_MAP[p[18] as usize]) &&
        p[28..36].iter().all(|c| COLOR_MAP[*c as usize] == COLOR_MAP[p[27] as usize]) &&
        p[37..45].iter().all(|c| COLOR_MAP[*c as usize] == COLOR_MAP[p[36] as usize])
    }

    /// キューブを動かす。定義からわかるがselfは変化しない。
    pub fn apply(&self, mv: &State) -> State {
        let mut p = [0; 54];

        for i in 0..54 {
            p[i] = mv.p[self.p[i] as usize];
        }

        State { p }
    }

    /// 回転記号を指定してキューブを動かす。定義からわかるがselfは変化しない。
    pub fn apply_moves(&self, moves: &HashMap<String, State>, mvs: &str) -> State {
        let mut cube = self.clone();

        for name in mvs.split(' ') {
            if name == "" {
                continue;
            }

            match moves.get(name) {
                None => eprintln!("無効な操作: {}", name),
                Some(mv) => cube = cube.apply(mv),
            }
        }

        cube
    }

    /// 指定した動きの逆の動きを取得する。
    pub fn get_prime(&self) -> State {
        let mut p = [0_u8; 54];

        for (i, v) in self.p.iter().enumerate() {
            p[*v as usize] = i as u8;
        }

        State { p }
    }

    /// キューブの状態を端末に出力する。
    /// 端末はtrueカラーに対応している前提で書いている。
    pub fn print(&self) {
        let up = self.get_face_colors(Face::Up);
        let left = self.get_face_colors(Face::Left);
        let front = self.get_face_colors(Face::Front);
        let right = self.get_face_colors(Face::Right);
        let back = self.get_face_colors(Face::Back);
        let down = self.get_face_colors(Face::Down);

        for i in 0..3 {
            println!("    {}{}{}", color_str!(up[i*3 + 0]), color_str!(up[i*3 + 1]), color_str!(up[i*3 + 2]));
        }

        for i in 0..3 {
            println!("{}{}{} {}{}{} {}{}{} {}{}{}",
                     color_str!(left[i*3 + 0]), color_str!(left[i*3 + 1]), color_str!(left[i*3 + 2]),
                     color_str!(front[i*3 + 0]), color_str!(front[i*3 + 1]), color_str!(front[i*3 + 2]),
                     color_str!(right[i*3 + 0]), color_str!(right[i*3 + 1]), color_str!(right[i*3 + 2]),
                     color_str!(back[i*3 + 0]), color_str!(back[i*3 + 1]), color_str!(back[i*3 + 2]));
        }

        for i in 0..3 {
            println!("    {}{}{}", color_str!(down[i*3 + 0]), color_str!(down[i*3 + 1]), color_str!(down[i*3 + 2]));
        }
    }

    /// キューブ面の色を取得する。
    pub fn get_face_colors(&self, face: Face) -> [Color; 9] {
        let mut colors = [Color::White; 9];

        let start = face as usize * 9;
        let end = start + 9;

        let p = &self.get_prime().p;

        for (i, pi) in (start..end).enumerate() {
            colors[i] = COLOR_MAP[p[pi] as usize]
        }

        colors
    }
}

// *演算子でキューブを動かす。
impl ops::Mul<&State> for &State {
    type Output = State;

    fn mul(self, rhs: &State) -> Self::Output {
        self.apply(rhs)
    }
}
