//! TODO: write this documentation

#[cfg(feature = "paths")]
pub use scythe_paths as paths;

#[cfg(feature = "platform")]
pub use scythe_platform as platform;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
