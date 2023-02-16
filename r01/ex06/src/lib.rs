
#![allow(dead_code)]

fn check_args(a: &[u8], b: &[u8])
{
	let mut i = 0;
	if a.is_empty() || b.is_empty()
	{
		assert!(false, "they are empty");
	}
	while i < a.len()
	{
		if !(a[i].is_ascii_digit()){
			assert!(false, "they are not ascii digit");}
		i += 1;
	}
	i = 0;

	while i < b.len()
	{
		if !(b[i].is_ascii_digit()){
			assert!(false, "they are not beween 0 and 9");}
		i += 1;
	}

}

fn lets_add(a: &[u8], b: &[u8]) ->u8
{
	let mut i = 0;
	let mut j = 0;
	let mut test_a = 0;
	let mut test_b = 0;
	let total;

	while i < a.len() && a[i] == b'0' {
		i += 1;
	}
	while j< b. len() && b[j] == b'0'{
		j += 1;
	}
	while i < a.len()
	{
		test_a = test_a * 10 + (a[i] - b'0');
		i += 1;
	}
	while j< b. len(){
		test_b = test_b * 10 + (b[j] - b'0');
		j += 1;
	}
	total = test_a + test_b;
	total

}

fn big_add(a: &[u8], b: &[u8]) -> Vec<u8>
{
	let mut vec = Vec::new();
	let mut result;

	check_args(a, b);
	result = lets_add(a, b);
	while result >= 0
	{
		vec.push(result % 10 + b'0');
		result /= 10;
	}
	vec.reverse();
	vec
}

#[cfg(test)]
mod	test {
	use crate:: big_add;

	#[test]
	fn	easy_test() {
		assert!(big_add(b"2", b"4") == b"6");
		assert!(big_add(b"10", b"8") == b"18");
		assert!(big_add(b"100", b"42") == b"142");
	}

	#[test]
	fn	basic_test() {
		assert!(big_add(b"120", b"22") == b"142");
		assert!(big_add(b"83", b"16") == b"99");
		assert!(big_add(b"23", b"32") == b"55");
	}

	#[test]
	fn	zero_test() {
		assert!(big_add(b"42", b"0") == b"42");
		assert!(big_add(b"0", b"42") == b"42");
		assert!(big_add(b"0", b"0") == b"0");
		assert!(big_add(b"0010", b"0200") == b"210");
		assert!(big_add(b"007", b"0") == b"7");
		assert!(big_add(b"00700", b"00000000") == b"700");
	}

	#[test]
	fn	carry_test() {
		assert!(big_add(b"9", b"2") == b"11");
		assert!(big_add(b"99", b"1") == b"100");
		assert!(big_add(b"3284", b"39275") == b"42559");
	}

	#[test]
	fn	big_test() {
		assert!(big_add(b"99999999999999999999999999999999999999999999999999", b"1") == b"100000000000000000000000000000000000000000000000000");
		assert!(big_add(b"9823590280573086239", b"4245958903612337415") == b"14069549184185423654");
		assert!(big_add(b"3627586591049095041", b"4723665748976621977") == b"8351252340025717018");
		assert!(big_add(b"1366252263159600498", b"7289523504808362761") == b"8655775767967963259");
		assert!(big_add(b"6342405623940402827", b"8723378674752413191") == b"15065784298692816018");
		assert!(big_add(b"5315858127012569344", b"5426297946789808780") == b"10742156073802378124");
		assert!(big_add(b"6817757680208716792", b"4488434026902281907") == b"11306191707110998699");
	}

	#[test]
	#[should_panic]
	fn	empty_first_test() {
		big_add(b"", b"2");
	}

	#[test]
	#[should_panic]
	fn	empty_second_test() {
		big_add(b"1", b"");
	}

	#[test]
	#[should_panic]
	fn	empty_both_test() {
		big_add(b"", b"");
	}

	#[test]
	#[should_panic]
	fn	invalid_input_1_test() {
		big_add(b"10s0", b"42");
	}

	#[test]
	#[should_panic]
	fn	invalid_input_2_test() {
		big_add(b"42", b"10s0");
	}

	#[test]
	#[should_panic]
	fn	invalid_input_3_test() {
		big_add(b"10s0", b"10s0");
	}

	#[test]
	#[should_panic]
	fn	invalid_input_4_test() {
		big_add(b"z", b"z");
	}
}
