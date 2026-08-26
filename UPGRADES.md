# Upgrades

## 0.11.0 — default-store configuration

This breaks WireRevision 3. Upgrade every consumer with this contract before
deploying it. `Configure` now carries only the ordinary and meta socket paths:
the owning Orchestrate Nexus derives its Sema store path locally. The former
store-path field and `StorePathImmutable` refusal are removed, with no
compatibility decoder or migration.
