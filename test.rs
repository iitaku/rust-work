extern crate hoge;
use hoge::hoge;
//mod hoge;

struct TimeBomb {
    s: &'static str
}

impl Drop for TimeBomb {
    fn drop(&mut self) {
        println!("dropped {}", self.s)
    }
}

trait Seq<T> {
    fn length(&self) -> uint;
}

impl<T> Seq<T> for Vec<T> {
    fn length(&self) -> uint {
        self.len()
    }
}

fn print_length<T>(xs: &Seq<T>)
{
    println!("{}", xs.length())
}

fn main () {
    print_length(&(*(box vec![TimeBomb{s:"0"}, TimeBomb{s:"1"}])) as &Seq<TimeBomb>);
    hoge();
    print_length(&(*(box vec![TimeBomb{s:"2"}])) as &Seq<TimeBomb>);
    hoge();
}
