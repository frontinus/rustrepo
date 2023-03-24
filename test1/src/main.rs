fn main() {
    let i = 4;
  let mut b = Box::new( (5, 2) );

  (*b).1 = 7;

  println!("{:?}", *b); // (5,7)
  println!("{:?}", b);  // (5,7)
}
