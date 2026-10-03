# meta-signal-orchestrate architecture

`ethos/signal.ethos` owns the privileged configuration wire contract.
Ethos-zero 13.0.0 emits the committed `src/generated/signal.rs`, and
`build.rs` refuses to build when the committed projection differs from what
the generator produces. That file is generated code, not a handwritten wire
interface.

The wire is signal 7.0.0's exchange layer. `Query` implements
`signal::Contracted` with `ETHOS` as its source, so the contract's identity is
the FNV-1a digest of `ethos/signal.ethos`: a connection is greeted once
(`Dispatch::Greet`), a peer built from another source is refused with
`ContractMismatch`, and each query opens an exchange (`Dispatch::Open`) that is
answered once (`Delivery::Answer`) and then ends (`Delivery::End`). The
framing, the envelope and the ledger are signal's; this crate contains no
meta Nexus policy.

`Configure` carries an `OrchestrateNexusConfiguration` of only the ordinary
and meta socket paths. The owning Nexus derives and owns its fixed store
location; it is never a wire-configurable path. Rejections are vocabulary:
`ConfigurationRejected` with the closed reason `InvalidConfiguration`, and
`PeerRefused` naming the refused peer's user id.
