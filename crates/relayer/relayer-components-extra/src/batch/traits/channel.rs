use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain::HasChainTypes;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::relay::traits::target::ChainTarget;
use hermes_runtime_components::traits::channel::HasChannelTypes;
use hermes_runtime_components::traits::channel_once::HasChannelOnceTypes;
use hermes_runtime_components::traits::runtime::{HasRuntime, RuntimeOf};

use crate::batch::types::aliases::MessageBatchSender;

#[derive_component(MessageBatchSenderGetterComponent, MessageBatchSenderGetter<Relay>)]
pub trait HasMessageBatchSender<Target>: HasRelayChains
where
    Target: ChainTarget<Self>,
    Target::TargetChain: HasRuntime,
    RuntimeOf<Target::TargetChain>: HasChannelTypes + HasChannelOnceTypes,
{
    fn get_batch_sender(&self) -> &MessageBatchSender<Target::TargetChain, Self::Error>;
}

pub trait HasMessageBatchSenderType<Error>: Async {
    type MessageBatchSender: Async;
}

impl<Chain, Error> HasMessageBatchSenderType<Error> for Chain
where
    Error: Async,
    Chain: HasChainTypes + HasRuntime,
    Chain::Runtime: HasChannelTypes + HasChannelOnceTypes,
{
    type MessageBatchSender = MessageBatchSender<Chain, Error>;
}

pub trait HasMessageBatchSenderTypes: Async {
    type SrcMessageBatchSender: Async;

    type DstMessageBatchSender: Async;
}

impl<Relay, SrcMessageBatchSender, DstMessageBatchSender> HasMessageBatchSenderTypes for Relay
where
    SrcMessageBatchSender: Async,
    DstMessageBatchSender: Async,
    Relay: HasRelayChains,
    Relay::SrcChain:
        HasMessageBatchSenderType<Relay::Error, MessageBatchSender = SrcMessageBatchSender>,
    Relay::DstChain:
        HasMessageBatchSenderType<Relay::Error, MessageBatchSender = DstMessageBatchSender>,
{
    type SrcMessageBatchSender = SrcMessageBatchSender;

    type DstMessageBatchSender = DstMessageBatchSender;
}
