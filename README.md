# meta-signal-orchestrate

MetaSignal contract for privileged `orchestrate` administration: dynamic role
creation, repository-index refresh, and exact registry-maintenance vocabulary.

Ordinary claim/release/handoff/activity messages live in `signal-orchestrate`.

`ForceRemoveRegistryRow` is producer-only until a coherent legacy contract-family
pin or the Protos bridge lets `orchestrate` consume it portably. It selects a
closed exact durable-row identity and never authorizes filesystem removal.
