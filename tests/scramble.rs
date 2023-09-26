use cube;

#[test]
fn test_scramble() {
    let moves = cube::moves::new();

    let solved = cube::State::new();
    let scramble = "U' F' D2 R U2 R' U2 F2 R D2 L2 D2 R' B U' L' B2 D2 B2 U2";
    let cb = solved.apply_moves(&moves, scramble);

    let w = cube::Color::White;
    let g = cube::Color::Green;
    let r = cube::Color::Red;
    let y = cube::Color::Yellow;
    let b = cube::Color::Blue;
    let o = cube::Color::Orange;

    assert_eq!(cb.get_up_colors(), [o, b, y, y, w, b, w, r, g]);
    assert_eq!(cb.get_front_colors(), [r, g, y, y, g, o, w, y, r]);
    assert_eq!(cb.get_right_colors(), [r, o, o, w, r, o, b, w, o]);
    assert_eq!(cb.get_back_colors(), [b, r, w, g, b, g, g, w, b]);
    assert_eq!(cb.get_left_colors(), [b, b, g, w, o, r, r, o, o]);
    assert_eq!(cb.get_down_colors(), [g, g, y, y, y, r, w, b, y]);
}
