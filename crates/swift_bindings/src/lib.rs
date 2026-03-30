unsafe extern "C" {
    unsafe fn NSUIInterfaceStyleCollect() -> i32;
}

pub mod nsui {
    use crate::NSUIInterfaceStyleCollect;
    /// Inferface style
    pub enum NSUIInterfaceStyle {
        Light,
        Dark,
    }

    impl From<i32> for NSUIInterfaceStyle {
        fn from(value: i32) -> Self {
            match value {
                0 => Self::Light,
                1 => Self::Dark,
                _ => unreachable!(),
            }
        }
    }

    /// Returns enum which represents the NSUIS
    pub fn get_interface_style() -> NSUIInterfaceStyle {
        let result = unsafe { NSUIInterfaceStyleCollect() };
        NSUIInterfaceStyle::from(result)
    }
}
