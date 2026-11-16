## 2026-11-05 — Reflection-odd coefficient solve (T-1631)

### What was done (commands, files, features)

Derived and implemented the exact centered odd-sector reduction. The r479/q16 solve has239 rows,344 orbit columns and82455 augmented entries, producing479 original-variable terms. All99 optimized overflow-checked native tests pass, including36 full/odd span controls and eight source integrations. Independent polynomial/field replay accepts the packet and rejects a reflected-partner sign corruption. The q15 odd control rejects with observed rank238. Exact-run counter:7227137 inner elimination updates. Twenty source/data hashes verify.

### Decisions made (and why)

Use reflection invariance and odd target after centering; prove equivalence before dropping the even sector. Keep both epsilon signs and fixed odd orbits. Preserve the previous478-term packet; new output has one additional term and16 source queries. Dimension savings are algebraic; no comparative timing or benchmark verdict is inferred from diagnostic counters.

### Open threads (unfinished, pick up next time)

Investigate recurrence-aware elimination inside the odd sector and complete source/setup/verification costs. Original4563-case cohort unchanged. Current-head CI and review remain required.

### Errors (what went wrong, how it was handled)

No native test failed. Eight compiler warnings retained. Partner-sign corruption is an intentional negative checker failure. A representation-theory PDF fetch timed out; the projection proof is supplied directly and NIST's character sign law was read. No cloud job ran.
