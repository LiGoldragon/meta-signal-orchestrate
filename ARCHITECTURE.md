# meta-signal-orchestrate architecture

`ethos/signal.ethos` owns the privileged configuration wire contract.
Ethos-zero 0.5.0 emits the committed `src/generated/signal.rs`; the executable
regeneration tool and byte-identical rustfmt test reject stale output. That
file is therefore provenance-marked generated code, not a handwritten wire
interface.

`Channel.{MetaOrchestrate 2 5}` generates source-owned protocol and channel
constants, closed request/reply/refusal roots, typed Datomic anatomy, and the
rkyv frame record. `src/codec.rs` owns the separate length-prefix/rkyv
validation boundary. Consumers use re-exports from this crate; it contains no
meta Nexus policy.

`Configure` consists only of `ordinary_socket_path` and `meta_socket_path`.
The owning Nexus derives and owns its fixed Sema store location; it is never a
wire-configurable path. `Configured` belongs to Reply and
`ConfigurationRejected` belongs to Refusal; its closed refusal payload is
`InvalidConfiguration`.
