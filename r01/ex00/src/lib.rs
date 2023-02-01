/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/01/28 14:44:19 by pjay              #+#    #+#             */
/*   Updated: 2023/01/28 15:28:50 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![allow(dead_code)]

fn add(a: &i32, b: i32) -> i32
{
	let c :i32;

	c = *a + b;

	c

}

fn add_assign(a: &mut i32, b: i32)
{
	*a = *a + b;
}

#[test]
fn add_3_5()
{
	assert_eq!(8, add(&3, 5), "la fonction n'a pas fonctonne");
}
#[test]
fn add_500_500()
{
	assert_eq!(0, add(&-500, 500), "la fonction n'a pas fonctonne");
}
#[test]
fn make_it_panic()
{
	assert_eq!(15, add(&-500, 500), "la fonction n'a pas fonctonne");
}
#[test]
fn assign_3_5()
{
	let mut a = 3;
	add_assign(&mut a, 5);
	assert_eq!(8, a , "la fonction n'a pas fonctonne");
}
#[test]
fn assign_500_500()
{
	let mut a = 500;
	add_assign(&mut a, 500);
	assert_eq!(1000, a , "la fonction n'a pas fonctonne");
}
#[test]
fn make_it_panic_w_ass()
{
	let mut a = 3;
	add_assign(&mut a, 5);
	assert_eq!(18, a , "la fonction n'a pas fonctonne");
}
