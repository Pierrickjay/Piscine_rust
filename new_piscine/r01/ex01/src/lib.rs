/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/07 16:23:46 by pjay              #+#    #+#             */
/*   Updated: 2023/02/07 16:29:21 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


#![allow(dead_code)]

fn min<'a>(a: &'a i32, b: &'a i32) -> &'a i32
{
	if *a > *b{
		return &b
	}
	else
	{
		return &a
	}

}

#[cfg(test)]
mod test {

	use crate::min;
	#[test]
	fn min_of() {
		let a = 19;
		let b = 15;
		assert_eq!(min(&a, &b), &b);
	}
}