use crate::base::one_for_all::impls::chain::queries::consensus_state::SendConsensusStateQueryToOfa;
use crate::base::one_for_all::impls::chain::queries::status::SendChainStatusQueryToOfa;
use crate::base::one_for_all::traits::chain::OfaIbcChain;
use crate::base::one_for_all::traits::chain::{OfaChainPreset, OfaIbcChainPreset};
use crate::base::one_for_all::traits::relay::OfaRelayPreset;
use crate::base::relay::impls::message_senders::chain_sender::SendIbcMessagesToChain;
use crate::base::relay::impls::message_senders::update_client::SendIbcMessagesWithUpdateClient;
use crate::common::one_for_all::presets::FullPreset;
use crate::full::batch::message_sender::SendMessagetoBatchWorker;
use crate::full::one_for_all::traits::chain::OfaFullChain;
use crate::full::one_for_all::traits::components::batch::OfaBatchPreset;
use crate::full::one_for_all::traits::relay::OfaFullRelay;
use crate::full::telemetry::impls::consensus_state::ConsensusStateTelemetryQuerier;
use crate::full::telemetry::impls::status::ChainStatusTelemetryQuerier;

impl<Chain> OfaChainPreset<Chain> for FullPreset
where
    Chain: OfaFullChain,
{
    type ChainStatusQuerier = ChainStatusTelemetryQuerier<SendChainStatusQueryToOfa>;
}

impl<Chain, Counterparty> OfaIbcChainPreset<Chain, Counterparty> for FullPreset
where
    Chain: OfaFullChain,
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaIbcChain<Chain>,
{
    type ConsensusStateQuerier = ConsensusStateTelemetryQuerier<SendConsensusStateQueryToOfa>;
}

impl<Relay> OfaRelayPreset<Relay> for FullPreset
where
    Relay: OfaFullRelay<Preset = FullPreset>,
    Relay::SrcChain: OfaFullChain,
    Relay::DstChain: OfaFullChain,
{
    type IbcMessageSender = SendMessagetoBatchWorker;
}

impl<Relay> OfaBatchPreset<Relay> for FullPreset
where
    Relay: OfaFullRelay<Preset = FullPreset>,
    Relay::SrcChain: OfaFullChain,
    Relay::DstChain: OfaFullChain,
{
    type IbcMessageSenderForBatchWorker = SendIbcMessagesWithUpdateClient<SendIbcMessagesToChain>;
}
