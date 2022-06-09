#[cfg(test)]
mod integration {
    use rusty::*;

    #[test]
    fn test_1() {
        let mut p = Point2::new(1, 2);

        let out1 = p.to_string();
        p.add(2, 1);
        let out2 = p.to_string();

        assert_eq!(out1, "Point2(1, 2)");
        assert_eq!(out2, "Point2(3, 3)");
    }
}
