# meta-signal-orchestrate

The generated MetaSignal wire contract for privileged Orchestrate
configuration. Its source of truth is `ethos/signal.ethos`; `build.rs` uses
`ethos-monolith` to regenerate the committed `src/generated/signal.rs`
projection.

The meta channel has ContractId 2 and WireRevision 4. It carries the closed
surface:

- `MetaOrchestrateRequest::Configure(Configure)`.
- `MetaOrchestrateReply::{Configured, ConfigurationRejected}`.

The concrete textual input is:

```text
Configure.{/tmp/orchestrate.sock /tmp/meta-orchestrate.sock}
```

`Frame` is generated alongside the source-owned `MetaOrchestrateWire` binding
and is the binary transport contract. This crate owns neither daemon startup,
persistence, socket rebinding, nor CLI argument parsing.
