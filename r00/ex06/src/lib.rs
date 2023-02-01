/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/01/26 13:56:29 by pjay              #+#    #+#             */
/*   Updated: 2023/01/26 15:39:01 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![allow(dead_code)]

fn fibs(n: u32) -> u32 {
    (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
}

fn is_prime(n: u32) -> bool {
    n >= 2 && !(2..n).any(|d| n % d == 0)
}

#[test]
fn zero_is_not_prime(){
	assert_eq!(false , is_prime(0));
}
#[test]
fn one_is_not_prime(){
	assert_eq!(false, is_prime(1));
}
#[test]
fn three_is_not_prime(){
	assert_eq!(true, is_prime(3));
}
#[test]
fn four_is_not_prime(){
	assert_eq!(false, is_prime(4));
}
#[test]
fn first_fib_is_0(){
	assert_eq!(0, fibs(0));
}
#[test]
fn fifth_fib_is_3(){
	assert_eq!(3, fibs(4));
}
#[test]
fn seventeenth_fib_is_987(){
	assert_eq!(987, fibs(16));
}

