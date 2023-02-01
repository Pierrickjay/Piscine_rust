/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/01/28 18:12:59 by pjay              #+#    #+#             */
/*   Updated: 2023/01/31 16:22:05 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![allow(dead_code)]


fn sum_all<'a>(slice: &'a[u32], threshold: &'a u32)-> i32
{
	let mut sum :i32 = 0;
	for item in slice
	{
		sum = slice[item as usize] + sum;
	}
	println!("{sum}");
	sum
}

// fn smallest_subslice(slice: &[u32], threshold: &u32) -> &[u32]
// {

// }


#[test]
fn	test_file()
{
	let array  = [3, 4, 1, 2, 12];
	let threshold : u32 = 10;
	assert_eq!(sum_all(&array, &threshold), 22, "pas le bon resultat");

}
// fn test_lifetimes() {
//     let array = [3, 4, 1, 2, 12];
//     let result;

//     {
//         let threshold = 1000;
//         result = smallest_subslice(&array, &threshold);
//     }

//     assert_eq!(result, &[]);
// }
