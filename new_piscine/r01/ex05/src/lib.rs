
#![allow(dead_code)]

fn deduplicate(list: &mut Vec<i32>)
{
	let mut i = 1;
	let mut j;

	while i < list.len()
	{
		j = i;
		while j > 0
		{
			if list[i] == list[j]
			{
				list.remove(i);
				break ;
			}
			j -= 1;
		}
		i += 1;
	}
}


#[cfg(test)]
#[test]
fn	test_init()
{
	let mut v = vec![1, 2, 2, 3, 2, 4, 3];
	deduplicate(&mut v);
	assert_eq!(v, [1, 2, 3, 4]);
}
#[test]
fn	test_init_2()
{
	let mut v = vec![1, 2, 2, 3, 2, 4, 3];
	deduplicate(&mut v);
	assert_eq!(v, [1, 3, 4]);
}
