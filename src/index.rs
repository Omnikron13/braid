// I'd call this delightful dance an anti-pattern, but it bafflingly seems to be
// the accepted way to deal with the artifically long, semantically meaningless,
// inelegant stutterings that rust insists on baking into module paths...
pub mod char_width;
pub use char_width::CharWidth;
