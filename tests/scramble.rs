use cube;

#[test]
fn test_scramble() {
    let moves = cube::moves::new();

    let solved = &cube::State::SOLVED;
    let scramble = "U' F' D2 R U2 R' U2 F2 R D2 L2 D2 R' B U' L' B2 D2 B2 U2";
    let cb = solved.apply_moves(&moves, scramble);

    let w = cube::Color::White;
    let g = cube::Color::Green;
    let r = cube::Color::Red;
    let y = cube::Color::Yellow;
    let b = cube::Color::Blue;
    let o = cube::Color::Orange;

    assert_eq!(cb.get_face_colors(cube::Face::Up), [o, b, y, y, w, b, w, r, g]);
    assert_eq!(cb.get_face_colors(cube::Face::Front), [r, g, y, y, g, o, w, y, r]);
    assert_eq!(cb.get_face_colors(cube::Face::Right), [r, o, o, w, r, o, b, w, o]);
    assert_eq!(cb.get_face_colors(cube::Face::Down), [g, g, y, y, y, r, w, b, y]);
    assert_eq!(cb.get_face_colors(cube::Face::Back), [b, r, w, g, b, g, g, w, b]);
    assert_eq!(cb.get_face_colors(cube::Face::Left), [b, b, g, w, o, r, r, o, o]);
}

#[test]
fn test_checkered_pattern() {
    let moves = cube::moves::new();

    let solved = &cube::State::SOLVED;
    let scramble = "M2 E2 S2";
    let cb = solved.apply_moves(&moves, scramble);

    let w = cube::Color::White;
    let g = cube::Color::Green;
    let r = cube::Color::Red;
    let y = cube::Color::Yellow;
    let b = cube::Color::Blue;
    let o = cube::Color::Orange;

    assert_eq!(cb.get_face_colors(cube::Face::Up), [w, y, w, y, w, y, w, y, w]);
    assert_eq!(cb.get_face_colors(cube::Face::Front), [g, b, g, b, g, b, g, b, g]);
    assert_eq!(cb.get_face_colors(cube::Face::Right), [r, o, r, o, r, o, r, o, r]);
    assert_eq!(cb.get_face_colors(cube::Face::Down), [y, w, y, w, y, w, y, w, y]);
    assert_eq!(cb.get_face_colors(cube::Face::Back), [b, g, b, g, b, g, b, g, b]);
    assert_eq!(cb.get_face_colors(cube::Face::Left), [o, r, o, r, o, r, o, r, o]);
}
