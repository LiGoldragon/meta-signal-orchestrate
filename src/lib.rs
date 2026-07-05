//! MetaSignal contract for privileged `orchestrate`
//! administration.
//!
//! Ordinary claim/release/handoff/activity traffic lives in
//! `signal-orchestrate`. This crate carries meta-signal
//! orders that mutate the orchestration substrate itself.

use nota::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
pub use signal_orchestrate::{
    ApplicationFailure, ApplicationFailureReason, ApplicationSuccess, BranchName,
    DownstreamComponent, DurationNanos, HarnessKind, LaneAssignment, LaneAuthority, LaneDetails,
    LaneIdentifier, LaneName, LaneOwner, LaneProjection, LaneRegistration, LaneResourceClaim,
    LaneStatus, PartialApplied, PurposeText, PushedState, RepositoryName, Role, RoleIdentifier,
    RoleName, RoleToken, ScopeReason, ScopeReference, SessionIdentifier, SessionName,
    TimestampNanos, WirePath, Worktree, WorktreeStatus,
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

/// Register (upsert) a single worktree the agent created. Carries the
/// full [`Worktree`] record; the daemon may re-derive `last_activity`
/// and `pushed_state` from the filesystem. Reply: [`WorktreeRegistered`].
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RegisterWorktree {
    pub worktree: Worktree,
}

/// Re-scan `~/wt/github.com/LiGoldragon/<repo>/<name>` and refresh the
/// whole worktree index, mirroring [`RefreshRepositoryIndexOrder`].
/// Reply: [`WorktreeIndexRefreshed`].
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RefreshWorktreeIndexOrder {}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct LaneRegistrationRequest {
    pub assignment: LaneAssignment,
    pub mode: LaneRegistrationMode,
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
pub enum LaneRegistrationMode {
    Fresh,
    Recovery,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct LaneUnregistrationRequest {
    pub session: SessionIdentifier,
    pub lane: LaneIdentifier,
    pub details: LaneDetails,
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
)]
pub struct RepositoryIndexRefreshed(u32);

impl RepositoryIndexRefreshed {
    pub fn new(repositories: u32) -> Self {
        Self(repositories)
    }

    pub fn repositories(&self) -> u32 {
        self.0
    }
}

/// Transition a single registered worktree's status to [`WorktreeStatus::Archived`].
/// The daemon looks up the worktree by `path`, updates its status, and
/// reprojects `worktrees.nota`. Reply: [`WorktreeArchived`].
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ArchiveWorktreeOrder {
    pub path: WirePath,
}

/// Ack for [`ArchiveWorktreeOrder`] — echoes the worktree after the status
/// transition to `Archived`.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct WorktreeArchived {
    pub worktree: Worktree,
}

/// Ack for [`RegisterWorktree`] — echoes the registered worktree.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct WorktreeRegistered {
    pub worktree: Worktree,
}

/// Ack for [`RefreshWorktreeIndexOrder`] — count of worktrees the scan
/// found, mirroring [`RepositoryIndexRefreshed`].
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
)]
pub struct WorktreeIndexRefreshed(u32);

impl WorktreeIndexRefreshed {
    pub fn new(worktrees: u32) -> Self {
        Self(worktrees)
    }

    pub fn worktrees(&self) -> u32 {
        self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct LaneRegistered {
    pub registration: LaneRegistration,
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
pub enum LaneAlreadyRegisteredResolution {
    FreshConflict,
    RecoveryInherited,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct LaneAlreadyRegistered {
    pub requested: LaneRegistrationRequest,
    pub active: LaneProjection,
    pub resolution: LaneAlreadyRegisteredResolution,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct LaneUnregistered {
    pub session: SessionIdentifier,
    pub lane: LaneIdentifier,
    pub ended_at: TimestampNanos,
    pub details: LaneDetails,
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
        operation Unregister(LaneUnregistrationRequest),
        operation SetAuthority(LaneAuthorityChange),
        operation RegisterWorktree(RegisterWorktree),
        operation RefreshWorktreeIndex(RefreshWorktreeIndexOrder),
        operation ArchiveWorktree(ArchiveWorktreeOrder),
    }
    reply MetaOrchestrateReply {
        RoleCreated(RoleCreated),
        RoleRetired(RoleRetired),
        RoleCreationRejected(RoleCreationRejected),
        RepositoryIndexRefreshed(RepositoryIndexRefreshed),
        LaneRegistered(LaneRegistered),
        LaneAlreadyRegistered(LaneAlreadyRegistered),
        LaneUnregistered(LaneUnregistered),
        LaneRetired(LaneRetired),
        LaneAuthoritySet(LaneAuthoritySet),
        WorktreeRegistered(WorktreeRegistered),
        WorktreeIndexRefreshed(WorktreeIndexRefreshed),
        WorktreeArchived(WorktreeArchived),
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

impl From<LaneUnregistrationRequest> for MetaOrchestrateRequest {
    fn from(payload: LaneUnregistrationRequest) -> Self {
        Self::Unregister(payload)
    }
}

impl From<LaneAuthorityChange> for MetaOrchestrateRequest {
    fn from(payload: LaneAuthorityChange) -> Self {
        Self::SetAuthority(payload)
    }
}

impl From<RegisterWorktree> for MetaOrchestrateRequest {
    fn from(payload: RegisterWorktree) -> Self {
        Self::RegisterWorktree(payload)
    }
}

impl From<RefreshWorktreeIndexOrder> for MetaOrchestrateRequest {
    fn from(payload: RefreshWorktreeIndexOrder) -> Self {
        Self::RefreshWorktreeIndex(payload)
    }
}

impl From<ArchiveWorktreeOrder> for MetaOrchestrateRequest {
    fn from(payload: ArchiveWorktreeOrder) -> Self {
        Self::ArchiveWorktree(payload)
    }
}
