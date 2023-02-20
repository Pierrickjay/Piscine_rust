#[cfg(test)]
mod    test {
    use crate::num_days_in_month;
    use crate::is_leap_year;

    #[test]
    #[should_panic]
    fn    zero_month_test() {
        num_days_in_month(4242, 0);
    }

    #[test]
    #[should_panic]
    fn    too_much_month_test() {
        num_days_in_month(4242, 13);
    }

    #[test]
    #[should_panic]
    fn    year_zero_test() {
        is_leap_year(0);
    }

    #[test]
    fn    leap_year_test() {
        assert_eq!(is_leap_year(4), true);
        assert_eq!(is_leap_year(40), true);
        assert_eq!(is_leap_year(400), true);
        assert_eq!(is_leap_year(1600), true);
        assert_eq!(is_leap_year(2004), true);
        assert_eq!(is_leap_year(2000), true);
        assert_eq!(is_leap_year(1200), true);
    }

    #[test]
    fn    not_leap_year_test() {
        assert_eq!(is_leap_year(1), false);
        assert_eq!(is_leap_year(1500), false);
        assert_eq!(is_leap_year(2003), false);
        assert_eq!(is_leap_year(42), false);
        assert_eq!(is_leap_year(4100), false);
    }

    #[test]
    fn    num_day_test() {
        assert_eq!(num_days_in_month(1500, 1), 31);
        assert_eq!(num_days_in_month(1600, 1), 31);
        assert_eq!(num_days_in_month(1500, 2), 28);
        assert_eq!(num_days_in_month(1600, 2), 29);
        assert_eq!(num_days_in_month(1500, 3), 31);
        assert_eq!(num_days_in_month(1600, 4), 30);
        assert_eq!(num_days_in_month(1500, 5), 31);
        assert_eq!(num_days_in_month(1600, 6), 30);
        assert_eq!(num_days_in_month(1500, 7), 31);
        assert_eq!(num_days_in_month(1600, 8), 31);
        assert_eq!(num_days_in_month(1500, 9), 30);
        assert_eq!(num_days_in_month(1600, 10), 31);
        assert_eq!(num_days_in_month(1500, 11), 30);
        assert_eq!(num_days_in_month(1600, 12), 31);
    }
}
