//! MetaSignal contract for privileged `orchestrate`
//! administration.
//!
//! Ordinary claim/release/handoff/activity traffic lives in
//! `signal-orchestrate`. This crate carries meta-signal
//! orders that mutate the orchestration substrate itself.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode, NotaEnum, NotaRecord};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
pub use signal_orchestrate::{
    ApplicationFailure, ApplicationFailureReason, ApplicationSuccess, DownstreamComponent,
    HarnessKind, LaneAuthority, LaneIdentifier, LaneRegistration, PartialApplied, Role,
    RoleIdentifier, RoleName, RoleToken, ScopeReason, WirePath,
};

pub mod schema;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CreateRoleOrder {
    pub role: RoleIdentifier,
    pub harness: HarnessKind,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RetireRoleOrder {
    pub role: RoleIdentifier,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum Retirement {
    Role(RetireRoleOrder),
    Lane(LaneIdentifier),
}

impl NotaEncode for Retirement {
    fn encode(&self, encoder: &mut Encoder) -> nota_codec::Result<()> {
        match self {
            Self::Role(order) => {
                encoder.start_record("Role")?;
                order.encode(encoder)?;
                encoder.end_record()
            }
            Self::Lane(lane) => {
                encoder.start_record("Lane")?;
                lane.encode(encoder)?;
                encoder.end_record()
            }
        }
    }
}

impl NotaDecode for Retirement {
    fn decode(decoder: &mut Decoder<'_>) -> nota_codec::Result<Self> {
        let head = decoder.peek_record_head()?;
        match head.as_str() {
            "Role" => {
                decoder.expect_record_head("Role")?;
                let order = RetireRoleOrder::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::Role(order))
            }
            "Lane" => {
                decoder.expect_record_head("Lane")?;
                let lane = LaneIdentifier::decode(decoder)?;
                decoder.expect_record_end()?;
                Ok(Self::Lane(lane))
            }
            other => Err(nota_codec::Error::UnknownVariant {
                enum_name: "Retirement",
                got: other.to_string(),
            }),
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RefreshRepositoryIndexOrder {}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LaneRegistrationRequest {
    pub role: Role,
    pub authority: LaneAuthority,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LaneAuthorityChange {
    pub lane: LaneIdentifier,
    pub authority: LaneAuthority,
}

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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LaneRegistered {
    pub registration: LaneRegistration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LaneRetired {
    pub lane: LaneIdentifier,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LaneAuthoritySet {
    pub lane: LaneIdentifier,
    pub authority: LaneAuthority,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum MetaOrchestrateUnimplementedReason {
    NotBuiltYet,
    DependencyNotReady,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
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
