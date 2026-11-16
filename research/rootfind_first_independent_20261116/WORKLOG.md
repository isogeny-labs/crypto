# Work log

## 2026-11-16, protocol freeze

### What was done

- Inspected the clean stacked branch at `c2468c8f5d`, the block-tail
  result and production collector; fetched `origin/main` at
  `a91b8b66d0f1ffc175fdce984d1059144ce232f6`.
- Queried shared KB revision `9141e386254340a9bdba3036f2bb706a` and
  read the supplied rootfinding PDF and the repository's Koblitz relation
  note. Sources checked (KB): `source:883ab40565c1f261353efc03ba04473e`
  at `crypto:research/notes/ecc2k130/RESEARCH_KOBLITZ_INDEX_CALCULUS.md`
  led to the rank-aware stopping rule; the recent block-tail report is newer
  than KB refresh.
- Conductor T-6795 reserved this study directory and test path. Conductor
  T-6708 still reserves the production collector paths; T-6720 reserves
  today's shared session log.

### Decisions made

- Test first-independent scanning with fixed disjoint K19/K23 streams and
  full K13 enumeration. Freeze the operation proxy and answer checks before
  implementation and data.
- AWS STS returned account `590183823895`, not the required IsoLab account
  `104186811093`. The existing monthly budget receipt already exceeded its
  limit. Do not submit the frozen EKS panel from this identity.

### Open threads

- Implement the standalone harness, run once from committed source, derive
  the result and visual record, publish in the existing stacked PR.
- Reconcile today's shared session log when T-6720 releases it.

### Errors

- The current AWS identity is the wrong account for IsoLab admission.
