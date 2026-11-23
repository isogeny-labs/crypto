# 2026-11-17 — exact-source RR9 norm coefficient certificate: passing Rust CI and independent arithmetic readback

**Scope:** original cofactor-one prime31 curve `E:y²=x³+847514846x+2087444724 mod 2147483647`, prime `n=2147446831`, first31 and actual depth-six Dickson31 bases. This validates a *GIVEN-leaf algebraic certificate*, not a target-conditioned extractor or an unknown-scalar ECDLP solve. All data are development-producer `[pcc:unverified]` until separately signed.

## Source-specific execution

- [GitHub Actions run #43995251039](https://github.com/isogeny-labs/crypto/actions/runs/43995251039), [job 133745586281](https://github.com/isogeny-labs/crypto/actions/runs/43995251039/job/133745586281), **SUCCESS** 2026-11-17 01:25:43 UTC.
- Exact-source commit: `04e07a0d97fe70f54cc7cd7f4f45bc5a0ed0bc63` from open PR #4466. Rust 1.100.0, GitHub Actions ubuntu-24.04, native pinned full source `1df714645aa683cf975bc76fcacb9842cb4780fd`. Source RR9 example copied unchanged into pinned native Rust tree.
- Existing reusable native library tests **40/40 PASS**. RR9 example tests **3/3 PASS**. Standalone release example completed: first31 32 distinct planted true/32 wrong-target false and Dickson31 32/32 (128 original determinant decisions); all prior duplicate, Hermite multiplicity and ninth-jet controls retained and passed. Each base needed exactly 32 draws to obtain 32 admissible cases.
- [Full 59,122-byte raw JSON artifact](https://github.com/isogeny-labs/crypto/actions/runs/43995251039/artifacts/14289436365), artifact ID `14289436365`, zipped SHA256 `3712a5c446374e038ec87372f3fb40eee8f8cf70188ce4de3c17c4b75a734433`; workflow upload completed normally.

## New original-curve x-norm checks

The Rust example now computes the unique projective `F=U(x)+yV(x)` in `L(9O)` vanishing at eight **supplied** signed leaves, through an 8x9 RREF with elliptic Hermite rows on diagonals. Its field-polynomial norm `N=U²-(x³+ax+b)V²` is a degree-nine polynomial whenever the eight-leaf sum `R` is finite. Every new certificate checks `N=-v₃²(x-x(R))∏(x-x(P_i))` *as ten field coefficients* and separately checks `F(-R)=0` and `F(-(R+G)) != 0`. Checking only `N(x(R))=0` would not certify orientation.

| Witness class | Raw original-curve certificates | Checks completed |
| --- | ---: | --- |
| Eight distinct signed leaves, first31 | 32 | 32/32 |
| Eight distinct signed leaves, Dickson31 | 32 | 32/32 |
| Multiplicity 3, 4 and 8; first31 | 3 | 3/3 |
| Multiplicity 3, 4 and 8; Dickson31 | 3 | 3/3 |
| **Total** | **70** | **70/70** |

### Independent arithmetic readback of raw artifact (producer-side, separately written calculation)

The raw ZIP was downloaded from GitHub Actions and extracted without modifying records. A separate simple Python arithmetic *verifier*, **not** an ECDLP solver or replacement for the Rust implementation, independently reconstructed the original Weierstrass group addition and signed negation, checked all supplied leaves and residuals on `E(F_p)`, recomputed each eight-leaf group sum and the wrong residual `R+G`, evaluated `F` at `-R` and `-(R+G)`, recomputed all 10 coefficients of `N` from the recorded nine kernel coefficients, and multiplied each nine-factor x-polynomial directly. For repeats, an independently written truncated square-root Taylor recursion checked all eight length-divisor vanishing equations at their required Hermite orders, including multiplicity eight.

**All 64 distinct and all six repeated records passed every independent numeric check**; no mismatch in 70 original-curve sums, 70 wrong residuals, 70 norm polynomials, 70 leading coefficients, 70 signed orientation pairs or the 70 sets of Hermite/value vanishing equations. This is a fresh producer-written numerical replay, **not** independently signed PCC and not a formally verified theorem or an isolated performance measurement. The manual verifier was not checked into the repository, so the reproducible Rust source and immutable CI JSON remain the primary published artifacts.

## Interpretation, prior art and next falsifiable experiment

This result gives a small coefficient representation for the original-ideal signed eight-leaf relation, *conditional on knowing the eight leaves*. It does **not** find a factor polynomial from public `R`. The key next experiment is B4/B8 zero-dimensional coefficient elimination with `F(-R)=0` and `(N/(x-x_R)) | H^8`, with exact sign/multiplicity/infinity branches, complete comparison to the independent 26,400 finite-instance B4/B8 signed Semaev decisions, then original Dickson31 negative targets before B31 unknown scalar recovery. Reuse the Rust selected-root/subresultant and F4 code in #4228/#4349; do not enumerate the 48,903,492 B31 multisets and relabel enumeration as algebraic extraction. Existing #4188 m4 polynomial norm/GCD and published APS/PKM/2018 hybrid improvements establish this is not a novelty claim about norms themselves.

No extra complete unknown-Q ECDLP logs, no new rank32 solver cost and no WZ/rho performance ratio were measured in this RR9 job. The separate exact rho comparison remains on open PR #4468. No paid compute, merge or leaderboard promotion.
