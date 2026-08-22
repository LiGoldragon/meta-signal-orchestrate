//! MetaSignal contract for the residual privileged `orchestrate` refresh.
//!
//! Atomic path-lock registration belongs solely to the ordinary
//! `signal-orchestrate` channel and its native Datom carrier. This meta
//! contract deliberately contains no PathLock operation or compatibility type.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;

pub mod bootstrap_manifest;
pub mod schema;

/// Canonical authority Interface text verified by the build transaction.
pub const AUTHORITY_INTERFACE_SOURCE: &str = include_str!("../schema/authority.ethos");

/// Checked Rust projection of the authority Interface's remaining closed type.
pub const AUTHORITY_INTERFACE_RUST: &str = include_str!("schema/authority/generated.rs");

/// The meta Orchestrate contract occupies the second wire seat in its family.
pub enum MetaOrchestrateWire {}

impl signal_frame::WireContract for MetaOrchestrateWire {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        signal_frame::ContractId::new(
            core::num::NonZeroU32::new(2).expect("the meta wire seat is nonzero"),
        ),
        signal_frame::WireRevision::new(
            core::num::NonZeroU16::new(2).expect("the meta wire revision is nonzero"),
        ),
    );
}

/// Request the daemon to refresh its declared repository index.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct RefreshRepositoryIndexOrder {}

/// The number of repositories observed while refreshing the declared index.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepositoryIndexRefreshed(u32);

impl RepositoryIndexRefreshed {
    pub fn new(repositories: u32) -> Self {
        Self(repositories)
    }

    pub fn repositories(&self) -> u32 {
        self.0
    }
}

signal_channel! {
    channel MetaOrchestrate contract MetaOrchestrateWire {
        operation Refresh(RefreshRepositoryIndexOrder),
    }
    reply MetaOrchestrateReply {
        RepositoryIndexRefreshed(RepositoryIndexRefreshed),
    }
}

pub type MetaOrchestrateRequest = Operation;
pub type MetaOperationKind = OperationKind;
pub type ChannelRequest = signal_frame::Request<Operation>;
pub type ChannelReply = signal_frame::Reply<MetaOrchestrateReply>;

impl Operation {
    pub fn operation_kind(&self) -> MetaOperationKind {
        self.kind()
    }
}

impl From<RefreshRepositoryIndexOrder> for MetaOrchestrateRequest {
    fn from(payload: RefreshRepositoryIndexOrder) -> Self {
        Self::Refresh(payload)
    }
}
