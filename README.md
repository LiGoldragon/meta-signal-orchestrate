# meta-signal-orchestrate

MetaSignal contract for privileged `orchestrate` administration: dynamic role
creation, repository-index refresh, and exact registry-maintenance vocabulary.

Ordinary claim/release/handoff/activity messages live in `signal-orchestrate`.

`ForceRemoveRegistryRow` has coherent immutable legacy family pins, but
`orchestrate` still needs a separate consumer integration. It selects a closed
exact durable-row identity and never authorizes filesystem removal.
