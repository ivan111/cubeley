fn main() {
    let solved = cube::State::new_solved();
    let r = &solved * "R";
    let rud = &r * "U D";

    r.print();
    rud.print();
}
