fn rotate2(s: &str, n: &i32) -> String {
    let len = s.len() as i32;
    
    let shift = ((n % len) + len) % len;

    let (left, right) = s.split_at((len - shift) as usize);
    format!("{}{}", right, left)
}

#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts.iter().for_each(|(n, exp)| {
        assert_eq!(rotate2(s, n), exp.to_string());
    });
}
