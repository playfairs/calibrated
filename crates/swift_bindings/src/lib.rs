pub mod nsappearance {
    // Listings of all C-ABI bnidings from the swiftc's
    // *.dylib to do with NSUI
    unsafe extern "C" {
        unsafe fn NSAppearanceEffectiveAppearance() -> i32;
    }

    /// Inferface style
    pub enum NSAppearance {
        NSAppearanceNameAqua,
        NSAppearanceNameDarkAqua,
    }

    impl From<i32> for NSAppearance {
        fn from(value: i32) -> Self {
            match value {
                0 => Self::NSAppearanceNameAqua,
                1 => Self::NSAppearanceNameDarkAqua,
                _ => unreachable!(),
            }
        }
    }

    /// Returns enum which represents the NSUIS
    pub fn get_interface_style() -> NSAppearance {
        let result = unsafe { NSAppearanceEffectiveAppearance() };
        NSAppearance::from(result)
    }
}
