# INTENT — owner-signal-orchestrate

*The owner-only wire contract for privileged `orchestrate` role and repository
administration. Defines the typed request/reply channel that the orchestration
owner uses to create and retire dynamic role lanes and refresh the repository
index.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this owner-only
`owner-signal-orchestrate` contract. Workspace-shape intent stays in the primary
workspace `primary/INTENT.md`. Component daemon intent stays in
`orchestrate/INTENT.md`. Ordinary role claims, releases, handoffs, observations,
and activity records stay in `signal-orchestrate/INTENT.md`.

## Why this repo exists

`owner-signal-orchestrate` is the **owner-only authority contract** for mutating
orchestration topology. The split is code-enforced now and
filesystem-permission-enforced later: callers can compile against the ordinary
`signal-orchestrate` contract without being able to express role creation or
repository-index refresh orders.

## The channel shape

The owner channel carries:

- **Requests:** `Create(CreateRoleOrder)` (create a dynamic role lane with its
  harness metadata), `Retire(RetireRoleOrder)` (retire a dynamic role from the
  active registry), `Refresh(RefreshRepositoryIndexOrder)` (re-scan local
  checkouts and refresh the orchestration repository index).
- **Replies:** `RoleCreated`, `RoleRetired`, `RoleCreationRejected` (valid order
  conflicting with existing state), `RepositoryIndexRefreshed`, `PartialApplied`
  (one or more downstream legs succeeded while a sibling failed; orchestrate
  records the divergence instead of rolling back), and
  `OwnerOrchestrateRequestUnimplemented` (in the owner vocabulary, not yet
  implemented).

Shared role and path nouns are imported from `signal-orchestrate`
(`RoleIdentifier`, `RoleName` alias, `HarnessKind`, the `PartialApplied` family,
`WirePath`). This crate does not duplicate ordinary claim, release, handoff,
activity, or scope records.

## Constraints

- Topology-changing orders live only in the owner contract — ordinary
  `signal-orchestrate` has no `CreateRoleOrder`, `RetireRoleOrder`, or
  `RefreshRepositoryIndexOrder` variant.
- Every owner request has a contract-local operation root in verb form. There is
  no public `Mutate` / `Retract` tag; the Sema class is a daemon-side projection.
- Harness assignment is typed, not hidden in a role string — `CreateRoleOrder`
  carries `HarnessKind` beside `RoleIdentifier`.
- Partial-failure semantics are commit-first-success-and-record-divergence; the
  `PartialApplied` reply names that outcome on the wire.
- This crate carries only typed wire vocabulary, NOTA codecs, and round-trip
  witnesses — no Kameo, Tokio, sema-engine, redb, filesystem mutation, GitHub,
  or ghq logic.

## Non-ownership

This crate does not own:

- the `orchestrate` daemon;
- the role registry table or claim table;
- report repository creation;
- lowering from contract operations to executable Component Commands or Sema
  effects.

## See also

- `ARCHITECTURE.md` — contract surface, shared nouns, and constraint witnesses.
- `../orchestrate/INTENT.md` — daemon-side intent (role registry, claims, supervision).
- `../signal-orchestrate/INTENT.md` — ordinary claim/release/handoff/activity contract.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and authority tiers.
