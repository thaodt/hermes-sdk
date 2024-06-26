use ibc::core::connection::types::version::Version;
use ibc_proto::google::protobuf::Any as IbcProtoAny;
use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenAck as ProtoMsgConnectionOpenAck;
use ibc_relayer_types::core::ics24_host::identifier::ConnectionId;
use ibc_relayer_types::signer::Signer;
use ibc_relayer_types::Height;
use prost_types::Any;

use crate::methods::encode::encode_to_any;
use crate::traits::message::DynCosmosMessage;

const TYPE_URL: &str = "/ibc.core.connection.v1.MsgConnectionOpenAck";

#[derive(Debug)]
pub struct CosmosConnectionOpenAckMessage {
    pub connection_id: ConnectionId,
    pub counterparty_connection_id: ConnectionId,
    pub version: Version,
    pub client_state: Any,
    pub update_height: Height,
    pub proof_try: Vec<u8>,
    pub proof_client: Vec<u8>,
    pub proof_consensus: Vec<u8>,
    pub proof_consensus_height: Height,
}

impl DynCosmosMessage for CosmosConnectionOpenAckMessage {
    fn counterparty_message_height_for_update_client(&self) -> Option<Height> {
        Some(self.update_height)
    }

    fn encode_protobuf(&self, signer: &Signer) -> IbcProtoAny {
        let proto_message = ProtoMsgConnectionOpenAck {
            connection_id: self.connection_id.as_str().to_string(),
            counterparty_connection_id: self.counterparty_connection_id.as_str().to_string(),
            client_state: Some(IbcProtoAny {
                type_url: self.client_state.type_url.clone(),
                value: self.client_state.value.clone(),
            }),
            proof_height: Some(self.update_height.into()),
            proof_try: self.proof_try.clone(),
            proof_client: self.proof_client.clone(),
            proof_consensus: self.proof_consensus.clone(),
            consensus_height: Some(self.proof_consensus_height.into()),
            version: Some(self.version.clone().into()),
            signer: signer.to_string(),
            host_consensus_state_proof: Vec::new(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
