#![feature(no_core, lang_items)]
#![no_core]

#[lang = "sized"]
pub trait Sized {}

#[lang = "unsize"]
pub trait Unsize<T: ?Sized> {}

#[lang = "coerce_unsized"]
pub trait CoerceUnsized<T> {}

#[lang = "copy"]
pub unsafe trait Copy {}

unsafe impl Copy for bool {}
unsafe impl Copy for u8 {}
unsafe impl Copy for u16 {}
unsafe impl Copy for u32 {}
unsafe impl Copy for u64 {}
unsafe impl Copy for usize {}
unsafe impl Copy for i8 {}
unsafe impl Copy for i16 {}
unsafe impl Copy for i32 {}
unsafe impl Copy for isize {}
unsafe impl Copy for f32 {}
unsafe impl Copy for char {}
unsafe impl<'a, T: ?Sized> Copy for &'a T {}
unsafe impl<T: ?Sized> Copy for *const T {}
unsafe impl<T: ?Sized> Copy for *mut T {}

#[lang = "bitor"]
pub trait BitOr<RHS = Self> {
    type Output;

    fn bitor(self, rhs: RHS) -> Self::Output;
}

impl BitOr for u32 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        self | rhs
    }
}

struct Jasper {
    data: u32,
}

pub fn jasper() {
    let _ktest = Jasper { data: 666666 };
}

pub fn add_numbers(x: u32, y: u32) -> u32 {
    x | y
}