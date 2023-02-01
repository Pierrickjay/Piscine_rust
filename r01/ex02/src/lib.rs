/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/01/28 15:37:15 by pjay              #+#    #+#             */
/*   Updated: 2023/01/28 16:27:23 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![allow(dead_code)]

/*
	fn min(a: &i32, b: &i32) -> &i32;
	This old function can t work because when we compile rust can know the the return
	is going to be a or b that we are going to return
	we could return a but if a is declared in a {} and if u acced the result later
	rust is not going to be happy
	Rust have a tools call the borrow checker that need to know the life time of every variable
*/

/*
	<'a> mean we are goint to declare a generic life time annotation
	it s saying that there is  a reliation ship beween ou a and b and the return
	with the mark 'a
	So the life time of the return value will be the same as the smalest
	life time of the parametters
*/


fn min<'a>(a: &'a i32, b: &'a i32) -> &'a i32
{
	if a > b{
		b
	}
	else{
		a}
}

#[test]
fn min_3_5()
{
	assert_eq!(&3, min(&3, &5), "la fonction n'a pas fonctonne");
}
#[test]
fn min_8_5()
{
	assert_eq!(&5, min(&8, &5), "la fonction n'a pas fonctonne");
}
#[test]
fn min_1_666()
{
	assert_eq!(&1, min(&1, &666), "la fonction n'a pas fonctonne");
}
#[test]
fn min_paniqu()
{
	assert_eq!(&3, min(&3, &1), "la fonction n'a pas fonctonne");
}
