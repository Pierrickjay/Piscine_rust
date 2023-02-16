/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/08 12:08:33 by pjay              #+#    #+#             */
/*   Updated: 2023/02/09 10:52:10 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![allow(dead_code)]
fn	count_slice(haystack: &[u32], needle: &[u32]) -> i32
{
	let mut i = 0;
	let mut j;
	let mut tmp;
	let mut count = 0;
	while i < haystack.len()
	{
		j = 0;
		tmp = 0;
		while j < needle.len(){
			//println!("{} == {}",haystack[i], needle[j]);
			if haystack[i] == needle[j]
			{
				count += 1;
				tmp += 1;
			}
			j += 1;
		}
		i += 1;
		if tmp == 0{
			//println!("count = {count}");
			return count;
		}
	}
	//println!("count = {count}");
	return count;
}

fn	return_array<'a>(haystack: &'a[u32], needle: &[u32]) -> &'a[u32]
{
	let mut i = 0;
	let mut j;
	let mut tmp;

	while i < haystack.len()
	{
		tmp = 0;
		j = 0;
		while j < needle.len(){
			if haystack[i] == needle[j]
			{
				tmp += 1;
			}
			j += 1;
		}
		if tmp == 0{
			return &haystack[..i];
		}
		i += 1;
	}
	return &haystack[..haystack.len()- 1];
}

fn largest_group<'a>(haystack: &'a[u32], needle: &[u32]) -> &'a[u32]
{
	let mut i = 0;
	let mut j;
	let mut largest_slice = 0;
	let mut pos = 0;

	// if needle.is_empty(){
	// 	return haystack[];
	// }
	while i < haystack.len()
	{
		j = 0;
		while j < needle.len(){

			if haystack[i] == needle[j]
			{

				if largest_slice  <= count_slice(&haystack[i..], &needle){
					//println!("enter here ?");
					pos = i;
					largest_slice = count_slice(&haystack[i..], &needle);
				}
			}
			j += 1;
		}
		i += 1;
	}
	return return_array(&haystack[pos..], needle);
}



#[cfg(test)]
#[test]

fn test_lifetimes() {
	let haystack = [1, 2, 3, 2, 1];
	let result;
	{
	let needle = [2, 3];
	result = largest_group(&haystack, &needle);
	}
	assert_eq!(result, &[2, 3, 2]);
}
#[test]
fn largest_test(){
	assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
	assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5]), &[5, 5]);
	assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[]), &[]);
	assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[4, 1]), &[]);
}

