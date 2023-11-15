pub trait Parse {
    fn from_str(input: &str) -> Self;

    fn from_string(input: String) -> Self
    where
        Self: Sized,
    {
        Self::from_str(input.as_str())
    }
}
