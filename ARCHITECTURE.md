# meta-signal-orchestrate — architecture

*MetaSignal contract for privileged `orchestrate` role, session/lane lifecycle,
and repository administration.*

## Direction

`meta-signal-orchestrate` is the meta authority contract for mutating orchestration topology. It exists to make the authority split code-enforced now and filesystem-permission-enforced later: callers can compile against the ordinary `signal-orchestrate` contract without being able to express role creation or repository-index refresh orders. Partial-failure semantics are commit-first-success-and-record-divergence; the `PartialApplied` reply names that outcome on the wire.

This crate carries only typed wire vocabulary, NOTA codecs, and round-trip witnesses — no Kameo, Tokio, sema-engine, redb, filesystem mutation, GitHub, or ghq logic.

## Ethos Interface Stage

`schema/authority.ethos` is the authority source for the five local closed
enumerations. It deliberately has no roles: this crate does not invent a
second description of the ordinary orchestration vocabulary imported from
`signal-orchestrate`.

The build accepts that source only through an authorized bootstrap transaction,
Core Nomos revalidation, Whole Logos lowering, and Rust Logos projection. The
checked Rust projection is an auditable witness of the Interface; public wire
behavior and NOTA codecs remain handwritten Rust while the Protos execution
surface grows into them. `META_SIGNAL_ORCHESTRATE_UPDATE_INTERFACE_ARTIFACTS=1`
is the sole explicit refresh authority. Cargo metadata publishes the owned
Ethos directory for structural consumers.

There is one Rust textual naming boundary: Rust Logos. Opaque authority seats
and canonical-order bytes live in `src/bootstrap_manifest.rs`; spelling and
source position never mint identity.

## Contract-local verbs

The public meta request surface is now:

- `Create(CreateRoleOrder)`
- `Retire(Retirement)`
- `Refresh(RefreshRepositoryIndexOrder)`
- `Register(LaneRegistrationRequest)`
- `Unregister(LaneUnregistrationRequest)`
- `SetAuthority(LaneAuthorityChange)`
- `ForceRemoveRegistryRow(ForceRemoveRegistryRowOrder)`

There is no public `Mutate` / `Retract` tag in this contract. The
meta socket remains the authority boundary; `orchestrate`
owns the typed Component Commands (Layer 2) that lower contract
operations to executable form, and projects them to payloadless Sema
class labels (Layer 3) for observation. See
`~/primary/skills/component-triad.md` §"Verbs come in three layers".

## Contract Surface (Layer 1)

| Operation | Projected Sema class | Meaning |
|---|---|---|
| `Create` | `Mutate` | Create a dynamic role lane with its harness metadata. |
| `Retire` | `Retract` | Retire a dynamic role from the active registry. |
| `Refresh` | `Mutate` | Re-scan local checkouts and refresh the orchestration repository index. |
| `Register` | `Mutate` | Atomically claim an orchestrator-assigned session/lane lifecycle registration. |
| `Unregister` | `Retract` | End an active session/lane lifecycle registration. |
| `SetAuthority` | `Mutate` | Change registered lane authority metadata. |
| `ForceRemoveRegistryRow` | `Retract` | Owner-authorized exact-row maintenance removal; it affects durable orchestrate state and derived projections only, never a checkout or other live filesystem resource. |

| Reply | Meaning |
|---|---|
| `RoleCreated` | The daemon created the role record and report-lane paths. |
| `RoleRetired` | The daemon retired the role record. |
| `RoleCreationRejected` | The create order was valid but conflicts with existing state. |
| `RepositoryIndexRefreshed` | The local repository index was refreshed. |
| `LaneRegistered` | The daemon registered the assigned session/lane; ownership is established. |
| `LaneAlreadyRegistered` | The atomic registration check found an active lane and carries active projection, age/time/details, plus `FreshConflict` or `RecoveryInherited` resolution. |
| `LaneUnregistered` | The daemon ended the active session/lane lifecycle registration. |
| `RegistryRowRemoved` | The exact requested row was retracted; the reply carries the typed identity and daemon-minted removal time. |
| `RegistryRowNotFound` | The exact requested row was already absent; nothing changed. |
| `PartialApplied` | One or more downstream mutation legs succeeded while one or more sibling legs failed; orchestrate records the divergence instead of rolling back. |
| `MetaOrchestrateRequestUnimplemented` | The request is part of the meta vocabulary but not implemented by the current runtime. |

## Shared Nouns

This crate imports role and path nouns from
`signal-orchestrate`:

- `RoleIdentifier`
- `HarnessKind`
- `SessionIdentifier`, `LaneAssignment`, `LaneRegistration`, `LaneProjection`,
  `LaneDetails`, and related lane owner/status/resource-claim projection nouns
- `PartialApplied` and its downstream success/failure records
- `RegistryRowIdentity`, a closed exact-key vocabulary for owner-authorized maintenance removal
- `WirePath`, `RepositoryName`, `BranchName`, `ScopeReference`, `WorkflowRunHandle`,
  `OrchestratorAgentIdentifier`, and `OrchestratorTopicPath`

It does not duplicate ordinary claim, release, handoff, activity, or
scope records.

## Constraints

| Constraint | Witness |
|---|---|
| Topology-changing orders live only in the meta-signal contract. | Ordinary `signal-orchestrate::OrchestrateRequest` has no `CreateRoleOrder`, `RetireRoleOrder`, or `RefreshRepositoryIndexOrder` variants; this crate round-trips all meta variants. |
| Every meta request has a contract-local operation root. | `MetaOrchestrateRequest::operation_kind()` witnesses `Create`, `Retire`, and `Refresh`. |
| Contract code contains no runtime. | Source contains no Kameo, Tokio, sema-engine, redb, filesystem mutation, GitHub, or ghq implementation. |
| Harness assignment is typed, not hidden in a role string. | `CreateRoleOrder` carries `HarnessKind` beside `RoleIdentifier`. |
| Lane lifecycle is meta-only and atomic. | `LaneRegistrationRequest` carries explicit `SessionIdentifier`, assigned `LaneIdentifier`, owner, details, and `Fresh`/`Recovery` mode; ordinary `signal-orchestrate` has no lifecycle mutation root. |
| Maintenance removal is exact and non-destructive. | `ForceRemoveRegistryRowOrder` carries a closed `RegistryRowIdentity` variant for every durable mutable row family (claim, role, lane, repository, worktree, activity, divergence, workflow resolution, agent, topic, topic membership, and triage audit); it excludes counters and engine metadata, accepts no lane-only or free-text selector, and never authorizes filesystem or Jujutsu effects. |

## Non-Ownership

- No `orchestrate` daemon.
- No role registry table.
- No claim table.
- No report repository creation.
- No workspace symlink writing.
- No CLI argv parsing.
- No filesystem permission enforcement.

## Code Map

```text
schema/authority.ethos               authority Interface for local closed types
src/bootstrap_manifest.rs            opaque declaration seats and canonical order
src/schema/authority/generated.rs    checked Rust Logos projection
src/lib.rs                            wire records, codecs, and signal channel
tests/bootstrap_boundary.rs          strict boundary and exact-pin witnesses
tests/round_trip.rs                   native frame and NOTA round trips
```

## See Also

- `../signal-orchestrate/ARCHITECTURE.md`
- `../orchestrate/ARCHITECTURE.md`
- `../signal-frame/ARCHITECTURE.md`
- `../signal-sema/ARCHITECTURE.md`
- `~/primary/skills/contract-repo.md`
- `~/primary/skills/component-triad.md` §"Verbs come in three layers".
