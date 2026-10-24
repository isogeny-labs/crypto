# PROTOCOL (DRAFT): equal-advice rho vs IC at n = 73 and n = 83, launch pointer

DRAFT. This file is a pointer. The question, rungs, advice budgets, controls and decision rule are frozen in research/equal_advice_wide_20261018/PROTOCOL.md; this card does not restate or change them.

To be frozen before `ready`:

1. Name the idle host (RAM at least 48 GiB, 32 threads, idle during online passes, isolation level recorded).
2. Pin the commit that contains research/equal_advice_wide_20261018/ and the two examples, and record it in card.json `code.commit`.
3. Record `git hash-object PROTOCOL.md` for this file in card.json `protocol.blob`.
4. Name the independent second-host replay (the protocol's `independent_replay_pointer`).
5. Measure the wide-step cost on the named host; the 0.4 to 1.0 us projection is a prediction until then.

Non-claims (from the frozen protocol): nothing about n = 73 or n = 83 outcomes; classed accounting and audit, not an improvement to the IC baseline.
