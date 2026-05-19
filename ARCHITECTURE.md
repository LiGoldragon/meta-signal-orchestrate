# owner-signal-persona-orchestrate — architecture

*OwnerSignal contract for privileged `persona-orchestrate` role and
repository administration.*

## 0 · TL;DR

`owner-signal-persona-orchestrate` is the owner-only Signal surface for
mutating orchestration topology. Ordinary role claims, releases,
handoffs, observations, and activity records stay in
`signal-persona-orchestrate`.

This split is code-enforced now and filesystem-permission-enforced
later: callers can compile against the ordinary contract without being
able to express role creation or repository-index refresh orders.

## 1 · Contract Surface

| Request | Signal verb | Meaning |
|---|---|---|
| `CreateRoleOrder` | `Mutate` | Create a dynamic role lane with its harness metadata. |
| `RetireRoleOrder` | `Retract` | Retire a dynamic role from the active registry. |
| `RefreshRepositoryIndexOrder` | `Mutate` | Re-scan local checkouts and refresh the orchestration repository index. |

| Reply | Meaning |
|---|---|
| `RoleCreated` | The daemon created the role record and report-lane paths. |
| `RoleRetired` | The daemon retired the role record. |
| `RoleCreationRejected` | The create order was valid but conflicts with existing state. |
| `RepositoryIndexRefreshed` | The local repository index was refreshed. |
| `OwnerOrchestrateRequestUnimplemented` | The request is part of the owner vocabulary but not implemented by the current runtime. |

## 2 · Shared Nouns

This crate imports role and path nouns from
`signal-persona-orchestrate`:

- `RoleIdentifier`
- `RoleName` compatibility alias
- `HarnessKind`
- `WirePath`

It does not duplicate ordinary claim, release, handoff, activity, or
scope records.

## 3 · Constraints

| Constraint | Witness |
|---|---|
| Topology-changing orders live only in the owner contract. | Ordinary `signal-persona-orchestrate::OrchestrateRequest` has no `CreateRoleOrder`, `RetireRoleOrder`, or `RefreshRepositoryIndexOrder` variants; this crate round-trips all owner variants. |
| Every owner request declares a Signal root verb. | `OwnerOrchestrateRequest::signal_verb()` witnesses `Mutate`, `Retract`, and `Mutate`. |
| Contract code contains no runtime. | Source contains no Kameo, Tokio, sema-engine, redb, filesystem mutation, GitHub, or ghq implementation. |
| Harness assignment is typed, not hidden in a role string. | `CreateRoleOrder` carries `HarnessKind` beside `RoleIdentifier`. |

## 4 · Non-Ownership

- No `persona-orchestrate` daemon.
- No role registry table.
- No claim table.
- No report repository creation.
- No workspace symlink writing.
- No CLI argv parsing.
- No filesystem permission enforcement.

## Code Map

```text
src/lib.rs            owner request/reply records and signal_channel! invocation
tests/round_trip.rs   frame round trips and verb witnesses
```

## See Also

- `../signal-persona-orchestrate/ARCHITECTURE.md`
- `../persona-orchestrate/ARCHITECTURE.md`
- `~/primary/skills/contract-repo.md`
