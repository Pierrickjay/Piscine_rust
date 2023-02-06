/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/05 14:24:42 by pjay              #+#    #+#             */
/*   Updated: 2023/02/05 16:06:35 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[warn(special_module_name)]

use ex05::number_to_letter;
use ex05::num_days_in_month;

fn fryday_the_13th()
{
	let mut year = 1;
	let mut month = 1;
	let mut day = 1;
	let mut i = 1;

	loop {
		while month <= 12
		{
			//println!("day of month {} = {}", month, num_days_in_month(year, month));
			while day <= num_days_in_month(year, month)
			{

				if i == 5 && day == 13
				{
					println!("Friday, {} {}, {}", number_to_letter(month), day, year);
				}
				if i == 7
				{
					i = 0;
				}
				day += 1;
				i += 1;
			}
			day = 1;
			month += 1;

		}
		if year == 6
		{
			break ;
		}
		month = 1;
		year += 1;
	}
}
fn main()
{
	fryday_the_13th();
}
