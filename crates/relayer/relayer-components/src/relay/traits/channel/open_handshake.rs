use cgp_core::prelude::*;

use crate::relay::traits::chains::HasRelayChains;
use crate::relay::types::aliases::{DstChannelId, DstPortId, SrcChannelId, SrcPortId};

#[derive_component(ChannelOpenHandshakeRelayerComponent, ChannelOpenHandshakeRelayer<Relay>)]
#[async_trait]
pub trait CanRelayChannelOpenHandshake: HasRelayChains {
    async fn relay_channel_open_handshake(
        &self,
        src_channel_id: &SrcChannelId<Self>,
        src_port_id: &SrcPortId<Self>,
        dst_port_id: &DstPortId<Self>,
    ) -> Result<DstChannelId<Self>, Self::Error>;
}
