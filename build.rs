fn main() {
    prost_build::Config::new()
        .compile_protos(
            &[
                // tendermint
                "./proto/tendermint/blocksync/types.proto",
                "./proto/tendermint/consensus/types.proto",
                "./proto/tendermint/consensus/wal.proto",
                "./proto/tendermint/crypto/keys.proto",
                "./proto/tendermint/crypto/proof.proto",
                "./proto/tendermint/libs/bits/types.proto",
                "./proto/tendermint/mempool/types.proto",
                "./proto/tendermint/state/types.proto",
                "./proto/tendermint/statesync/types.proto",
                "./proto/tendermint/store/types.proto",
                "./proto/tendermint/types/block.proto",
                "./proto/tendermint/types/canonical.proto",
                "./proto/tendermint/types/events.proto",
                "./proto/tendermint/types/evidence.proto",
                "./proto/tendermint/types/params.proto",
                "./proto/tendermint/types/types.proto",
                "./proto/tendermint/types/validator.proto",
                "./proto/tendermint/version/types.proto",
            ],
            &["proto"],
        )
        .expect("Failed to compile proto files");
}
