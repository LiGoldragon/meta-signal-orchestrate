//! MetaSignal contract for privileged `orchestrate`
//! administration.
//!
//! Ordinary claim/release/handoff/activity traffic lives in
//! `signal-orchestrate`. This crate carries meta-signal
//! orders that mutate the orchestration substrate itself.

use nota_next::{Block, Delimiter, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
pub use signal_orchestrate::{
    ApplicationFailure, ApplicationFailureReason, ApplicationSuccess, DownstreamComponent,
    HarnessKind, LaneAuthority, LaneIdentifier, LaneRegistration, PartialApplied, Role,
    RoleIdentifier, RoleName, RoleToken, ScopeReason, WirePath,
};

pub mod schema;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CreateRoleOrder {
    pub role: RoleIdentifier,
    pub harness: HarnessKind,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RetireRoleOrder {
    pub role: RoleIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum Retirement {
    Role(RetireRoleOrder),
    Lane(LaneIdentifier),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RefreshRepositoryIndexOrder {}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct LaneRegistrationRequest {
    pub role: Role,
    pub authority: LaneAuthority,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct LaneAuthorityChange {
    pub lane: LaneIdentifier,
    pub authority: LaneAuthority,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RoleCreated {
    pub role: RoleIdentifier,
    pub harness: HarnessKind,
    pub report_repository_path: WirePath,
    pub report_lane_path: WirePath,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RoleRetired {
    pub role: RoleIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RoleCreationRejected {
    pub role: RoleIdentifier,
    pub reason: RoleCreationRejectionReason,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum RoleCreationRejectionReason {
    RoleAlreadyExists,
    ReportRepositoryAlreadyExists,
    ReportLaneAlreadyExists,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepositoryIndexRefreshed {
    pub repositories: u32,
}

impl NotaDecode for RepositoryIndexRefreshed {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let children = NotaBlock::new(block).expect_children(
            Delimiter::Parenthesis,
            "RepositoryIndexRefreshed",
            1,
        )?;
        let repositories = u32::try_from(u64::from_nota_block(&children[0])?)
            .map_err(|error| NotaDecodeError::Parse(error.to_string()))?;
        Ok(Self { repositories })
    }
}

impl NotaEncode for RepositoryIndexRefreshed {
    fn to_nota(&self) -> String {
        Delimiter::Parenthesis.wrap([u64::from(self.repositories).to_nota()])
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct LaneRegistered {
    pub registration: LaneRegistration,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct LaneRetired {
    pub lane: LaneIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct LaneAuthoritySet {
    pub lane: LaneIdentifier,
    pub authority: LaneAuthority,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum MetaOrchestrateUnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
}

#[cfg_attr(feature = "nota-text", derive(NotaEncode, NotaDecode))]
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct MetaOrchestrateRequestUnimplemented {
    pub operation: MetaOperationKind,
    pub reason: MetaOrchestrateUnimplementedReason,
}

signal_channel! {
    channel MetaOrchestrate {
        operation Create(CreateRoleOrder),
        operation Retire(Retirement),
        operation Refresh(RefreshRepositoryIndexOrder),
        operation Register(LaneRegistrationRequest),
        operation SetAuthority(LaneAuthorityChange),
    }
    reply MetaOrchestrateReply {
        RoleCreated(RoleCreated),
        RoleRetired(RoleRetired),
        RoleCreationRejected(RoleCreationRejected),
        RepositoryIndexRefreshed(RepositoryIndexRefreshed),
        LaneRegistered(LaneRegistered),
        LaneRetired(LaneRetired),
        LaneAuthoritySet(LaneAuthoritySet),
        PartialApplied(PartialApplied),
        MetaOrchestrateRequestUnimplemented(MetaOrchestrateRequestUnimplemented),
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

impl From<CreateRoleOrder> for MetaOrchestrateRequest {
    fn from(payload: CreateRoleOrder) -> Self {
        Self::Create(payload)
    }
}

impl From<RetireRoleOrder> for MetaOrchestrateRequest {
    fn from(payload: RetireRoleOrder) -> Self {
        Self::Retire(Retirement::Role(payload))
    }
}

impl From<LaneIdentifier> for MetaOrchestrateRequest {
    fn from(payload: LaneIdentifier) -> Self {
        Self::Retire(Retirement::Lane(payload))
    }
}

impl From<RefreshRepositoryIndexOrder> for MetaOrchestrateRequest {
    fn from(payload: RefreshRepositoryIndexOrder) -> Self {
        Self::Refresh(payload)
    }
}

impl From<LaneRegistrationRequest> for MetaOrchestrateRequest {
    fn from(payload: LaneRegistrationRequest) -> Self {
        Self::Register(payload)
    }
}

impl From<LaneAuthorityChange> for MetaOrchestrateRequest {
    fn from(payload: LaneAuthorityChange) -> Self {
        Self::SetAuthority(payload)
    }
}
