/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   fizzbuzz.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: pjay <marvin@42.fr>                        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/01/26 12:03:07 by pjay              #+#    #+#             */
/*   Updated: 2023/01/26 13:38:43 by pjay             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn	fizzbuzz()
{
	let mut i = 1;

	while i <= 100
	{
		match (i % 3, i % 5)
		{
			(0, 0) =>println!("write(1,\"fizzbuzz\\n\",9);"),
			(0, _) =>println!("write(1,\"fizz\\n\",5);"),
			(_, 0) =>println!("write(1,\"buzz\\n\",5);"),
			_ =>match i > 10
			{
				true =>println!("write(1,\"{}\\n\", 3);", i),
				_ =>println!("write(1,\"{}\\n\", 2);", i),
			}
		}
		i += 1
	}
		// if i % 3 == 0 && i % 5 == 0{println!("write(1,\"fizzbuzz\\n\",9);")}
		// else if i % 3 == 0{println!("write(1,\"fizz\\n\",5);")}
		// else if i % 5 == 0{println!("write(1,\"buzz\\n\",5);")}
		// else{
		// 	if i < 10{compteur = 2}
		// 	else{compteur = 3}
		// 	println!("write(1,\"{}\\n\",{});", i, compteur)}
		// i += 1
}



fn main()
{
	println!("#include <unistd.h>");
	print!("int main()\n{{");
	fizzbuzz();
	print!("}}");
}
