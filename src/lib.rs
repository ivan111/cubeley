//! 3x3x3のキューブパズルをシミュレーションする

pub mod moves;

use std::ops;
use std::collections::HashMap;

use colored::*;

pub const NUM_P: usize = 54;

/// キューブの状態を表す。
/// キューブの動きも状態で表す。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    p: Box<[u8; NUM_P]>,  // インデックス => 値の置換[上9, 前9, 右9, 下9, 後9, 左9]。pは値を変えてはいけない。
}

/// キューブの色
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
static COLOR_MAP: [Color; NUM_P] = [
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


/// 上が白で、前が緑の状態のキューブ。
const SOLVED_P: [u8; NUM_P] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
    30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
    40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53
];


/// 最大公約数
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b)
    }

    a
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(2, 3), 1);
    assert_eq!(gcd(4, 2), 2);
    assert_eq!(gcd(247, 962), 13);
}


/// 最小公倍数
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(2, 3), 6);
    assert_eq!(lcm(4, 2), 4);
    assert_eq!(lcm(247, 962), 18278);
}


impl Default for State {
    fn default() -> State {
        State::new_solved()
    }
}

impl State {
    /// 上が白で、前が緑の新しいキューブを作る。
    pub fn new_solved() -> State {
        State { p: Box::new(SOLVED_P) }
    }

    /// 指定した動きを取得する。
    pub fn get_move(name: moves::MOVES) -> State {
        State { p: Box::new(moves::MOVES_P[name as usize]) }
    }

    /// 指定した動きを取得する。
    pub fn get_move_by_name(name: &str) -> Option<State> {
        moves::get_move(name)
    }

    /// 新しいキューブを作る。
    pub fn new(p: Box<[u8; NUM_P]>) -> State {
        let st = State { p };

        if !st.is_valid_permutation() {
            panic!("不正な置換: {:?}", st.p);
        }

        st
    }

    /// 巡回置換から新しいキューブを作る。
    pub fn cycles(cp: &Vec<u8>) -> State {
        let mut p = SOLVED_P;

        if !cp.is_empty() {
            let first = cp[0];
            let last = cp.last().unwrap();

            for v in cp.windows(2) {
                p[v[0] as usize] = v[1];
            }

            p[*last as usize] = first;
        }

        let st = State { p: Box::new(p) };

        if !st.is_valid_permutation() {
            panic!("不正な置換: {:?}", st.p);
        }

        st
    }

    /// 巡回置換の積から新しいキューブを作る。
    pub fn product_of_cycles(pcp: &Vec<Vec<u8>>) -> State {
        let mut st = Self::new_solved();

        for cp in pcp {
            st = st.apply(&Self::cycles(cp));
        }

        st
    }

    /// pを取得する。
    pub fn get_p(&self) -> [u8; NUM_P] {
        *self.p
    }

    /// キューブがそろっているならtrueを返す。
    /// 回転記号x, y, z, E, M, Sなどセンターキューブを動かしていない場合のみ使用可能。
    pub fn is_solved0(&self) -> bool {
        *self.p == SOLVED_P
    }

    /// キューブがそろっているならtrueを返す。
    /// 回転記号x, y, z, E, M, Sなどセンターキューブを動かしていない場合は、
    /// is_solved0で比較したほうが速い。
    pub fn is_solved(&self) -> bool {
        self.p[1..9].iter().all(|c| COLOR_MAP[*c as usize] == COLOR_MAP[self.p[0] as usize]) &&
        self.p[10..18].iter().all(|c| COLOR_MAP[*c as usize] == COLOR_MAP[self.p[9] as usize]) &&
        self.p[19..27].iter().all(|c| COLOR_MAP[*c as usize] == COLOR_MAP[self.p[18] as usize]) &&
        self.p[28..36].iter().all(|c| COLOR_MAP[*c as usize] == COLOR_MAP[self.p[27] as usize]) &&
        self.p[37..45].iter().all(|c| COLOR_MAP[*c as usize] == COLOR_MAP[self.p[36] as usize])
    }

    pub fn count_solved0_corners(&self) -> i32 {
        let mut count = 0;

        for i in [0, 2, 6, 8, 27, 29, 33, 35] {
            if self.p[i] as usize == i {
                count += 1;
            }
        }

        count
    }

    pub fn count_solved0_edges(&self) -> i32 {
        let mut count = 0;

        for i in [1, 3, 5, 7, 12, 14, 39, 41, 28, 30, 32, 34] {
            if self.p[i] as usize == i {
                count += 1;
            }
        }

        count
    }

    /// キューブを動かす。定義からわかるがselfは変化しない。
    pub fn apply(&self, mv: &State) -> State {
        let mut p = [0; NUM_P];

        for (i, v) in self.p.into_iter().enumerate() {
            p[i] = mv.p[v as usize];
        }

        State { p: Box::new(p) }
    }

    /// 回転記号を指定してキューブを動かす。定義からわかるがselfは変化しない。
    pub fn apply_moves(&self, mvs: &str) -> Result<State, String> {
        let mut p = *self.p;

        for name in mvs.split_whitespace() {
            if let Some(mv) = State::get_move_by_name(name) {
                for (i, v) in p.into_iter().enumerate() {
                    p[i] = mv.p[v as usize]
                }
            } else {
                return Err(format!("無効な操作: {}", name));
            }
        }

        Ok(State { p: Box::new(p) })
    }

    /// 回転記号を指定してキューブを動かす。定義からわかるがselfは変化しない。
    pub fn apply_arg_moves(&self, moves: &HashMap<String, State>, mvs: &str) -> Result<State, String> {
        let mut cube = self.clone();

        for name in mvs.split_whitespace() {
            match moves.get(name) {
                None => return Err(format!("無効な操作: {}", name)),
                Some(mv) => cube = cube.apply(mv),
            }
        }

        Ok(cube)
    }

    /// 指定した動きの逆の動きを取得する。
    pub fn get_prime(&self) -> State {
        let mut p = [0_u8; NUM_P];

        for (i, v) in self.p.iter().enumerate() {
            p[*v as usize] = i as u8;
        }

        State { p: Box::new(p) }
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
            println!("    {}{}{}", color_str!(up[i*3]), color_str!(up[i*3 + 1]), color_str!(up[i*3 + 2]));
        }

        for i in 0..3 {
            println!("{}{}{} {}{}{} {}{}{} {}{}{}",
                     color_str!(left[i*3]), color_str!(left[i*3 + 1]), color_str!(left[i*3 + 2]),
                     color_str!(front[i*3]), color_str!(front[i*3 + 1]), color_str!(front[i*3 + 2]),
                     color_str!(right[i*3]), color_str!(right[i*3 + 1]), color_str!(right[i*3 + 2]),
                     color_str!(back[i*3]), color_str!(back[i*3 + 1]), color_str!(back[i*3 + 2]));
        }

        for i in 0..3 {
            println!("    {}{}{}", color_str!(down[i*3]), color_str!(down[i*3 + 1]), color_str!(down[i*3 + 2]));
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

    /// 置換として正しい形式か？
    fn is_valid_permutation(&self) -> bool {
        let mut p = self.p.to_vec();
        p.sort();

        *p == SOLVED_P
    }

    /// 巡回置換の積を返す。
    pub fn get_cycles(&self) -> Vec<Vec<u8>> {
        let mut used = [false; NUM_P];
        let mut pcp = vec![];

        for i in 0..NUM_P {
            if i == self.p[i] as usize || used[i] {
                continue;
            }

            used[i] = true;

            let mut cp = vec![i as u8];

            let mut v = self.p[i];

            while i != v as usize {
                used[v as usize] = true;
                cp.push(v);
                v = self.p[v as usize];
            }

            pcp.push(cp);
        }

        pcp
    }

    /// 何回この操作を繰り返すと元に戻るかを返す
    pub fn get_period(&self) -> usize {
        self.get_cycles().iter().map(|v| v.len()).reduce(lcm).unwrap_or(0)
    }

    /// 巡回置換の積を出力する。
    pub fn print_cycles(&self) {
        let pcp = self.get_cycles();

        if pcp.is_empty() {
            println!("()");
            return
        }

        for cp in pcp {
            print!("({})", cp.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" "));
        }

        println!();
    }
}

// *演算子でキューブを動かす。
impl ops::Mul for &State {
    type Output = State;

    fn mul(self, rhs: Self) -> Self::Output {
        self.apply(rhs)
    }
}

impl ops::Mul<&str> for &State {
    type Output = State;

    fn mul(self, rhs: &str) -> Self::Output {
        self.apply_moves(rhs).expect("エラー")
    }
}
