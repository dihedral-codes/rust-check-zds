use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::u64;

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

fn even_parity(x: u64) -> bool {
  let parity = x.count_ones() & 1;

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
}

impl Bracelets {
  fn new(w: usize) -> Bracelets  {
    // Start with 0111..11000....0
    let curr: u64 = (!0u64 >> (64 - w)) << (K - w - 1);
    Bracelets {curr: curr}
  }

}

impl Iterator for Bracelets {
  type Item = u64;

  fn next(&mut self) -> Option<u64> {

    // adapted from: http://graphics.stanford.edu/~seander/bithacks.html
    let t:u64 = self.curr | (self.curr - 1);
    self.curr = (t + 1) | (((!t & (t + 1)) - 1) >> (self.curr.trailing_zeros() + 1));

    if self.curr <= MASK {
      Some(self.curr)
    } else {
      None
    }
  }
}

fn check_wt(a:[u64;K]) -> (bool, u64) {

	for i in 2..(D/2) {
    for j in Bracelets::new(i) {
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
  // let d: u64 = 0b101111011;
  // let mut a = matrix(d);

	let mut ds:[u64; 524288] = [0; 524288];

  let f = File::open("fergal.txt").unwrap();
  let reader = BufReader::new(f);
  let mut i = 0;
  for line in reader.lines() {
    let line = line.unwrap().to_string();
    let x:u64 = u64::from_str_radix(&line, 2).unwrap();
    ds[i] = x;
    i += 1;
  }
  
  for d in ds.iter() {
    let d = *d;
    let a = matrix(d);
    
    if !is_zd(a) {
      println!("{} is not a zero divisor.", d);
    }

    let result = check_wt(a);

    if result.0 {
      println!("{:064b} has minD >= {}.", d, D);
    } else {
      println!("{:x} with combination {:x} has weight less than {}.", d, result.1, D);
    }
  }

}

