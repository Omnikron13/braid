// I'd call this delightful dance an anti-pattern, but it bafflingly seems to be
// the accepted way to deal with the artifically long, semantically meaningless,
// inelegant stutterings that rust insists on baking into module paths...
pub mod char_width;
pub use char_width::CharWidth;
pub use char_width::CharWidthBuilder;

pub trait Index {
   fn split(&self, r: std::ops::Range<usize>) -> (Option<Self>, Option<Self>) where Self: Sized;
}

trait IndexBuilder<T> where T: Index {
   fn new() -> Self;
   fn push(&mut self, c: char);
   fn freeze(self) -> T;
}
