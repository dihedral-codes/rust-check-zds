const K: usize = 12;
const D: usize = 8;
const MASK: u64 = 0xfff;

fn cycle(d: u64, i: usize) -> u64 {
  ((d << i) | (d >> (K - i))) & MASK
}

#[test]
fn cycle_test() {
  assert_eq!(2, cycle(1,1));
}

fn matrix(d: u64) -> [u64; K] {
  let mut a: [u64; K] = [0; K];

  for i in 0..K {
    a[i] = cycle(d, i);
  }

  a
}

fn even_parity(d: u64) -> bool {
  let parity = d.count_ones() & 1;

  if parity == 1 {
    false
  } else {
    true
  }
}

fn is_zd(a: [u64; K]) -> bool {
  for i in 1..(K/2) {
    let comb = a[0] & a[i];
    if !even_parity(comb) {
      return false
    }
  }
  true
}


struct Bracelets {
  curr: u64
  , mask: u64
}

impl Bracelets {

  fn new(k: usize, w: usize) -> Bracelets  {

    // Set the bit mask for k bits.
    let mask:u64 = !0u64 >> (64 - K);

    // Start with 0111..11000....0
    let curr: u64 = ((1u64 << w) - 1) << (k - w - 1);

    Bracelets { curr: curr, mask: mask }
  }

}

impl Iterator for Bracelets {
  type Item = u64;

  fn next(&mut self) -> Option<u64> {

    // adapted from: http://graphics.stanford.edu/~seander/bithacks.html
    let t:u64 = self.curr | (self.curr - 1);
    self.curr = (t + 1) | (((!t & (t + 1)) - 1) >> (self.curr.trailing_zeros() + 1));

    if self.curr <= self.mask {
      Some(self.curr)
    } else {
      None
    }
  }
}

fn check_wt(a:[u64;K]) -> (bool, u64) {

	for i in 2..(D/2) {
    for j in Bracelets::new(K, i) {
  		if comb_weight(j, a) < D - i {
        return (false, j);
		  }
	  }
  }

	(true, 0)
}

fn comb_weight(d:u64, a:[u64;K]) -> usize {

	let mut w:usize = 0;

	for i in 0..K {
		if !even_parity(d & a[i]) {
			w += 1;
		}
	}


	w

}


fn main() {
  let d: u64 = 0b101111011;
  let a = matrix(d);

  for i in a.iter() {
    println!("{:012b}", i);
  }
  
  println!("ZD: {}", is_zd(matrix(d)));

  let result = check_wt(a);

  if result.0 {
    println!("{:x} has weight at least {}.", d, D);
  } else {
    println!("{:x} with combination {:x} has weight less than {}.", d, result.1, D);
  }

}

