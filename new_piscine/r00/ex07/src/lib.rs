/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/02/05 16:25:40 by pjay              #+#    #+#             */
/*   Updated: 2023/02/06 11:53:30 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![allow(dead_code)]

// fn find_patern(query: &[u8], pattern: &[u8]) -> usize
// {
// 	let mut position = 0;
// 	let mut i = 0;
// 	let mut j = 0;

// 	while i < query.len() && j < pattern.len()
// 	{
// 		if query[i] == pattern[j]
// 		{
// 			position = i;
// 			while query[i] == pattern[j]{
// 				if pattern[i] == 42{
// 					find_patern(query[i..], pattern[j..])
// 				}
// 				i += 1
// 				j += 1
// 			}
// 		}
// 	}
// }

fn check_asterix(pattern: &[u8], position: usize) -> bool
{
	let mut index = position;
	while index < pattern.len()
	{
		if pattern[index] != 42
		{
			return false
		}
		index += 1;
	}
	return true;
}

fn check_cond_len(query: &[u8], pattern: &[u8]) -> bool
{
	if query.is_empty() && !pattern.is_empty() && check_asterix(pattern, 0) == true
	{
		return true
	}
	if query.is_empty() && pattern.is_empty()
	{
		println!("enter here");
		return true
	}
	if query.is_empty() && !pattern.is_empty() && check_asterix(pattern, 0) == false
	{
		return false
	}
	return false ;
}

fn strpcmp(query: &[u8], pattern: &[u8]) -> bool
{
	let mut i = 0;
	let mut j = 0;
	//let mut compteur = 0;

	if query.len() != pattern.len()
	{
		if check_cond_len(query, pattern) == true
		{
			println!("out here 1");
			return true
		}
		if !query.is_empty() && pattern.is_empty() || query.is_empty() && !pattern.is_empty() && check_asterix(pattern, 0) == false
		{

			return false
		}
	}
	while i < query.len() && j < pattern.len()
	{
		if pattern[j] == 42
		{
			while j < pattern.len() && pattern[j] == 42 {
				j += 1;
			}
			//println!("{j} && pattern.len = {}", pattern.len());
			if pattern.len() > j
			{
				while i < query.len() && query[i] != pattern[j] {
					i += 1;
					//compteur += 1;
				}
				//println!{"query[i] = {} pattern[j] ={}", query[i], pattern[j]}
				//compteur -= 1;
				if i == query.len() {
					println!("out here 1");
					return false;
				}
			}
			else {
				return true ;
			}
		}
		else if query[i] != pattern[j]
		{
			println!("out here 2 ");
			return false;
		}
		else
		{
			i += 1;
			j += 1;
		}
	}
	if i == query.len() && j  < pattern.len() || j == pattern.len() && i < query.len(){
		println!("out here4 ");
		return false
	}
	println!("out here final");
	return true
}
#[test]
    fn ultimate() {
        assert!(strpcmp(b"abbcdefcdeeeffef", b"ab*cd*ef"));
    }

#[cfg(test)]
mod test {
    use crate::strpcmp;

    #[test]
    fn basic_test() {
        assert!(strpcmp(b"salut", b"salut"));
        assert!(strpcmp(b"42", b"42"));
        assert!(strpcmp(b"abc", b"abc"));
        assert!(!strpcmp(b"abc", b"abd"));
        assert!(!strpcmp(b"abc", b"bbc"));
    }

    #[test]
    fn wildcard() {
        assert!(strpcmp(b"abc", b"*"));
        assert!(strpcmp(b"abc", b"a*"));
        assert!(strpcmp(b"abc", b"ab*"));
        assert!(strpcmp(b"abc", b"******"));
        assert!(strpcmp(b"abc", b"a*****c"));
        assert!(strpcmp(b"abc", b"a***b**c"));
        assert!(!strpcmp(b"abc", b"a*abc"));
        assert!(!strpcmp(b"abc", b"a*bb*cc"));
        assert!(strpcmp(b"ab000cd", b"ab*cd"));
        assert!(strpcmp(b"abcd", b"ab*cd"));
    }

    #[test]
    fn empty() {
        assert!(strpcmp(b"", b""));
        assert!(strpcmp(b"", b"*"));
        assert!(strpcmp(b"", b"**********"));
        assert!(!strpcmp(b"", b"a*"));
        assert!(!strpcmp(b"", b"*a*"));
        assert!(!strpcmp(b"", b"*a"));
    }


}
#[test]
fn strcmp_abc()
{
	assert_eq!(strpcmp(b"abc", b"abc"), true, "ca panique fort");
}

#[test]
fn strcmp_abcd()
{
	assert_eq!(strpcmp(b"abcd", b"ab*"), true, "ca panique fort");
}

#[test]
fn strcmp_cab()
{
	assert_eq!(strpcmp(b"cab", b"ab*"), false, "ca panique fort");
}

#[test]
fn strcmp_dcab()
{
	assert_eq!(strpcmp(b"dcab", b"*ab"), true, "ca panique fort");
}

#[test]
fn strcmp_abc_ab()
{
	assert_eq!(strpcmp(b"abc", b"*ab"), false, "ca panique fort");
}

#[test]
fn caca()
{
	assert_eq!(strpcmp(b"salut", b"sal"), false, "ca caca fort");
}

#[test]
fn strcmp_ab0()
{
	assert_eq!(strpcmp(b"ab000cd", b"ab*cd"), true, "ca panique fort");
}

#[test]
fn strcmp_abcd_ab()
{
	assert_eq!(strpcmp(b"abcd", b"ab*cd"), true, "ca panique fort");
}
