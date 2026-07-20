# meta-signal-orchestrate

MetaSignal contract for privileged `orchestrate` administration: dynamic role
creation, repository-index refresh, and exact registry-maintenance vocabulary.

Ordinary claim/release/handoff/activity messages live in `signal-orchestrate`.

`ForceRemoveRegistryRow` selects one of twelve closed durable-row identities
and never authorizes filesystem or Jujutsu removal. It uses the immutable
current-Criome-compatible ordinary contract family; `orchestrate` still needs a
separate consumer integration.
