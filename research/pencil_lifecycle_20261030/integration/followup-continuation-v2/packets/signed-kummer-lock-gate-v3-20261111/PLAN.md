# Two-workspace lock gate v3

Resolves driver and standalone adapter locks sequentially under one shared 45-second resolver deadline within the original 60-second reservation. The rustc proxy only permits the pinned version query.

Source-only scope. No compiler, test, resolver or scientific payload was invoked. All older packets remain unchanged. Exact executable differences are in v3 successor.patch.

Combined: 1800 build + 600 execution seconds, 2 CPU / 16 GiB, one attempt, no automatic retry. Lock gate: one attempt, 60 seconds total, 45 shared resolver seconds, 1 CPU / 2 GiB, private network and pinned version-only rustc proxy.

Pending: independent source review; actual generated driver and adapter locks; published input and lock readback; absolute UTC deadline; live owner and ledger; fresh cache and current release receipts; current campaign authorization receipt for 129600 seconds and 2026-11-11T12:00:00Z.

KB tools unavailable; pinned local primary source and independent F1/F2/F3 review used. No novelty claim.


V3 corrects F2-R1: parse the actual Rust Record envelope, require integer sequence zero, then inspect event.kind and event.data. Identity rejection controls now construct this same envelope. V2 remains preserved as reviewed evidence; source review is pending.
