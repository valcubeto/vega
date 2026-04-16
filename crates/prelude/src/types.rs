use std::rc::Rc;

pub type Str = Box<str>;
// TODO: see std::rc::Weak
pub type SharedStr = Rc<str>;

pub type Slice<T> = Box<[T]>;
pub type SliceView<'a, T> = &'a [T];
