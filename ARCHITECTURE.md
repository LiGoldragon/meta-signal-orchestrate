# meta-signal-orchestrate architecture

`ethos/signal.ethos` owns the privileged configuration wire contract. A Cargo
build invokes `ethos-monolith::SignalGeneration` for that source and rejects
stale committed `src/generated/signal.rs` output. That file is therefore
provenance-marked generated code, not a handwritten wire interface.

`Channel.{MetaOrchestrate 2 4}` generates the source-owned
`MetaOrchestrateWire` binding, request operation, closed reply enum, Dotos
codecs, and `signal-frame` declaration. Consumers use re-exports from this
crate; it contains no meta daemon policy.

`Configure` consists only of `ordinary_socket_path` and `meta_socket_path`.
The owning Nexus derives and owns its fixed Sema store location; it is never a
wire-configurable path. The closed refusal is `InvalidConfiguration`.
