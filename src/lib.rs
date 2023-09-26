//! 3x3x3のキューブパズルをシミュレーションする

pub mod moves;

use std::ops;
use std::collections::HashMap;

use colored::*;

/// キューブの状態を表す。
/// キューブの動きも状態で表す。
#[derive(Debug, Clone, PartialEq)]
pub struct State {
    cc: [u8; 6],  // センターカラー。[上, 前, 右, 下, 後, 左]
    cp: [u8; 8],  // コーナー位置
    co: [u8; 8],  // コーナー方向
    ep: [u8; 12],  // エッジ位置
    eo: [u8; 12],  // エッジ方向
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

// センターピース色
static CENTER_COLORS: [Color; 6] = [
    Color::White,
    Color::Green,
    Color::Red,
    Color::Yellow,
    Color::Blue,
    Color::Orange,
];

// コーナーピース色
static CORNER_COLORS: [[Color; 3]; 8] = [
    [Color::White, Color::Blue, Color::Orange],
    [Color::White, Color::Red, Color::Blue],
    [Color::White, Color::Green, Color::Red],
    [Color::White, Color::Orange, Color::Green],
    [Color::Yellow, Color::Orange, Color::Blue],
    [Color::Yellow, Color::Blue, Color::Red],
    [Color::Yellow, Color::Red, Color::Green],
    [Color::Yellow, Color::Green, Color::Orange],
];

// エッジピース色
static EDGE_COLORS: [[Color; 2]; 12] = [
    [Color::Blue, Color::Orange],
    [Color::Blue, Color::Red],
    [Color::Green, Color::Red],
    [Color::Green, Color::Orange],
    [Color::White, Color::Blue],
    [Color::White, Color::Red],
    [Color::White, Color::Green],
    [Color::White, Color::Orange],
    [Color::Yellow, Color::Blue],
    [Color::Yellow, Color::Red],
    [Color::Yellow, Color::Green],
    [Color::Yellow, Color::Orange],
];

macro_rules! corner_color {
    // corner_color(self, 0) は以下のように展開される
    // CORNER_COLORS[self.cp[0] as usize][self.co[0] as usize]
    ($self:ident, $perm:expr) => ({
        let perm_val = &$perm;
        CORNER_COLORS[$self.cp[*perm_val] as usize][$self.co[*perm_val] as usize]
    });

    // corner_color(self, 0, 1) は以下のように展開される
    // CORNER_COLORS[self.cp[0] as usize][((self.co[0] + 1) % 3) as usize]
    ($self:ident, $perm:expr, $orient:expr) => ({
        let perm_val = &$perm;
        CORNER_COLORS[$self.cp[*perm_val] as usize][(($self.co[*perm_val] + $orient) % 3) as usize]
    });
}

macro_rules! edge_color {
    // edge_color(self, 0) は以下のように展開される
    // EDGE_COLORS[self.ep[0] as usize][self.eo[0] as usize]
    ($self:ident, $perm:expr) => ({
        let perm_val = &$perm;
        EDGE_COLORS[$self.ep[*perm_val] as usize][$self.eo[*perm_val] as usize]
    });

    // edge_color(self, 0, 1) は以下のように展開される
    // EDGE_COLORS[self.ep[0] as usize][((self.eo[0] + 1) % 2) as usize]
    ($self:ident, $perm:expr, $orient:expr) => ({
        let perm_val = &$perm;
        EDGE_COLORS[$self.ep[*perm_val] as usize][(($self.eo[*perm_val] + $orient) % 2) as usize]
    });
}

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
        cc: [0, 1, 2, 3, 4, 5],
        cp: [0, 1, 2, 3, 4, 5, 6, 7],
        co: [0; 8],
        ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        eo: [0; 12],
    };

    /// 上が白で、前が緑の状態の新しいキューブを作る。
    pub fn new() -> State {
        Self::SOLVED
    }

    /// キューブがそろっているならtrueを返す。
    /// 回転記号x, y, z, E, M, Sなどセンターキューブを動かしていない場合は、
    /// SOLVEDと==で比較したほうが速い。
    pub fn is_solved(&self) -> bool {
        let up = self.get_up_colors();
        let front = self.get_front_colors();
        let right = self.get_right_colors();
        let down = self.get_down_colors();
        let back = self.get_back_colors();

        up[1..].iter().all(|c| *c == up[0]) &&
        front[1..].iter().all(|c| *c == front[0]) &&
        right[1..].iter().all(|c| *c == right[0]) &&
        down[1..].iter().all(|c| *c == down[0]) &&
        back[1..].iter().all(|c| *c == back[0])
    }

    /// キューブを動かす。定義からわかるがselfは変化しない。
    pub fn apply(&self, mv: &State) -> State {
        let mut cube = State{cc: [0; 6], cp: [0; 8], co: [0; 8], ep: [0; 12], eo: [0; 12]};

        for i in 0..6 {
            cube.cc[i] = self.cc[mv.cc[i] as usize];
        }

        for i in 0..8 {
            cube.cp[i] = self.cp[mv.cp[i] as usize];
            cube.co[i] = (self.co[mv.cp[i] as usize] + mv.co[i]) % 3;
        }

        for i in 0..12 {
            cube.ep[i] = self.ep[mv.ep[i] as usize];
            cube.eo[i] = (self.eo[mv.ep[i] as usize] + mv.eo[i]) % 2;
        }

        cube
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

    /// キューブの状態を端末に出力する。
    /// 端末はtrueカラーに対応している前提で書いている。
    pub fn print(&self) {
        let up = self.get_up_colors();
        let left = self.get_left_colors();
        let front = self.get_front_colors();
        let right = self.get_right_colors();
        let back = self.get_back_colors();
        let down = self.get_down_colors();

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

    /// 上面の色を取得する。
    pub fn get_up_colors(&self) -> [Color; 9] {
        let mut colors = [Color::White; 9];

        colors[0] = corner_color!(self, 0);
        colors[1] = edge_color!(self, 4);
        colors[2] = corner_color!(self, 1);
        colors[3] = edge_color!(self, 7);
        colors[4] = CENTER_COLORS[self.cc[0] as usize];
        colors[5] = edge_color!(self, 5);
        colors[6] = corner_color!(self, 3);
        colors[7] = edge_color!(self, 6);
        colors[8] = corner_color!(self, 2);

        colors
    }

    /// 正面の色を取得する。
    pub fn get_front_colors(&self) -> [Color; 9] {
        let mut colors = [Color::White; 9];

        colors[0] = corner_color!(self, 3, 2);
        colors[1] = edge_color!(self, 6, 1);
        colors[2] = corner_color!(self, 2, 1);
        colors[3] = edge_color!(self, 3);
        colors[4] = CENTER_COLORS[self.cc[1] as usize];
        colors[5] = edge_color!(self, 2);
        colors[6] = corner_color!(self, 7, 1);
        colors[7] = edge_color!(self, 10, 1);
        colors[8] = corner_color!(self, 6, 2);

        colors
    }

    /// 右面の色を取得する。
    pub fn get_right_colors(&self) -> [Color; 9] {
        let mut colors = [Color::White; 9];

        colors[0] = corner_color!(self, 2, 2);
        colors[1] = edge_color!(self, 5, 1);
        colors[2] = corner_color!(self, 1, 1);
        colors[3] = edge_color!(self, 2, 1);
        colors[4] = CENTER_COLORS[self.cc[2] as usize];
        colors[5] = edge_color!(self, 1, 1);
        colors[6] = corner_color!(self, 6, 1);
        colors[7] = edge_color!(self, 9, 1);
        colors[8] = corner_color!(self, 5, 2);

        colors
    }

    /// 下面の色を取得する。
    pub fn get_down_colors(&self) -> [Color; 9] {
        let mut colors = [Color::White; 9];

        colors[0] = corner_color!(self, 7);
        colors[1] = edge_color!(self, 10);
        colors[2] = corner_color!(self, 6);
        colors[3] = edge_color!(self, 11);
        colors[4] = CENTER_COLORS[self.cc[3] as usize];
        colors[5] = edge_color!(self, 9);
        colors[6] = corner_color!(self, 4);
        colors[7] = edge_color!(self, 8);
        colors[8] = corner_color!(self, 5);

        colors
    }

    /// 後面の色を取得する。
    pub fn get_back_colors(&self) -> [Color; 9] {
        let mut colors = [Color::White; 9];

        colors[0] = corner_color!(self, 1, 2);
        colors[1] = edge_color!(self, 4, 1);
        colors[2] = corner_color!(self, 0, 1);
        colors[3] = edge_color!(self, 1);
        colors[4] = CENTER_COLORS[self.cc[4] as usize];
        colors[5] = edge_color!(self, 0);
        colors[6] = corner_color!(self, 5, 1);
        colors[7] = edge_color!(self, 8, 1);
        colors[8] = corner_color!(self, 4, 2);

        colors
    }

    /// 左面の色を取得する。
    pub fn get_left_colors(&self) -> [Color; 9] {
        let mut colors = [Color::White; 9];

        colors[0] = corner_color!(self, 0, 2);
        colors[1] = edge_color!(self, 7, 1);
        colors[2] = corner_color!(self, 3, 1);
        colors[3] = edge_color!(self, 0, 1);
        colors[4] = CENTER_COLORS[self.cc[5] as usize];
        colors[5] = edge_color!(self, 3, 1);
        colors[6] = corner_color!(self, 4, 1);
        colors[7] = edge_color!(self, 11, 1);
        colors[8] = corner_color!(self, 7, 2);

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
