use cube;

#[test]
fn test_period() {
    // 何も操作しなければ0回で元に戻る
    let solved = cube::State::new_solved();
    assert_eq!(solved.get_period(), 0);

    // Rは4回で元に戻る
    let r = &solved * "R";
    assert_eq!(r.get_period(), 4);

    // Rを63回、U'を63回の計126回で元に戻る
    let rup = &solved * "R U'";
    assert_eq!(rup.get_period(), 63);

    // Ubパームは3回で元に戻る
    let ub = &solved * "M2 U M U2 M' U M2";
    assert_eq!(ub.get_period(), 3);

    // OLL 17は18回で元に戻る
    let oll17 = &solved * "R U R' U R' F R F' U2 R' F R F'";
    assert_eq!(oll17.get_period(), 18);
}
