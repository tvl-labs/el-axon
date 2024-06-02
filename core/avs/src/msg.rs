use std::sync::Arc;

use futures_util::TryFutureExt;
use rlp_derive::{RlpDecodable, RlpEncodable};

use common_crypto::{BlsPrivateKey, HashValue, PrivateKey, Signature};
use core_storage::StorageError;
use protocol::traits::{Context, MessageHandler, Priority, Rpc, Storage, TrustFeedback};
use protocol::types::{BlockNumber, Bytes};
use protocol::{async_trait, ProtocolError};

pub const RPC_RESP_AVS_SIG: &str = "/rpc_call/avs/avs_resp_sig";

#[derive(RlpEncodable, RlpDecodable, Clone, Debug)]
pub struct AvsSig {
    pub block_number: u64,
    pub sig:          Bytes,
    pub address:      Bytes,
}

pub struct AvsMessageHandler<R, S> {
    rpc:         Arc<R>,
    storage:     Arc<S>,
    private_key: BlsPrivateKey,
    address:     Bytes,
}

impl<R: Rpc + 'static, S: Storage + 'static> AvsMessageHandler<R, S> {
    pub fn new(rpc: Arc<R>, storage: Arc<S>, private_key: BlsPrivateKey, address: Bytes) -> Self {
        AvsMessageHandler {
            rpc,
            storage,
            private_key,
            address,
        }
    }
}

#[async_trait]
impl<R: Rpc + 'static, S: Storage + 'static> MessageHandler for AvsMessageHandler<R, S> {
    type Message = BlockNumber;

    async fn process(&self, ctx: Context, msg: BlockNumber) -> TrustFeedback {
        let sig = match self.storage.get_block_header(ctx.clone(), msg).await {
            Ok(Some(block)) => Ok(block),
            Ok(None) => Err(StorageError::GetNone(msg.to_string()).into()),
            Err(e) => Err(e),
        }
        .map(|h| AvsSig {
            block_number: h.number,
            sig:          self
                .private_key
                .sign_message(&HashValue::from_bytes_unchecked(h.state_root.0))
                .to_bytes(),
            address:      self.address.clone(),
        });

        self.rpc
            .response(ctx, RPC_RESP_AVS_SIG, sig, Priority::High)
            .unwrap_or_else(move |e: ProtocolError| log::warn!("[core_avs] push signature {:?}", e))
            .await;

        TrustFeedback::Neutral
    }
}
