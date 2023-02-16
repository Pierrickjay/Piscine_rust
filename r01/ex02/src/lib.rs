/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/07 16:31:51 by pjay              #+#    #+#             */
/*   Updated: 2023/02/08 14:48:13 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

//assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);

#![allow(dead_code)]
const fn color_name(color: & [u8; 3]) -> &'static str
{
	match color {
	[0, 0, 0] =>  "pure black",
	[255, 255, 255] =>  "pure white",
	[255, 0, 0] =>  "pure red",
	[0, 255, 0] =>  "pure green",
	[0, 0, 255] => "pure blue",
	[128, 128, 128] =>  "perfect grey",
	[0..=30, 0..=30, 0..=30] => "almost black",
	[128..=255, 0..=127, 0..=127] => "redish",
	[0..=127, 128..=255, 0..=127] => "greenish",
	[0..=127, 0..=127, 128..=255] => "blueish",
	_ => "unknown"}
}

#[cfg(test)]
#[test]
fn test_lifetimes() {
	let name_of_the_best_color;
	{
		let the_best_color = [42, 42, 42];
		name_of_the_best_color = color_name(&the_best_color);
	}
	assert_eq!(name_of_the_best_color, "unknown");
}
#[test]
fn collor_test(){
	assert_eq!(color_name(&[0,0,0]), "pure black");
	assert_eq!(color_name(&[255, 255, 255]), "pure white");
	assert_eq!(color_name(&[255, 0, 0]), "pure red");
	assert_eq!(color_name(&[0, 255, 0]), "pure green");
	assert_eq!(color_name(&[0, 0, 255]), "pure blue");
	assert_eq!(color_name(&[128, 128, 128]), "perfect grey");
	assert_eq!(color_name(&[30, 30, 30]), "almost black");
	assert_eq!(color_name(&[129, 30, 30]), "redish");
	assert_eq!(color_name(&[30,129, 30]), "greenish");
	assert_eq!(color_name(&[30,11, 129]), "blueish");
	assert_eq!(color_name(&[255,255,0]), "unknown");
}

	#[test]
	fn	pure_black_test() {
		assert!(color_name(&[0, 0, 0]) == "pure black", "Pure black test failed");
	}

	#[test]
	fn	pure_white_test() {
		assert!(color_name(&[255, 255, 255]) == "pure white", "Pure white test failed");
	}

	#[test]
	fn	pure_red_test() {
		assert!(color_name(&[255, 0, 0]) == "pure red", "Pure red test failed");
	}

	#[test]
	fn	pure_green_test() {
		assert!(color_name(&[0, 255, 0]) == "pure green", "Pure green test failed");
	}

	#[test]
	fn	pure_blue_test() {
		assert!(color_name(&[0, 0, 255]) == "pure blue", "Pure blue test failed");
	}

	#[test]
	fn	perfect_grey_test() {
		assert!(color_name(&[128, 128, 128]) == "perfect grey", "Perfect grey test failed");
	}

	#[test]
	fn	almost_black_test() {
		assert!(color_name(&[30, 30, 30]) == "almost black", "Almost black test failed");
		assert!(color_name(&[0, 0, 30]) == "almost black", "Almost black test failed");
		assert!(color_name(&[30, 0, 00]) == "almost black", "Almost black test failed");
		assert!(color_name(&[0, 30, 0]) == "almost black", "Almost black test failed");
		assert!(color_name(&[1, 1, 1]) == "almost black", "Almost black test failed");
	}

	#[test]
	fn	redish_test() {
		assert!(color_name(&[128, 127, 127]) == "redish", "Redish test failed");
		assert!(color_name(&[255, 127, 0]) == "redish", "Redish test failed");
		assert!(color_name(&[255, 0, 127]) == "redish", "Redish test failed");
		assert!(color_name(&[255, 1, 1]) == "redish", "Redish test failed");
		assert!(color_name(&[128, 0, 0]) == "redish", "Redish test failed");
		assert!(color_name(&[128, 0, 1]) == "redish", "Redish test failed");
		assert!(color_name(&[128, 1, 0]) == "redish", "Redish test failed");
	}

	#[test]
	fn	greenish_test() {
		assert!(color_name(&[127, 128, 127]) == "greenish", "Greenish test failed");
		assert!(color_name(&[127, 255, 0]) == "greenish", "Greenish test failed");
		assert!(color_name(&[0, 255, 127]) == "greenish", "Greenish test failed");
		assert!(color_name(&[1, 255, 1]) == "greenish", "Greenish test failed");
		assert!(color_name(&[0, 128, 0]) == "greenish", "Greenish test failed");
		assert!(color_name(&[0, 128, 1]) == "greenish", "Greenish test failed");
		assert!(color_name(&[1, 128, 0]) == "greenish", "Greenish test failed");
	}

	#[test]
	fn	blueish_test() {
		assert!(color_name(&[127, 127, 128]) == "blueish", "Blueish test failed");
		assert!(color_name(&[127, 0, 255]) == "blueish", "Blueish test failed");
		assert!(color_name(&[0, 127, 255]) == "blueish", "Blueish test failed");
		assert!(color_name(&[1, 1, 255]) == "blueish", "Blueish test failed");
		assert!(color_name(&[0, 0, 128]) == "blueish", "Blueish test failed");
		assert!(color_name(&[0, 1, 128]) == "blueish", "Blueish test failed");
		assert!(color_name(&[1, 0, 128]) == "blueish", "Blueish test failed");
	}

	#[test]
	fn	unknown_test() {
		assert!(color_name(&[255, 255, 0]) == "unknown", "Unknown test failed");
		assert!(color_name(&[255, 0, 255]) == "unknown", "Unknown test failed");
		assert!(color_name(&[0, 255, 255]) == "unknown", "Unknown test failed");
		assert!(color_name(&[32, 32, 32]) == "unknown", "Unknown test failed");
		assert!(color_name(&[42, 42, 42]) == "unknown", "Unknown test failed");
		assert!(color_name(&[127, 127, 127]) == "unknown", "Unknown test failed");
}
