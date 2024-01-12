pub mod internal_tx {
    include!(concat!(env!("OUT_DIR"), "/subnet_dpn.internal_tx.rs"));
}
pub mod tx {
    include!(concat!(env!("OUT_DIR"), "/subnet_dpn.tx.rs"));
}
pub mod session {
    include!(concat!(env!("OUT_DIR"), "/subnet_dpn.session.rs"));
}

#[cfg(test)]
mod tests {}
