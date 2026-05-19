//! OwnerSignal contract for privileged `persona-orchestrate`
//! administration.
//!
//! Ordinary claim/release/handoff/activity traffic lives in
//! `signal-persona-orchestrate`. This crate carries owner-only
//! orders that mutate the orchestration substrate itself.

use nota_codec::{NotaEnum, NotaRecord};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;
pub use signal_persona_orchestrate::{HarnessKind, RoleIdentifier, RoleName, WirePath};

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CreateRoleOrder {
    pub role: RoleIdentifier,
    pub harness: HarnessKind,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RetireRoleOrder {
    pub role: RoleIdentifier,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RefreshRepositoryIndexOrder {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RoleCreated {
    pub role: RoleIdentifier,
    pub harness: HarnessKind,
    pub report_repository_path: WirePath,
    pub report_lane_path: WirePath,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RoleRetired {
    pub role: RoleIdentifier,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RoleCreationRejected {
    pub role: RoleIdentifier,
    pub reason: RoleCreationRejectionReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum RoleCreationRejectionReason {
    RoleAlreadyExists,
    ReportRepositoryAlreadyExists,
    ReportLaneAlreadyExists,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, Copy, PartialEq, Eq,
)]
pub struct RepositoryIndexRefreshed {
    pub repositories: u32,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum OwnerOrchestrateOperationKind {
    CreateRoleOrder,
    RetireRoleOrder,
    RefreshRepositoryIndexOrder,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum OwnerOrchestrateUnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct OwnerOrchestrateRequestUnimplemented {
    pub operation: OwnerOrchestrateOperationKind,
    pub reason: OwnerOrchestrateUnimplementedReason,
}

signal_channel! {
    channel OwnerOrchestrate {
        request OwnerOrchestrateRequest {
            Mutate CreateRoleOrder(CreateRoleOrder),
            Retract RetireRoleOrder(RetireRoleOrder),
            Mutate RefreshRepositoryIndexOrder(RefreshRepositoryIndexOrder),
        }
        reply OwnerOrchestrateReply {
            RoleCreated(RoleCreated),
            RoleRetired(RoleRetired),
            RoleCreationRejected(RoleCreationRejected),
            RepositoryIndexRefreshed(RepositoryIndexRefreshed),
            OwnerOrchestrateRequestUnimplemented(OwnerOrchestrateRequestUnimplemented),
        }
    }
}

pub type Frame = OwnerOrchestrateFrame;
pub type FrameBody = OwnerOrchestrateFrameBody;
pub type ChannelRequest = OwnerOrchestrateChannelRequest;
pub type ChannelReply = OwnerOrchestrateChannelReply;
pub type RequestBuilder = OwnerOrchestrateRequestBuilder;

impl OwnerOrchestrateRequest {
    pub fn operation_kind(&self) -> OwnerOrchestrateOperationKind {
        match self {
            Self::CreateRoleOrder(_) => OwnerOrchestrateOperationKind::CreateRoleOrder,
            Self::RetireRoleOrder(_) => OwnerOrchestrateOperationKind::RetireRoleOrder,
            Self::RefreshRepositoryIndexOrder(_) => {
                OwnerOrchestrateOperationKind::RefreshRepositoryIndexOrder
            }
        }
    }
}

impl From<CreateRoleOrder> for OwnerOrchestrateRequest {
    fn from(payload: CreateRoleOrder) -> Self {
        Self::CreateRoleOrder(payload)
    }
}

impl From<RetireRoleOrder> for OwnerOrchestrateRequest {
    fn from(payload: RetireRoleOrder) -> Self {
        Self::RetireRoleOrder(payload)
    }
}

impl From<RefreshRepositoryIndexOrder> for OwnerOrchestrateRequest {
    fn from(payload: RefreshRepositoryIndexOrder) -> Self {
        Self::RefreshRepositoryIndexOrder(payload)
    }
}

impl From<RoleCreated> for OwnerOrchestrateReply {
    fn from(payload: RoleCreated) -> Self {
        Self::RoleCreated(payload)
    }
}

impl From<RoleRetired> for OwnerOrchestrateReply {
    fn from(payload: RoleRetired) -> Self {
        Self::RoleRetired(payload)
    }
}

impl From<RoleCreationRejected> for OwnerOrchestrateReply {
    fn from(payload: RoleCreationRejected) -> Self {
        Self::RoleCreationRejected(payload)
    }
}

impl From<RepositoryIndexRefreshed> for OwnerOrchestrateReply {
    fn from(payload: RepositoryIndexRefreshed) -> Self {
        Self::RepositoryIndexRefreshed(payload)
    }
}

impl From<OwnerOrchestrateRequestUnimplemented> for OwnerOrchestrateReply {
    fn from(payload: OwnerOrchestrateRequestUnimplemented) -> Self {
        Self::OwnerOrchestrateRequestUnimplemented(payload)
    }
}
