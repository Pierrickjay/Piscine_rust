fn strpcmp(query: &[u8], pattern: &[u8]) -> bool
{
	let mut i = 0;
	let mut j = 0;
	let mut compteur = 0;

	while i < query.len() && j < pattern.len()
	{
		if pattern[j] == 42
		{
			while j < pattern.len() && pattern[j] == 42
			{
				j +=1;
			}
			if pattern.len() > j
			{
				while query[i] != pattern[j] && i < query.len()
				{
					i += 1;
					compteur += 1;
				}
				compteur -= 1;
				if i == query.len()
				{

					return false;
				}
			}
			else
			{
				println!("test");
				return true ;
			}
		}
		else if query[i] != pattern[j]
		{
			return false;
		}
		else
		{
			i += 1;
			j += 1;
		}
	}
	println!("query len = {} pattern len = {} compteur = {compteur}", query.len(), pattern.len());
	if query.len() != pattern.len() + compteur
	{
		return false;
	}
	return true
}

fn main() {
    assert!(strpcmp(b"ab8948449894849joujoihohdefg", b"a*d*fg*"));
}
