# meta-signal-orchestrate architecture

`ethos/signal.ethos` owns the privileged configuration wire contract.
`ethos/nexus.ethos` and `ethos/sema.ethos` are the required empty component
modules. A Cargo build invokes `ethos-monolith::ComponentGeneration` over that
directory and rejects stale committed output; `src/generated/signal.rs` is
therefore provenance-marked generated code, not a handwritten wire interface.

`Channel.{MetaOrchestrate 2 3}` generates the source-owned
`MetaOrchestrateWire` binding, request operation, closed reply enum, Dotos
codecs, and `signal-frame` declaration. Consumers use re-exports from this
crate; it contains no meta daemon policy.

`Configure` consists only of `store_path`, `ordinary_socket_path`, and
`meta_socket_path`. Its closed refusal records either `StorePathImmutable` or
`InvalidConfiguration`.
