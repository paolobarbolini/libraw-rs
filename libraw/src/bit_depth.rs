pub trait BitDepth: private::Sealed {}

impl BitDepth for u8 {}
impl BitDepth for u16 {}

mod private {
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
}
