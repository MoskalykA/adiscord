use serde_repr::Deserialize_repr;

#[allow(non_camel_case_types)]
#[derive(Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum Flags {
    None = 0,
    PINNED = 1 << 1,
    REQUIRE_TAG = 1 << 4,
}
