// I'd call this delightful dance an anti-pattern, but it bafflingly seems to be
// the accepted way to deal with the artifically long, semantically meaningless,
// inelegant stutterings that rust insists on baking into module paths...
pub mod char_width;
pub mod newline;

pub use char_width::CharWidth;
pub use char_width::CharWidthBuilder;
pub use newline::Newline;
pub use newline::NewlineBuilder;

pub trait Index {
   fn split(&self, r: std::ops::Range<usize>) -> (Option<Self>, Option<Self>) where Self: Sized;
}

pub trait IndexBuilder<T> where T: Index {
   fn new() -> Self;
   fn push(&mut self, c: char);
   fn freeze(self) -> T;
}


pub struct Compound {
   pub newline: Newline,
   pub char_width: CharWidth,
}

impl Index for Compound {
   fn split(&self, r: std::ops::Range<usize>) -> (Option<Self>, Option<Self>) where Self: Sized {
      let (nl1, nl2) = self.newline.split(r.clone());
      let (cw1, cw2) = self.char_width.split(r);
      (
         Some(Compound{ newline: nl1.unwrap(), char_width: cw1.unwrap() }),
         Some(Compound{ newline: nl2.unwrap(), char_width: cw2.unwrap() }),
      )
   }
}

pub struct CompoundBuilder {
   newline: NewlineBuilder,
   char_width: CharWidthBuilder,
}

impl IndexBuilder<Compound> for CompoundBuilder {
   fn new() -> Self {
      Self {
         newline: NewlineBuilder::new(),
         char_width: CharWidthBuilder::new(),
      }
   }

   fn push(&mut self, c: char) {
      self.newline.push(c);
      self.char_width.push(c);
   }

   fn freeze(self) -> Compound {
      Compound {
         newline: self.newline.freeze(),
         char_width: self.char_width.freeze(),
      }
   }
}
