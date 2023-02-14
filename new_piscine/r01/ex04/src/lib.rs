/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/09 10:58:40 by pjay              #+#    #+#             */
/*   Updated: 2023/02/14 18:13:53 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![allow(dead_code)]

fn	find_highest_number(boxes: &mut [[u32; 2]]) -> usize
{
	let mut test = 0;
	let mut pos = 0;
	let mut i = 0;
	while i < boxes.len()
	{
		if boxes[i][0] > test{
			test = boxes[i][0];
			pos = i;
		}
		i += 1;
	}
	pos
}

// fn	check_sorted_box(boxes: &mut [[u32; 2]])
// {
// 	fn mut test = 0;
// 	let mut i = 0;
// 	let mut j;
// 	while i < boxes.len()
// 	{
// 		j = 0;
// 		while j < boxes[i].len()
// 		{
// 			if i != 0 && boxes[i][j] > boxes[i - 1][j]
// 			{
// 				j = 0;
// 			}
// 			j += 1;
// 		}
// 		i += 1;
// 	}
// }

fn	sort_boxes(boxes: &mut [[u32; 2]])
{
	let mut i = 1;
	let mut j;
	if boxes.is_empty() == true{std::panic!("The box is empty");}
	j = find_highest_number(boxes);
	boxes.swap(0 , j);
	while i < boxes.len()
	{
		j = find_highest_number(&mut boxes[i..][0..]);
		boxes.swap(j + i , i);
		if boxes[i][0] > boxes[i - 1][0]
		{
			std::panic!();
		}
		println!("{} {}", boxes[i][0], boxes[i][1]);
		i += 1;
	}
}
#[cfg(test)]
mod	sort_boxes {
	use crate::sort_boxes;

	#[test]
	fn	exemple_test() {
		let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
	}

	#[test]
	fn	already_sort_test() {
		let mut	boxes = [[5, 5], [5, 4], [4, 4], [3, 4], [3, 3]];
		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[5, 5], [5, 4], [4, 4], [3, 4], [3, 3]]);
	}

	#[test]
	fn	reverse_test() {
		let mut boxes = [[1, 0], [3, 3], [3, 3], [4, 3], [5, 7]];
		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
	}

	#[test]
	fn	duplicate_test() {
		let mut	boxes = [[1, 1], [2, 3], [1, 1], [2, 3]];
		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[2, 3], [2, 3], [1, 1], [1, 1]]);
	}

	#[test]
	fn	single_box_test() {
		let mut	boxes = [[42, 42]];
		sort_boxes(&mut boxes);
		assert_eq!(boxes, [[42, 42]]);
	}

	#[test]
	#[should_panic(expected = "The box is empty")]
	fn	empty_test() {
		sort_boxes(&mut []);
	}

	#[test]
	#[should_panic(expected = "The boxes cant be sort")]
	fn	impossible_test() {
		let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3], [6, 6]];
		sort_boxes(&mut boxes);
	}
}
