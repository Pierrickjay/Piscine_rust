/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/05 13:53:56 by pjay              #+#    #+#             */
/*   Updated: 2023/02/06 09:07:23 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![allow(dead_code)]
pub fn is_leap_year(year: u32) -> bool
{
	if year == 0
	{
		panic!("Year 0 doesn't work");
	}
	// println!("% 4 = {} % 100 = {} % 400 = {}",year % 4, year % 100, year % 400);
	if year % 4 == 0
	{
		if year % 100 == 0
		{
			if year % 400 == 0
			{
				true
			}
			else {
				false
			}
		}
		else{
			true
		}
	}
	else{
		false
	}
}

pub fn num_days_in_month(year: u32, month: u32) -> u32
{
	if month > 12 || month == 0{
		panic!("The month cannot be greater than 12 and equal or less than 0");
	}
	match month {
		1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
		9 | 11 | 6 | 4 => 30,
		_ => match is_leap_year(year)
		{
			true => 29,
			_ => 28,
		}
	}
}

pub fn number_to_letter(month : u32) -> &'static str
{
	match month {
		1 => "January",
		2 => "February",
		3 => "March",
		4 => "April",
		5 => "May",
		6 => "June",
		7 => "July",
		8 => "August",
		9 => "September",
		10 => "October",
		11 => "November",
		_ => "December",
	}
}

// #[test]
// fn is_leap_year_1600()
// {
// 	assert_eq!(is_leap_year(1600), true,"Panique 1600 is bsextil");
// }

// #[test]
// fn is_leap_year_1500_is_not_leap()
// {
// 	assert_eq!(is_leap_year(1500), false, "Panique 1500 is not bsextil");
// }

// #[test]
// fn is_leap_year_2004_is_leap()
// {
// 	assert_eq!(is_leap_year(2004), true, "Panique 2004 is bsextil");
// }

// #[test]
// fn is_leap_year_2003_is_not_leap()
// {
// 	assert_eq!(is_leap_year(2003), false, "Panique  2003 is not bsextil");
// }

// #[test]
// fn num_days_in_month_2_2023()
// {
// 	assert_eq!(num_days_in_month(2023, 2), 28);
// }
// #[test]
// fn num_days_in_month_1_1()
// {
// 	assert_eq!(num_days_in_month(1, 1), 31);
// }

// #[test]
// fn num_days_in_month_6_1994()
// {
// 	assert_eq!(num_days_in_month(1994, 6), 30);
// }

// #[test]
// #[should_panic(expected = "The month cannot be greater than 12")]
// fn num_days_in_month_18_2023()
// {
// 	assert_eq!(num_days_in_month(2023, 18), 30);
// }

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
