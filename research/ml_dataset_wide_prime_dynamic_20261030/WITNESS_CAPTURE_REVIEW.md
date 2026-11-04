# Dedicated witness capture source review

Reviewed patch 2267405d7e82d3f4bd70506fc4304b9e9b68546dda4a8b4ab593c0c432c861e1 against its accepted contract. Source review only; patch remains unapplied and uncompiled.

The fixed identity, preadmitted buffers, bounded serializer, incremental hash tariff, shared counter bridge, sticky failures and publication ordering match the intended contract. Both API orders check identity and persisted content. NewCommit increments only after durable head acknowledgement; errors after head rename are explicitly indeterminate. Optional sequence on legacy adoption avoids invented provenance. Requested storage is declared helper-layout accounting, not measured stack or RSS.

No concrete blocker found in this pass. Application must wait for the active group validation so its result remains bound to unchanged source. Required next checks are actual Rust compilation and the authored wire/restore, idempotence, cap/deadline and failure controls. Caller integration still must escrow and settle cumulative budgets without resets, retain exact witness bytes, and implement mathematical replay before invoking publication. This review does not activate the profile or establish mathematical correctness of submitted bytes.
