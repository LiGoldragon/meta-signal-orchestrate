# meta-signal-orchestrate

The generated MetaSignal wire contract for privileged Orchestrate
configuration. Its source of truth is `ethos/signal.ethos`; ethos-zero
13.0.0 generates the committed `src/generated/signal.rs` projection, which
`build.rs` holds byte-identical.

The contract rides signal 7.0.0's exchange layer. Its identity is the
digest of `ethos/signal.ethos`, settled once per connection by the greeting
(`Query` implements `signal::Contracted`); each query then opens one exchange,
is answered once and ends. It carries the closed `Query` and `Response`
roots:

- `Query::Configure(OrchestrateNexusConfiguration)` and
  `Query::ReverseMetaConfiguration`.
- `Response::Configured`, `Response::OrdinaryConfigurationReopened`,
  `Response::ConfigurationRejected` and `Response::PeerRefused`.

This crate owns neither Nexus startup, persistence, socket rebinding, nor CLI
argument parsing.
