use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert::{ConvertFrom, TryConvertFrom};
use hermes_protobuf_encoding_components::impls::any::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};
use hermes_protobuf_encoding_components::impls::from_context::EncodeFromContext;
use hermes_protobuf_encoding_components::types::Protobuf;
use ibc::core::commitment_types::merkle::MerkleProof;
use ibc_proto::ibc::core::commitment::v1::MerkleProof as ProtoMerkleProof;
use prost_types::Any;

use crate::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};

pub struct CosmosConverterComponents;

delegate_components! {
    CosmosConverterComponents {
        (TendermintClientState, ProtoTendermintClientState): ConvertFrom,
        (ProtoTendermintClientState, TendermintClientState): TryConvertFrom,

        (TendermintConsensusState, ProtoTendermintConsensusState): ConvertFrom,
        (ProtoTendermintConsensusState, TendermintConsensusState): TryConvertFrom,

        (MerkleProof, ProtoMerkleProof): ConvertFrom,
        (ProtoMerkleProof, MerkleProof): TryConvertFrom,

        (TendermintClientState, Any): EncodeAsAnyProtobuf<Protobuf, EncodeFromContext>,
        (Any, TendermintClientState): DecodeAsAnyProtobuf<Protobuf, EncodeFromContext>,

        (TendermintConsensusState, Any): EncodeAsAnyProtobuf<Protobuf, EncodeFromContext>,
        (Any, TendermintConsensusState): DecodeAsAnyProtobuf<Protobuf, EncodeFromContext>,
    }
}
