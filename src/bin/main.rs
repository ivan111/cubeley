fn main() {
    let moves = cube::moves::new();

    let r = moves.get("R").unwrap();
    let rp = r.apply_moves(&moves, "U D");

    r.print();
    rp.print();
}
