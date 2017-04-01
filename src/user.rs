/// Descriptor for a (username, unique discrimnator) pair.
///
/// OWAPI uses the pair formatted as `"uname-UD"` for API access.
pub trait BNetUser: Sized {
    /// Convert `self` into `"uname-UD"`.
    fn identifier(self) -> String;
}

impl<'a> BNetUser for &'a str {
    fn identifier(self) -> String {
        self.to_string()
    }
}

impl BNetUser for String {
    fn identifier(self) -> String {
        self
    }
}

impl<'a> BNetUser for &'a String {
    fn identifier(self) -> String {
        self.clone()
    }
}

impl<'a> BNetUser for (&'a str, u32) {
    fn identifier(self) -> String {
        format!("{}-{}", self.0, self.1)
    }
}

impl BNetUser for (String, u32) {
    fn identifier(self) -> String {
        format!("{}-{}", self.0, self.1)
    }
}

impl<'a> BNetUser for (&'a String, u32) {
    fn identifier(self) -> String {
        format!("{}-{}", self.0, self.1)
    }
}
