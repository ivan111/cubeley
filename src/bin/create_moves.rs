use std::collections::HashMap;

use cube::State;

fn print_state(name: &str, s: &State) {
    let p = s.get_p();

    println!("    moves.insert(String::from(\"{}\"), State::no_check_new(Box::new({:?})));", name, p);
}

fn main() {
    let mut moves = HashMap::new();

    let mut x_arr = [0_u8; 54];
    for (i, v) in (36..45).rev().chain(0..9)
        .chain([20, 23, 26, 19, 22, 25, 18, 21, 24]).chain(9..18).chain((27..36).rev())
        .chain([51, 48, 45, 52, 49, 46, 53, 50, 47]).enumerate() {
        x_arr[i] = v;
    }
    let x = State::new(Box::new(x_arr));
    moves.insert(String::from("x"), x);

    let mut y_arr = [0_u8; 54];
    for (i, v) in [2, 5, 8, 1, 4, 7, 0, 3, 6].into_iter().chain(45..54).chain(9..18)
        .chain([33, 30, 27, 34, 31, 28, 35, 32, 29]).chain(18..27).chain(36..45).enumerate() {
        y_arr[i] = v;
    }
    let y = State::new(Box::new(y_arr));
    moves.insert(String::from("y"), y);

    let solved = State::new_solved();

    let z = solved.apply_moves(&moves, "y y y x y");
    moves.insert(String::from("z"), z);


    let u = State::product_of_cycles(&vec![vec![9, 45, 36, 18], vec![10, 46, 37, 19], vec![11, 47, 38, 20],
        vec![0, 2, 8, 6], vec![1, 5, 7, 3]]);
    moves.insert(String::from("U"), u);

    let d = solved.apply_moves(&moves, "x x U x x");
    moves.insert(String::from("D"), d);

    let r = solved.apply_moves(&moves, "z z z U z");
    moves.insert(String::from("R"), r);

    let l = solved.apply_moves(&moves, "z U z z z");
    moves.insert(String::from("L"), l);

    let f = solved.apply_moves(&moves, "x U x x x");
    moves.insert(String::from("F"), f);

    let b = solved.apply_moves(&moves, "x x x U x");
    moves.insert(String::from("B"), b);


    let m = solved.apply_moves(&moves, "x x x R L L L");
    moves.insert(String::from("M"), m);

    let e = solved.apply_moves(&moves, "y y y U D D D");
    moves.insert(String::from("E"), e);

    let s = solved.apply_moves(&moves, "z F F F B");
    moves.insert(String::from("S"), s);


    let uw = solved.apply_moves(&moves, "U E E E");
    moves.insert(String::from("u"), uw.clone());
    moves.insert(String::from("Uw"), uw);

    let fw = solved.apply_moves(&moves, "F S");
    moves.insert(String::from("f"), fw.clone());
    moves.insert(String::from("Fw"), fw);

    let rw = solved.apply_moves(&moves, "R M M M");
    moves.insert(String::from("r"), rw.clone());
    moves.insert(String::from("Rw"), rw);

    let bw = solved.apply_moves(&moves, "B S S S");
    moves.insert(String::from("b"), bw.clone());
    moves.insert(String::from("Bw"), bw);

    let lw = solved.apply_moves(&moves, "L M");
    moves.insert(String::from("l"), lw.clone());
    moves.insert(String::from("Lw"), lw);

    let dw = solved.apply_moves(&moves, "D E");
    moves.insert(String::from("d"), dw.clone());
    moves.insert(String::from("Dw"), dw);

    for name in [
        "x", "y", "z",
        "U", "F", "R", "D", "B", "L",
        "u", "f", "r", "d", "b", "l",
        "Uw", "Fw", "Rw", "Dw", "Bw", "Lw",
        "M", "E", "S",
    ] {
        let st = moves.get(name).unwrap();
        print_state(name, st);

        let st2 = st * st;
        let name2 = String::from(name) + "2";
        print_state(&name2, &st2);

        let st_prime = st.get_prime();
        let name_prime = String::from(name) + "'";
        print_state(&name_prime, &st_prime);
    }
}
