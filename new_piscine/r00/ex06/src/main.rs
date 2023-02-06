/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/05 16:14:24 by pjay              #+#    #+#             */
/*   Updated: 2023/02/05 16:19:27 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


use std::cmp::Ordering;

fn pick_an_object()
{
	let price : i32;
	let mut test : i32;
	price = ftkit::random_number(0..100);
	println!("Me and my infinite wisdom have found an appropriate secret you shall yearn for");
	loop
	{
		test = ftkit::read_number();
		match test.cmp(&price)
		{
			Ordering::Equal => break,
			Ordering::Greater => println!("This student might not be as smart as I was told. This answer is obviously too weak"),
			Ordering::Less => println!("Sometimes I wonder whether I should retire. I would have guessed higher."),
		}
	}
	println!("That is right! The secret was indeed the number {}, which you have brilliantly discovered!", price);
}

fn main() {
    pick_an_object();
}
