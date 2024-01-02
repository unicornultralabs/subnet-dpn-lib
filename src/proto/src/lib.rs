pub mod internal_tx {
    include!(concat!(env!("OUT_DIR"), "/subnet_dpn.internal_tx.rs"));
}
#[cfg(test)]
mod tests {}
