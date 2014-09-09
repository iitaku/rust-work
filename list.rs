extern crate debug;

enum List<T> {
  Cons(T, Box<List<T>>),
  Nil
}

impl<T: PartialEq> PartialEq for List<T> {
  fn eq(&self, ys: &List<T>) -> bool {
    match (self, ys) {
      (&Nil, &Nil) => true,
      (&Cons(ref x, box ref xs_tail), &Cons(ref y, box ref ys_tail))
        if x == y => xs_tail == ys_tail,
        _ => false
    }
  }
}

fn main () {
  let mut xs = Cons(1i, box Cons(2i, box Nil));
  let ys = xs;

  xs = Nil;

  println!("{:?}", xs == ys);
  println!("{:?}", ys);
}
