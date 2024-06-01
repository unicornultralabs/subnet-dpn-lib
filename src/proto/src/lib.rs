pub mod internal_tx {
    include!(concat!(env!("OUT_DIR"), "/subnet_dpn.internal_tx.rs"));
}
pub mod tx {
    include!(concat!(env!("OUT_DIR"), "/subnet_dpn.tx.rs"));
}
pub mod session {
    include!(concat!(env!("OUT_DIR"), "/subnet_dpn.session.rs"));
}

pub mod proxy_acc {
    include!(concat!(env!("OUT_DIR"), "/subnet_dpn.proxy_acc.rs"));
}

pub mod notification {
    include!(concat!(env!("OUT_DIR"), "/subnet_dpn.noti.rs"));
}

pub mod stream_payload {
    include!(concat!(env!("OUT_DIR"), "/subnet_dpn.stream_payload.rs"));
}

pub mod user_balance {
    include!(concat!(env!("OUT_DIR"), "/subnet_dpn.user_balance.rs"));
}

#[cfg(test)]
mod tests {}
