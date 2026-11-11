# j=0 order-3 automorphism factor base (2026-11-11) — all `[pcc:unverified]`

Example: `examples/prime_fast_j0_sym.rs`. Curves y²=x³+b, p≡1 mod 3, prime order n (26 and 30 bits, two curves each),
single target, 1 thread, same host. Orbit factor base: B representatives, 6B covered points (3B x-values), B unknowns,
relation R = ±λ^k F_i ± λ^m F_j (2 summands). Every solve verified kG=Q.

| comparison | result |
|---|---|
| sym (B) vs plain `ic_solve` 2-summand at 3B x-values (48 solves) | sym 1.9x–2.6x faster; all verified (`run_26.jsonl`, `run_30.jsonl`) |
| sym (2-summand) vs `ic_solve` **3-summand**, c=0.5 (the strongest implemented baseline), 26 bits | 3-summand 25–31 ms vs sym 420–624 ms: **sym ~15–20x slower** (`run3_26.jsonl`; 30-bit run stopped) |
| sym vs single-target rho, negation only | rho 3–5 ms: sym 120x (26b) – 1400x+ (30b) behind |

Verdict: the symmetry gives a real constant (≈2x) inside the 2-summand family (fewer unknowns, ~3x fewer relations needed
at equal per-target cost) but 2-summand IC is far off the 3-summand baseline, so **no improvement over the strongest baseline**.
Not an improvement; the sym gain is bounded by the 3x unknown reduction already priced in.
Successor (not built): symmetric 3-summand (orbit-reduced pair table, B² entries over orbit elements) — needs lookup of
x(F_j±λ^m F_k) modulo the x-orbit; expected gain ≤ ~2x on a baseline that is already 100x+ behind rho, and rho gains √3 from
the same symmetry. Matched rho with automorphism (`aut_folded_rho.rs`) not run. Unfrozen exploratory run, no holdout protocol.
