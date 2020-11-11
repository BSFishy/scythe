//! TODO: write this documentation

use std::fmt;

/// TODO: write this documentation
#[derive(Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Platform {
    Windows,
    Mac,
    Linux,
    #[allow(non_camel_case_types)] iOS,
    Android,
    Unknown,
}

/// TODO: write this documentation
#[derive(Debug, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum PlatformVendor {
    Microsoft,
    Apple,
    Google,
    Unknown,
}

/// TODO: write this documentation
pub fn get_current_platform() -> Platform {
    if cfg!(target_os = "windows") {
        Platform::Windows
    } else if cfg!(target_os = "macos") {
        Platform::Mac
    } else if cfg!(target_os = "linux") {
        Platform::Linux
    } else if cfg!(target_os = "ios") {
        Platform::iOS
    } else if cfg!(target_os = "android") {
        Platform::Android
    } else {
        Platform::Unknown
    }
}

/// TODO: write this documentation
pub fn get_current_platform_vendor() -> PlatformVendor {
    get_current_platform().get_vendor()
}

impl Platform {
    /// TODO: write this documentation
    pub fn get_current() -> Platform {
        get_current_platform()
    }

    /// TODO: write this documentation
    pub fn get_vendor(&self) -> PlatformVendor {
        match self {
            Platform::Windows => PlatformVendor::Microsoft,
            Platform::Mac => PlatformVendor::Apple,
            Platform::iOS => PlatformVendor:: Apple,
            Platform::Android => PlatformVendor::Google,
            _ => PlatformVendor::Unknown,
        }
    }
}

impl Default for Platform {
    fn default() -> Self {
        get_current_platform()
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PlatformVendor {
    /// TODO: write this documentation
    pub fn get_current() -> PlatformVendor {
        get_current_platform_vendor()
    }
}

impl Default for PlatformVendor {
    fn default() -> Self {
        get_current_platform_vendor()
    }
}

impl fmt::Display for PlatformVendor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_current_platform_works() {
        let platform = get_current_platform();

        if cfg!(target_os = "windows") {
            assert_eq!(platform, Platform::Windows, "current platform on windows should be windows")
        } else if cfg!(target_os = "macos") {
            assert_eq!(platform, Platform::Mac, "current platform on macos should be macos")
        } else if cfg!(target_os = "linux") {
            assert_eq!(platform, Platform::Linux, "current platform on linux should be linux")
        } else if cfg!(target_os = "ios") {
            assert_eq!(platform, Platform::iOS, "current platform on ios should be ios")
        } else if cfg!(target_os = "android") {
            assert_eq!(platform, Platform::Android, "current platform on android should be android")
        } else {
            assert_eq!(platform, Platform::Unknown, "current platform on unknown platform should be unknown")
        }
    }
}
