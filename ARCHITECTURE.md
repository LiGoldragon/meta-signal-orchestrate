# meta-signal-orchestrate — architecture

`meta-signal-orchestrate` declares the residual privileged Signal contract for
refreshing Orchestrate's declared repository index. It is a wire-type crate:
no daemon, durable store, text parser, filesystem mutation, or CLI parser.

## Boundary

The ordinary `signal-orchestrate` contract owns atomic path-lock registration.
Its `PathLock` carrier is constructed only from the native Datom `PathLock`
record, then sent in an ordinary Signal frame. This meta contract has no
PathLock operation, reply, re-export, or compatibility type.

The meta contract retains its own second contract seat at wire revision 2. The
crate version is 0.9.0. It exposes exactly one operation:

| Request | Reply | Meaning |
| --- | --- | --- |
| `Refresh(RefreshRepositoryIndexOrder)` | `RepositoryIndexRefreshed` | Refresh the declared repository index and return the observed count. |

## Interface source

`schema/authority.ethos` is the source for the one local closed enumeration:
`MetaOperationKind.[Refresh]`. The build accepts it only through the authorized
bootstrap transaction, Core Nomos revalidation, Whole Logos lowering, and Rust
Logos projection. `META_SIGNAL_ORCHESTRATE_UPDATE_INTERFACE_ARTIFACTS=1` is the
explicit refresh authority for the checked Rust projection.

## Constraints

- The sole privileged request is repository-index refresh.
- Ordinary path-lock registration remains separate, binary Signal carrying
  native Datom data.
- The contract has no DOTOS representation.
- The crate declares only vocabulary and binary frame codecs; it contains no
  execution logic.

## Code map

```text
schema/authority.ethos               authority Interface for MetaOperationKind
src/bootstrap_manifest.rs            opaque declaration seats and canonical order
src/schema/authority/generated.rs    checked Rust Logos projection
src/lib.rs                            binary request/reply vocabulary and channel
tests/bootstrap_boundary.rs          Interface and wire-binding witnesses
tests/round_trip.rs                  literal binary frame round trips
```

## See also

- `../signal-orchestrate/ARCHITECTURE.md` — ordinary native-Datom path locks.
- `../orchestrate/ARCHITECTURE.md` — runtime consumer.
