# meta-signal-orchestrate

The generated MetaSignal wire contract for privileged Orchestrate
configuration. Its source of truth is `ethos/signal.ethos`; the `regenerate`
example invokes Ethos-zero WireContract emission and rustfmt to produce the
committed `src/generated/signal.rs` projection.

The meta channel has ContractId 2 and WireRevision 5. It carries closed
Request, Reply, and Refusal roots:

- `Request::Configure(Configure)`.
- `Reply::Configured(Configured)`.
- `Refusal::ConfigurationRejected(ConfigurationRejected)`.

The concrete textual input is:

```text
Configure.{/tmp/orchestrate.sock /tmp/meta-orchestrate.sock}
```

`Frame` is generated alongside protocol/channel constants. Hand-owned
`SignalFrameCodec` length-prefixes, rkyv-validates, and checks those constants.
This crate owns neither Nexus startup, persistence, socket rebinding, nor CLI
argument parsing.
