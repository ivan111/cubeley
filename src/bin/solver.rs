use std::time;

use cube::State;

// 反対面
macro_rules! inv_face {
    ($face:expr) => {
        match $face {
            'U' => 'D',
            'D' => 'U',
            'L' => 'R',
            'R' => 'L',
            'F' => 'B',
            'B' => 'F',
            _ => panic!("不正な面"),
        }
    };
}

// 前の1手を考慮して次の1手として使える操作であるかを判定する
// - 同じ面は連続して回さない (e.g. R' R2 は不可)
// - 対面を回すときは順序を固定する (e.g. D Uは良いが、U Dは不可)
fn is_move_available(prev_move: Option<&String>, cur_move: &str) -> bool {
    if prev_move == None {
        return true  // 最初の一手はどの操作も可能
    }

    let prev_face = prev_move.unwrap().chars().next().unwrap();  // 1手前で回した面
    let cur_face = cur_move.chars().next().unwrap();  // 現在回そうとしている面

    if prev_face == cur_face {
        // 同一面は不可能
        return false;
    }

    if inv_face!(prev_face) == cur_face {
        return prev_face < cur_face;  // 対面のときは、辞書順なら可
    }

    true
}

const MOVE_NAMES: [&str; 18] = [
"U", "F", "R", "D", "B", "L",
"U2", "F2", "R2", "D2", "B2", "L2",
"U'", "F'", "R'", "D'", "B'", "L'",
];


fn depth_limited_search(state: &State, solution: &mut Vec<String>, depth: i32) -> bool {
    if depth == 0 && state.is_solved0() {
        return true;
    }

    if depth == 0 {
        return false;
    }

    for move_name in MOVE_NAMES {
        if !is_move_available(solution.last(), move_name) {
            continue;
        }

        solution.push(move_name.to_string());

        if depth_limited_search(&(state * move_name), solution, depth - 1) {
            return true;
        }

        solution.pop();
    }

    false
}

fn start_search(state: &State, max_length: i32) -> Option<Vec<String>> {
    let mut solution = vec![];

    for depth in 0..max_length {
        println!("Start searching length {}", depth);
        if depth_limited_search(&state, &mut solution, depth) {
            return Some(solution);
        }
    }

    None
}

fn main() {
    let scramble = "R U R' F2 D2 L";
    let scrambled_state = &cube::State::new_solved() * scramble;

    let now = time::Instant::now();

    match start_search(&scrambled_state, 7) {
        Some(v) => println!("{:?}", v),
        None => println!("Solution not found"),
    }

    println!("{:?}", now.elapsed());
}
