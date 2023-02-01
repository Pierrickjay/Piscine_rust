/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/01/28 16:34:48 by pjay              #+#    #+#             */
/*   Updated: 2023/01/28 16:47:13 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![allow(dead_code)]

fn add_vectors(a: [i32; 3], b: [i32; 3]) -> [i32; 3]
{
	let mut vecors : [i32; 3] = [0, 0, 0];

	for i in 0..3
	{
		vecors[i] = a[i] + b[i];
	}
	vecors
}

#[test]
fn assign_3_5()
{

	let a = [8, 8, 8];
	let b = [8, 8, 8];
	assert_eq!(add_vectors(a, b), [16, 16, 16]," time to panique");
}

#[test]
fn panique()
{

	let a = [1, 2, 3];
	let b = [2, 3, 15];
	assert_eq!(add_vectors(a, b), [3, 5, 7]," time to panique");
}

#[test]
fn assign_3_5_7()
{
	let a = [1, 2, 3];
	let b = [2, 3, 4];
	assert_eq!(add_vectors(a, b), [3, 5, 7]," time to panique");
}
