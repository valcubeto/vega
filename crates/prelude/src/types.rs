use std::rc::Rc;

pub type Str = Box<str>;
// TODO: see std::rc::Weak
pub type SharedStr = Rc<str>;

pub type Slice<T> = Box<[T]>;
pub type SliceView<'a, T> = &'a [T];

pub fn normalize_str_slice<'a>(slice: SliceView<'a, Str>) -> Vec<&'a str> {
    slice.iter()
        .map(|string| string.as_ref())
        .collect()
}
