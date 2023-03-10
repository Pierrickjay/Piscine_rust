/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   collatz.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/01 16:53:09 by pjay              #+#    #+#             */
/*   Updated: 2023/02/01 16:56:32 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn collatz(start: u32)
{
	let mut n = start;

	while n > 1
	{
		println!("{n}");
		if n % 2 == 0
		{
			n = n / 2;
		}
		else
		{
			n = 3 * n + 1;
		}
	}
	println!("{n}");
}

fn main ()
{
	collatz(3);
}
