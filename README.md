# meta-signal-orchestrate

The generated MetaSignal wire contract for privileged Orchestrate
configuration. Its source of truth is `ethos/signal.ethos`; ethos-zero
27.0.0 generates the committed `src/generated/signal.rs` projection, which
`build.rs` holds byte-identical.

The contract rides signal 7.0.0's exchange layer. Its identity is the
digest of `ethos/signal.ethos`, settled once per connection by the greeting
(`Input` implements `signal::Contracted`); each input then opens one exchange,
is answered once and ends. It carries the closed `Input` and `Output`
roots, paired by position, and the `Answering` capabilities with
`impl Answerable for Input` that a Nexus answers through:

- `Input::Configure(OrchestrateNexusConfiguration)` and
  `Input::ReverseMetaConfiguration`.
- `Output::Configured`, `Output::OrdinaryConfigurationReopened`,
  `Output::ConfigurationRejected` and `Output::PeerRefused`.

This crate owns neither Nexus startup, persistence, socket rebinding, nor CLI
argument parsing.
