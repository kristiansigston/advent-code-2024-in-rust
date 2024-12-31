// test check cross mas

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_cross_mas() {
        let word_search = vec![
            vec!['M', 'A', 'S'],
            vec!['A', 'A', 'A'],
            vec!['S', 'A', 'S'],
        ];
        assert_eq!(check_cross_mas(&word_search, 0, 0, 1, 1), true);
        assert_eq!(check_cross_mas(&word_search, 0, 0, 1, 0), false);
        assert_eq!(check_cross_mas(&word_search, 0, 0, 0, 1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 0, 1, -1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 0, -1, 1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 0, -1, -1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 0, 0, -1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 0, -1, 0), false);
        assert_eq!(check_cross_mas(&word_search, 0, 0, 0, 0), false);
        assert_eq!(check_cross_mas(&word_search, 0, 1, 1, 1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 1, 1, 0), false);
        assert_eq!(check_cross_mas(&word_search, 0, 1, 0, 1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 1, 1, -1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 1, -1, 1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 1, -1, -1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 1, 0, -1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 1, -1, 0), false);
        assert_eq!(check_cross_mas(&word_search, 0, 1, 0, 1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 2, 1, 1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 2, 1, 0), false);
        assert_eq!(check_cross_mas(&word_search, 0, 2, 0, 1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 2, 1, -1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 2, -1, 1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 2, -1, -1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 2, 0, -1), false);
        assert_eq!(check_cross_mas(&word_search, 0, 2, -1, 0), false);
        assert_eq!(check_cross_mas(&word_search, 0, 2, 0, 1), false);
        assert_eq!(check_cross_mas(&word_search, 1, 0, 1, 1), false);
        assert_eq!(check_cross_mas(&word_search, 1, 0, 1, 0), false);
        assert_eq!(check_cross_mas(&word_search, 1, 0, 0, 1), false);
        assert_eq!(check_cross_mas(&word_search, 1, 0, 1, -1), false);
        assert_eq!(check_cross_mas(&word_search, 1, 0, -1, 1), false);
        assert_eq!(check_cross_mas(&word_search, 1, 0, -1, -1), false);
        assert_eq!(check_cross_mas(&word_search, 1, 0, 0, -1), false);
        assert_eq!(check_cross_mas(&word_search, 1, 0, -1, 0), false);
    }
}
