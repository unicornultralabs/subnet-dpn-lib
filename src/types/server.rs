use num_derive::FromPrimitive;

#[derive(Debug, Clone, FromPrimitive)]
pub enum ServerStatus {
    Inactive,
    Active,
    Full,
}

