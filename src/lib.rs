pub mod tendermint {
    pub mod blocksync {
        include!(concat!(env!("OUT_DIR"), "/tendermint.blocksync.rs"));
    }

    pub mod consensus {
        include!(concat!(env!("OUT_DIR"), "/tendermint.consensus.rs"));
    }

    pub mod crypto {
        include!(concat!(env!("OUT_DIR"), "/tendermint.crypto.rs"));
    }

    pub mod libs {
        pub mod bits {
            include!(concat!(env!("OUT_DIR"), "/tendermint.libs.bits.rs"));
        }
    }

    pub mod mempool {
        include!(concat!(env!("OUT_DIR"), "/tendermint.mempool.rs"));
    }

    pub mod state {
        include!(concat!(env!("OUT_DIR"), "/tendermint.state.rs"));
    }

    pub mod statesync {
        include!(concat!(env!("OUT_DIR"), "/tendermint.statesync.rs"));
    }

    pub mod store {
        include!(concat!(env!("OUT_DIR"), "/tendermint.store.rs"));
    }

    pub mod types {
        include!(concat!(env!("OUT_DIR"), "/tendermint.types.rs"));
    }

    pub mod version {
        include!(concat!(env!("OUT_DIR"), "/tendermint.version.rs"));
    }
}

pub mod google {
    pub use prost_types::Timestamp;
}
