pub use self::gen_client::Client as DidClient;
use codec::Codec;
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use parami_did_utils::derive_storage_key;
use parking_lot::RwLock;
use sp_core::offchain::OffchainStorage;
use std::sync::Arc;

#[rpc]
pub trait DidApi<DecentralizedId> {
    /// Get metadata of a DID
    ///
    /// # Arguments
    ///
    /// * `did` - The DID
    /// * `key` - The metadata key
    ///
    /// # Results
    ///
    /// the requested metadata
    #[rpc(name = "did_getMetadata")]
    fn get_metadata(&self, did: DecentralizedId, key: String) -> Result<String>;

    /// Batch get metadata of a DID
    ///
    /// # Arguments
    ///
    /// * `did` - The DID
    /// * `keys` - The metadata keys
    ///
    /// # Results
    ///
    /// the requested metadata
    #[rpc(name = "did_batchGetMetadata")]
    fn batch_get_metadata(&self, did: DecentralizedId, keys: Vec<String>) -> Result<Vec<String>>;
}

pub struct DidRpcHandler<T: OffchainStorage, DecentralizedId> {
    storage: Arc<RwLock<T>>,
    _marker: std::marker::PhantomData<DecentralizedId>,
}

impl<T, DecentralizedId> DidRpcHandler<T, DecentralizedId>
where
    T: OffchainStorage,
    DecentralizedId: Codec,
{
    pub fn new(storage: T) -> Self {
        Self {
            storage: Arc::new(RwLock::new(storage)),
            _marker: Default::default(),
        }
    }
}

impl<T, DecentralizedId> DidApi<DecentralizedId> for DidRpcHandler<T, DecentralizedId>
where
    T: OffchainStorage + 'static,
    DecentralizedId: Codec + Send + Sync + 'static,
{
    fn get_metadata(&self, did: DecentralizedId, key: String) -> Result<String> {
        let metadata = self
            .storage
            .read()
            .get(
                sp_offchain::STORAGE_PREFIX,
                &*derive_storage_key(key.as_bytes(), &did),
            )
            .map(from_utf8)
            .unwrap_or_default();

        Ok(metadata)
    }

    fn batch_get_metadata(&self, did: DecentralizedId, keys: Vec<String>) -> Result<Vec<String>> {
        let mut result = Vec::new();

        for key in keys {
            let metadata = self
                .storage
                .read()
                .get(
                    sp_offchain::STORAGE_PREFIX,
                    &*derive_storage_key(key.as_bytes(), &did),
                )
                .map(from_utf8)
                .unwrap_or_default();

            result.push(metadata);
        }

        Ok(result)
    }
}

fn from_utf8<S: AsRef<[u8]>>(s: S) -> String {
    String::from_utf8_lossy(s.as_ref()).into_owned()
}
