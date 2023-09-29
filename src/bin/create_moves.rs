use std::collections::HashMap;

use cube::State;

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

    let z = solved.apply_arg_moves(&moves, "y y y x y").unwrap();
    moves.insert(String::from("z"), z);


    let u = State::product_of_cycles(&vec![vec![9, 45, 36, 18], vec![10, 46, 37, 19], vec![11, 47, 38, 20],
        vec![0, 2, 8, 6], vec![1, 5, 7, 3]]);
    moves.insert(String::from("U"), u);

    let d = solved.apply_arg_moves(&moves, "x x U x x").unwrap();
    moves.insert(String::from("D"), d);

    let r = solved.apply_arg_moves(&moves, "z z z U z").unwrap();
    moves.insert(String::from("R"), r);

    let l = solved.apply_arg_moves(&moves, "z U z z z").unwrap();
    moves.insert(String::from("L"), l);

    let f = solved.apply_arg_moves(&moves, "x U x x x").unwrap();
    moves.insert(String::from("F"), f);

    let b = solved.apply_arg_moves(&moves, "x x x U x").unwrap();
    moves.insert(String::from("B"), b);


    let m = solved.apply_arg_moves(&moves, "x x x R L L L").unwrap();
    moves.insert(String::from("M"), m);

    let e = solved.apply_arg_moves(&moves, "y y y U D D D").unwrap();
    moves.insert(String::from("E"), e);

    let s = solved.apply_arg_moves(&moves, "z F F F B").unwrap();
    moves.insert(String::from("S"), s);


    let uw = solved.apply_arg_moves(&moves, "U E E E").unwrap();
    moves.insert(String::from("u"), uw.clone());
    moves.insert(String::from("Uw"), uw);

    let fw = solved.apply_arg_moves(&moves, "F S").unwrap();
    moves.insert(String::from("f"), fw.clone());
    moves.insert(String::from("Fw"), fw);

    let rw = solved.apply_arg_moves(&moves, "R M M M").unwrap();
    moves.insert(String::from("r"), rw.clone());
    moves.insert(String::from("Rw"), rw);

    let bw = solved.apply_arg_moves(&moves, "B S S S").unwrap();
    moves.insert(String::from("b"), bw.clone());
    moves.insert(String::from("Bw"), bw);

    let lw = solved.apply_arg_moves(&moves, "L M").unwrap();
    moves.insert(String::from("l"), lw.clone());
    moves.insert(String::from("Lw"), lw);

    let dw = solved.apply_arg_moves(&moves, "D E").unwrap();
    moves.insert(String::from("d"), dw.clone());
    moves.insert(String::from("Dw"), dw);

    let names = [
        "x", "y", "z",
        "U", "F", "R", "D", "B", "L",
        "Uw", "Fw", "Rw", "Dw", "Bw", "Lw",
        "M", "E", "S",
    ];

    println!("pub enum MOVES {{");

    for nm in names {
        let mut nm_iter = nm.chars();
        let name = nm_iter.next().unwrap().to_uppercase().collect::<String>() + nm_iter.as_str();
        let name2 = name.clone() + "2";
        let name_prime = name.clone() + "Prime";

        println!("    {}, {}, {},", name, name2, name_prime);
    }

    println!("}}");


    println!();
    println!("const MOVES_P: [[u8; 54]; {}] = [", (names.len()*3));

    for name in names {
        let st = moves.get(name).unwrap();
        println!("    {:?},", st.get_p());

        let st2 = st * st;
        println!("    {:?},", st2.get_p());

        let st_prime = st.get_prime();
        println!("    {:?},", st_prime.get_p());
    }

    println!("];");


    println!();
    println!("fn name2enum(name: &str) -> Option<MOVES> {{");
    println!("    match name {{");

    for nm in names {
        let mut nm_iter = nm.chars();
        let name = nm_iter.next().unwrap().to_uppercase().collect::<String>() + nm_iter.as_str();
        let name2 = name.clone() + "2";
        let name_prime = name.clone() + "Prime";

        println!("        \"{}\" => Some(MOVES::{}),", nm, name);
        println!("        \"{}\" => Some(MOVES::{}),", String::from(nm) + "2", name2);
        println!("        \"{}\" => Some(MOVES::{}),", String::from(nm) + "'", name_prime);

        if nm == "Uw" || nm == "Fw" || nm == "Rw" || nm == "Dw" || nm == "Bw" || nm == "Lw" {
            let mut w_nm_iter = nm.chars();
            let w_name = w_nm_iter.next().unwrap().to_lowercase().collect::<String>();

            println!("        \"{}\" => Some(MOVES::{}),", &w_name, name);
            println!("        \"{}\" => Some(MOVES::{}),", String::from(&w_name) + "2", name2);
            println!("        \"{}\" => Some(MOVES::{}),", String::from(&w_name) + "'", name_prime);
        }
    }

    println!("        _  => None,");
    println!("    }}");
    println!("}}");
}
