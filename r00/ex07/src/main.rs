/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/01/26 15:17:21 by pjay              #+#    #+#             */
/*   Updated: 2023/01/27 15:27:36 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::cmp::Ordering;

fn pick_an_object()
{
	let object = "voiture trop style avec des neons";
	let price : i32;
	let mut test : i32;
	let mut compteur = 0;
	price = ftkit::random_number(0..1000000);
	// println!("{} coute {}", object, price);
	loop
	{
		println!("{} coute ?", object);
		test = ftkit::read_number();
		match test.cmp(&price)
		{
			Ordering::Equal => println!("ET CEST LE JUSTE PRIX !! congratulation you won a {}", object),
			Ordering::Greater => println!("Vous etes trop haut chef"),
			Ordering::Less => println!("Vous etes trop bas chef"),
		}
		if price == test{break}
		compteur += 1;
	}
	println!("{} couts", compteur);
}

fn main() {
    pick_an_object();
}
