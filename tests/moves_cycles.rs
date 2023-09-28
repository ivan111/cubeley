use cube;

#[test]
fn test_cycles() {
    let moves = cube::moves::new();

    assert_eq!(moves.get("U").unwrap().get_cycles(),
        vec![vec![0, 2, 8, 6], vec![1, 5, 7, 3], vec![9, 45, 36, 18], vec![10, 46, 37, 19], vec![11, 47, 38, 20]]);

    assert_eq!(moves.get("F").unwrap().get_cycles(),
        vec![vec![6, 18, 29, 53], vec![7, 21, 28, 50], vec![8, 24, 27, 47], vec![9, 11, 17, 15], vec![10, 14, 16, 12]]);

    assert_eq!(moves.get("R").unwrap().get_cycles(),
        vec![vec![2, 42, 29, 11], vec![5, 39, 32, 14], vec![8, 36, 35, 17], vec![18, 20, 26, 24], vec![19, 23, 25, 21]]);

    assert_eq!(moves.get("D").unwrap().get_cycles(),
        vec![vec![15, 24, 42, 51], vec![16, 25, 43, 52], vec![17, 26, 44, 53], vec![27, 29, 35, 33], vec![28, 32, 34, 30]]);

    assert_eq!(moves.get("B").unwrap().get_cycles(),
        vec![vec![0, 51, 35, 20], vec![1, 48, 34, 23], vec![2, 45, 33, 26], vec![36, 38, 44, 42], vec![37, 41, 43, 39]]);

    assert_eq!(moves.get("L").unwrap().get_cycles(),
        vec![vec![0, 9, 27, 44], vec![3, 12, 30, 41], vec![6, 15, 33, 38], vec![45, 47, 53, 51], vec![46, 50, 52, 48]]);
}
