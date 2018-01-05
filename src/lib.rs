#![no_std]

pub mod category;

pub trait CharExt {
    fn general_category(self) -> category::General;
}

impl CharExt for char {
    #[inline]
    fn general_category(self) -> category::General { category::General::en(include_bytes!("../data/category")[self as usize] as usize) }
}
