/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/06 18:26:42 by marvin            #+#    #+#             */
/*   Updated: 2023/02/07 16:15:50 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![allow(dead_code)]

fn add(a: &i32, b: i32) -> i32
{
	let res = *a + b;
	res
}

fn add_assign(a: &mut i32, b: i32)
{
	*a = *a + b;
}


#[cfg(test)]
mod test {
	use crate::add;
	use crate::add_assign;

	#[test]
	fn add_test() {
		assert_eq!(add(&8, 9),17);
		assert_eq!(add(&42, 42), 84);
		assert_eq!(add(&0, 0), 0);
	}
	#[test]
	fn add_assign_test() {
		let mut a = 42;
		let b = 42;
		add_assign(&mut a, b);
		assert_eq!(a, 84);
	}
}
