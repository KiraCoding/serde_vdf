use serde::Deserialize;

/// Deserialize an instance of type `T` from a string of VDF text.
pub fn from_str<'a, T>(s: &'a str) -> Result<T, ()>
where
    T: Deserialize<'a>,
{
    todo!()
}

