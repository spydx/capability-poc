#[allow(proc_macro_derive_resolution_fallback)]
use serde::{self, de, Deserialize};

#[derive(Deserialize, Debug)]
#[serde(untagged)] // This is the magic. see https://serde.rs/enum-representations.html
pub enum VecOrOne<T> {
    Vec(Vec<T>),
    One(T),
}

pub fn deser_one_as_vec<'de, D: de::Deserializer<'de>, T: Deserialize<'de>>(
    de: D,
) -> Result<Vec<T>, D::Error> {
    use de::Deserialize as _;
    match VecOrOne::deserialize(de)? {
        VecOrOne::Vec(v) => Ok(v),
        VecOrOne::One(i) => Ok(vec![i]),
    }
}
