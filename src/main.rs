const N: usize = 12;
const MASK: u64 = 0b111111111111;

fn cycle(d: u64, i: usize) -> u64 {
  ((d << i) | (d >> (N - i))) & MASK
}

#[test]
fn cycle_test() {
  assert_eq!(2, cycle(1,1));
}

fn matrix(d: u64) -> [u64; N] {
  let mut A: [u64; N] = [0; N];

  for i in 0..N {
    A[i] = cycle(d, i);
  }

  A
}

fn even_parity(mut d: u64) -> bool {
  let parity = d.count_ones() & 1;

	if d == 1 {
		false
	} else {
		true
	}
}

fn is_zd(A: [u64; N]) -> bool {
  for i in 1..(N/2) {
		let comb = A[0] & A[i];
  	if !even_parity(comb) {
			return false
		}
	}
	true
}

fn bracelet() -> u64 {
  1
}

fn main() {
	let d: u64 = 0b101111011;

  for i in matrix(d).iter() {
    println!("{:012b}", i);
  }

	println!("ZD: {}", is_zd(matrix(d)));
}

