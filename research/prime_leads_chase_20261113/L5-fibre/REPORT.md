# L5-fibre — KN-OPEN-08ba7e: compact summation message for two-isogeny-fibre factor bases

2026-11-13. Every number below is `[pcc:unverified]` (local toy runs on a shared VM; no pcc claim,
no bench.record, no EKS, nothing committed). Protocol: `PROTOCOL.md` (sha256 in `PROTOCOL.sha256`,
frozen before measurement); amendments A1, A2 in `AMENDMENTS.md`.

## Verdicts

| Sub-question | Verdict | Basis |
|---|---|---|
| 1. What the current network computes / query cost | Explained; the shipped "message" compresses **local membership only**; query cost is coordinate-space enumeration (measured) | crate source + 5 crate runs |
| 2. Is the summation-message width sub-generic for fibre bases? | **KILL at toy scale** (amended reading A1). Under the letter of the original rule: **INCONCLUSIVE** (47/48 cells in band; the one outlier is a B=16 collision fluctuation, resolved by A1 over 30 curves, in the *more*-generic direction) | measured, 12–26-bit, m=2,3 |
| 3. Does the fibre structure gain or only relabel? | **Relabel** — exact group isomorphism to an x-coordinate base on E′ (proof below; numerically confirmed 5/5 crate instances: crate = direct Python on E = Python on Vélu codomain E′) | derived (D) + measured |
| Lead overall | **Relabel; not a lead.** Reduces to IC on E′ with an x′-defined base (closure rows A4/A6, and A11 when the 2:1 fibre is kept) | |

## Sources checked
research-kb and the lead atlas are **not available** in this environment (per COMMON.md). Grepped both
repos (+ papers dirs) for KN-OPEN-08ba7e / "summation boundary" / sumset width: hits only in
`crypto/docs/structured-prime-ic.md`, `crypto/research/structured_ic_integration_20261104/README.md`,
`crypto/research/structured_ic_postmerge_20261105/README.md`, `cryptanalysis/docs/ic/structured-prime-ic-20261104/README.md`
and session logs — none measures the summation-message width or states the E′ reduction.
Background read: closure note `research/notes/index-calculus/PRIME_FIELD_IC_CLOSURE_20261112.md`,
`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` §1–2.4.

## 1. What the current network computes (crates/hashed-ecdl-tensor, `structured.rs`)

- `IsogenyFibres`: kernel T=(t,0); image abscissa X(P) = u + 3t + (3t²+a)/u, u = x−t. This is the Vélu
  2-isogeny x-coordinate shifted by 2t (X = x_Vélu(φ(P)) + 2t; verified: Vélu images lie on E′ in every run).
  F_S = {P : [r]P = O, P ≠ T, X(P) ∈ S}, both y-signs.
- `Prepared::new` builds ONE Boolean/arithmetic network over bit-encoded points: m input points
  (1 flag + 2·⌈log p⌉ coordinate bits each), each with a membership constraint, a chain of complete
  point additions, and the target point as pinned interface bits with `assert_point_equal`.
  Membership is either the circuit (zero-test u≠0, product of |S| quadratic factors = 0, and a full
  double-and-add circuit for [r]P = O) or `fibre_message`: one reduced decision diagram over the
  2⌈log p⌉ coordinate bits built from quadratic roots and lifts (12–22 nonterminal nodes at the toys).
- `count` = exact number of ordered signed m-tuples (repetition allowed) summing to R; `witness` =
  prefix-count self-reduction (one count per interface bit) then arithmetic replay.
- Backend actually used: `Functional` (finite-size fallback; the Appendix D recursion never engages at
  these sizes): it enumerates free program inputs (the coordinate words) with dependency pruning.
  **There is no summation message**: nothing is shared across slots except the local membership DD.

Measured query cost (crate diagnostics, uncalibrated "charged steps", B=|F|=4 fixed):

| p | r | m | membership | steps / query (min–max over 4 targets) | wall (4 queries) | counts = Python (E) = Python (E′) |
|---|---|---|---|---|---|---|
| 11 | 7 | 2 | circuit | 2.01e8–2.18e8 | 115 s | yes |
| 11 | 7 | 2 | fibre_message | 4.94e7–5.34e7 | 26 s | yes |
| 19 | 7 | 2 | fibre_message | 1.54e8–1.55e8 | 83 s | yes |
| 43 | 13 | 2 | fibre_message | 2.45e8–8.88e8 | 369 s | yes |
| 11 | 7 | 3 | fibre_message | 4.38e8–4.52e8 | 467 s | yes |
| 5 (repo fixture) | 3 | 2 | fibre_message | 7.7e6–8.0e6 | 1.8 s | (fixture) |

Cost grows with the coordinate space (≈ p^{1.1–2.1} from p=11→43 at m=2 with B fixed) and by ≈×p per
extra slot (m=2→3 at p=11: ×8.5), i.e. it is not even B^m enumeration; the local message saves ×4 over
the circuit (constant). Matched rho at these sizes is ≈ √r ≈ 3–4 group operations.

**What "full constrained summation boundary message" means.** A target-independent object M_j that
represents, after the first j slots, which partial sums s = P_1+…+P_j (P_i ∈ F) are reachable (with
multiplicities for counting), such that pinning R and closing slots j+1..m costs ≪ B. For the
point-ordered diagram / tensor network with R free, the minimal exact bond at cut j is exactly
W_j = |jF| (rows indexed by distinct partial sums have disjoint column supports, so they are distinct and
linearly independent). With R pinned, the minimal width is w_j(R) = |jF ∩ (R − (m−j)F)| — tiny (measured
mean max-cut ≤ 2.2 for m=3) but it *is* the set of partial solutions, so building it is the decomposition
problem itself. In the Fourier basis of Z_r the message is diagonal: N_m(R) = r^{-1} Σ_χ \hat{1_F}(χ)^m χ(−R);
an approximately compact message needs \hat{1_F} concentrated on few characters.

## 2. Measurement (exact, independent Python; logs by BSGS; `code/widths.py`, `code/widths_big.py`)

Arms per prime, same B: A1 fibre (random S), A2 fibre (interval S={0,1,2,…}), I interval-x on E,
P positive control (AP of logs {±1..±B/2}), N three random symmetric subsets of G. Curves have
#E = 2r or 4r, p ≡ 3 mod 4. W_j = |jF| exact; ratios to mean(N). Positive control passes at every
B ≥ 32 (W2/N ≤ 0.13, W3/N ≤ 0.02 at 26 bits), so the detector sees additive structure when present.

| bits | r | B rule | B | W2 N (mean) | W2 A1/A2/I ÷ N | W3 N (mean) | W3 A1/A2/I ÷ N | W2,W3 AP-control ÷ N | top-B Fourier energy A1/A2 vs N |
|---|---|---|---|---|---|---|---|---|---|
| 12 | 1319 | p^1/3 | 14 | 96 | 1.028/1.028/1.028 | 400 | 1.165/0.965/1.120 | 0.301, 0.107 | 0.0686/0.0799 vs 0.0797 |
| 12 | 1319 | p^1/4 | 8 | 30 | 1.112/1.112/1.112 | 77 | 1.241/1.241/1.241 | 0.573, 0.323 | 0.0406/0.0403 vs 0.0424 |
| 12 | 1319 | p^1/5 | 8 | 33 | 1.000/1.000/1.000 | 88 | 1.091/1.091/1.091 | 0.515, 0.284 | 0.0404/0.0403 vs 0.0429 |
| 14 | 4079 | p^1/3 | 20 | 193 | 1.021/1.041/0.979 | 1100 | 1.044/1.095/0.977 | 0.212, 0.055 | 0.0416/0.0423 vs 0.0448 |
| 14 | 4079 | p^1/4 | 10 | 51 | 1.000/1.000/1.000 | 180 | 1.000/1.000/1.000 | 0.412, 0.172 | 0.0193/0.0193 vs 0.0193 |
| 14 | 4079 | p^1/5 | 8 | 33 | 1.000/1.000/1.000 | 96 | 1.000/1.000/1.000 | 0.515, 0.260 | 0.0140/0.0140 vs 0.0140 |
| 16 | 14629 | p^1/3 | 40 | 786 | 0.994/0.958/0.997 | 7638 | 1.015/0.928/0.987 | 0.103, 0.016 | 0.0271/0.0314 vs 0.0279 |
| 16 | 14629 | p^1/4 | 16 | 129 | 1.000/0.953/1.000 | 696 | 0.979/0.885/1.002 | 0.256, 0.070 | 0.0111/0.0124 vs 0.0111 |
| 16 | 14629 | p^1/5 | 10 | 51 | 1.000/1.000/1.000 | 180 | 1.000/1.000/1.000 | 0.412, 0.172 | 0.0060/0.0059 vs 0.0060 |
| 18 | 106957 | p^1/3 | 60 | 1792 | 0.995/0.999/0.991 | 30858 | 0.991/0.981/0.980 | 0.068, 0.006 | 0.0073/0.0075 vs 0.0072 |
| 18 | 106957 | p^1/4 | 22 | 243 | 1.000/1.000/1.000 | 1796 | 0.982/1.004/1.004 | 0.185, 0.037 | 0.0027/0.0026 vs 0.0028 |
| 18 | 106957 | p^1/5 | 12 | 73 | 1.000/1.000/1.000 | 304 | 1.000/1.000/1.000 | 0.342, 0.122 | 0.0012/0.0012 vs 0.0012 |
| 20 | 403951 | p^1/3 | 94 | 4402 | 0.998/0.996/1.000 | 117052 | 0.997/0.998/1.008 | 0.043, 0.002 | 0.0035/0.0035 vs 0.0035 |
| 20 | 403951 | p^1/4 | 30 | 449 | 1.004/1.004/1.004 | 4469 | 1.013/1.011/1.016 | 0.136, 0.020 | 0.0011/0.0011 vs 0.0012 |
| 20 | 403951 | p^1/5 | 16 | 129 | 1.000/1.000/1.000 | 704 | 1.000/1.000/1.000 | 0.256, 0.070 | 0.0005/0.0005 vs 0.0005 |
| 22 | 1911083 | p^1/3 | 156 | 12140 | 0.999/0.999/0.999 | 538795 | 1.000/1.000/0.998 | 0.026, 0.001 | 0.0014/0.0014 vs 0.0014 |
| 22 | 1911083 | p^1/4 | 44 | 969 | 1.000/1.000/1.000 | 14218 | 0.998/0.998/0.999 | 0.092, 0.009 | 0.0004/0.0004 vs 0.0004 |
| 22 | 1911083 | p^1/5 | 22 | 243 | 1.000/1.000/1.000 | 1804 | 1.000/1.000/1.000 | 0.185, 0.037 | 0.0002/0.0002 vs 0.0002 |
| 24 | 8166589 | p^1/3 | 254 | 32184 | 1.000/1.000/1.000 | 2319061 | 1.000/1.001/0.999 | 0.016, 0.000 | 0.0006/0.0006 vs 0.0006 |
| 24 | 8166589 | p^1/4 | 64 | 2049 | 1.000/1.000/1.000 | 43667 | 1.001/1.001/1.000 | 0.063, 0.004 | 0.0002/0.0002 vs 0.0002 |
| 24 | 8166589 | p^1/5 | 28 | 393 | 1.000/1.000/1.000 | 3696 | 1.000/1.000/1.000 | 0.145, 0.023 | 0.0001/0.0001 vs 0.0001 |
| 26 | 16808513 | p^1/3 | 324 | 52420 | 1.000/1.000/1.000 | 4814310 | 1.000/1.000/0.999 | 0.012, 0.000 | n/a/n/a vs n/a |
| 26 | 16808513 | p^1/4 | 76 | 2889 | 1.000/1.000/1.000 | 73131 | 1.000/0.998/0.999 | 0.053, 0.003 | n/a/n/a vs n/a |
| 26 | 16808513 | p^1/5 | 32 | 513 | 1.000/1.000/1.000 | 5499 | 1.001/1.001/1.001 | 0.127, 0.018 | n/a/n/a vs n/a |

(Full per-arm table incl. pinned widths and max Fourier coefficient: `out/analysis_all.md`; raw rows
`out/widths.jsonl` (12–24 bits, FFT instrument), `out/widths_26.jsonl` (26 bits, FFT-free instrument; A/A
identical to FFT on 42/42 rows at 18/20 bits, `out/widths_direct.jsonl`). Plot `out/widths.svg`.)

**Rule outcome (original, frozen):** positive control passes; 47/48 fibre cells have |ratio−1| ≤ max(band,
0.10). The exception: 16 bits, B=16, A2 (W2 0.953, W3 0.885), band 0.02 from 3 seeds with ~0 spread.
By the letter of the rule → **INCONCLUSIVE**. No cell approaches the SURVIVE threshold (ratio ≤ 0.5).

**Amendment A1 (30 curves at 16 bits, 10 controls each, `out/followup16.json`)**: excess additive
coincidences c_j = mean_N(W_j) − W_j (positive = sub-generic), mean ± s.e.:

| B | A1 c2 | A1 c3 | A2 c2 | A2 c3 | I c2 | I c3 | N c2 | N c3 |
|---|---|---|---|---|---|---|---|---|
| 16 | −0.15±0.30 | 0.66±4.45 | −0.41±0.17 | −6.67±2.58 | −0.15±0.25 | −1.0±3.8 | 0.00±0.08 | 0.0±1.17 |
| 28 | −0.78±0.80 | −7.7±20.3 | 0.02±0.67 | 5.5±17.3 | 0.29±1.01 | −1.0±19.9 | 0.0±0.29 | 0.0±6.9 |

No fibre arm exceeds controls by 3 s.e.; the only ≥2 s.e. effect (A2, B=16) is in the *generic* direction.
Spectra: top-B Fourier energy of fibre bases equals the random control to ≤0.004 absolute at every
14–24-bit cell (≤0.012 at 12 bits) (flat spectrum; no low-rank approximate message).

Observed growth: W2 ≈ B²/2 + 1 and W3 ≈ min(c·B³, r) exactly as the generic prediction min(B^j, p)
(up to symmetry constants) for all fibre and interval arms.

## 3. Theory: relabel, with the reduction

**Proposition (exact relabel).** Let G ≤ E(F_p) have odd prime order r, T = (t,0), φ: E → E′ = E/⟨T⟩ (Vélu).
ψ := φ|_G : G → G′ := φ(G) is a group isomorphism (ker φ ∩ G = {O}), with inverse [2^{-1} mod r]∘φ̂, both
O(log r) group operations (φ itself O(1) field operations). Since X(P) = x(φ(P)) + 2t,
F_S = ψ^{-1}(S′), S′ := {Q ∈ G′ : x(Q) + 2t ∈ S}. Hence for every m and R ∈ G:
#{(P_i) ∈ F_S^m : ΣP_i = R} = #{(Q_i) ∈ S′^m : ΣQ_i = ψ(R)}, witnesses correspond coordinatewise,
and every DD/tensor-network/message for one side transfers to the other with identical widths
(|jF_S| = |jS′|) and O(1) field work per point. Measured: 5/5 crate instances satisfy it exactly.

Consequences:
1. A fibre base is an **x-coordinate base on E′**. With S an interval it *is* the interval-x base on E′
   (closure row A6: MITM/Boolean-Macaulay fits 0.69/0.80/≥1.10 vs 0.5); with S random it is a random
   x-defined base. The only freedom is the choice of S on E′, which any IC on E′ already has.
2. No algebraic degree gain: membership as a condition on x′ is ∏_{s∈S}(X − s) of degree |S| ≍ |F|
   (§2.3 of the program: degree = B), and on E it is ∏ of |S| quadratics, degree 2|S| — worse.
3. Without the [r]P=O restriction (full fibres F̃ = φ^{-1}(S′), closed under +T), each Q-tuple has 2^m
   lifts of which exactly half sum to R: N^{E,F̃}_m(R) = 2^{m−1} N^{E′,S′}_m(φ(R)) — closure row A11
   (base |B|/ℓ, ℓ=2). Either way the 2-isogeny contributes nothing beyond the E′ problem.
4. Why no compact summation message can come from "structure" here: |2F| ≤ K|F| forces (Freiman–
   Green–Ruzsa in Z_r) the **discrete logs** of F into a coset progression of dimension d(K) and size
   f(K)|F|. For an x-defined set that would be a correlation between x-coordinates and logs, itself an
   ECDLP leak; measurements show none (all fibre/interval ratios within 0.02 of 1 at ≥ 18 bits; flat spectra). The
   one base family that does have it (AP of logs, positive control) needs known logs and is kangaroo/BSGS.

## Distance to matched rho (exponent; rho = 1/2)

| Route | Per-relation / total | Exponent | Distance |
|---|---|---|---|
| Crate network as shipped (Functional backend) | per query ≥ coordinate-space enumeration (measured ≈ p^{1.1–2.1}, m=2) | ≥ 1 per query | ≥ p^{1/2} per *query* (rho solves whole DLP in √r) |
| Generic-width message (= explicit (m−1)-sum table; what the measured widths allow) | p/B^{m−2} + B^{m−1} (+ mB²), optimum B = p^{1/(2m−3)} | (m−1)/(2m−3): m=3: 0.667, m=4: 0.600, m=5: 0.571 | 1/(2(2m−3)): 0.167, 0.100, 0.071 — never ≤ 0 (consistent with S·T² ≳ n) |
| §2.4 requirement at m=4, B≈p^{1/5} | needs T_dec ≪ p^{1/4} **and** no B^{3} table build | — | generic message gives T_dec = B = p^{1/5} but needs a p^{3/5} table: total p^{3/5} |

Derived, not measured: the formulas assume the measured generic widths persist (they did, 12–26 bits).

## What is NOT shown
- No lower bound against all compact messages: a non-point-ordered or algebraic (x-coordinate, Semaev/F4)
  representation is not excluded by width measurements; it is the E′ IC problem, already covered (and not
  closed by proof) by closure rows A4–A6 / §6.
- Bit-level OBDD widths of the summation predicate were not computed (point-level widths are a lower bound
  for any order that keeps each point's bits contiguous; interleaved bit orders not measured).
- m = 4 widths not measured (W3 suffices for m=4's (m−1)-sum message; |4F| not computed).
- Crate queries only at p ≤ 43 (cost); censored nothing; 26 bits is the largest width measurement.
- Fourier spectrum not computed at 26 bits (memory, amendment A2).

## Next action with more compute
None for this lead as a factor-base family (it is a relabel). If KN-OPEN-08ba7e is kept, re-scope it as
"compact summation message for x-defined bases on any curve", i.e. the open F4/Semaev question of closure
§6.2, and close the fibre-specific item with the proposition above.

## Evidence
`PROTOCOL.md`, `PROTOCOL.sha256`, `AMENDMENTS.md`, `code/` (ec.py, widths.py, widths_big.py, widths_rfft.py
(failed MemoryError attempt, log kept), crate_xcheck.py, followup.py, analyse.py, plot.py),
`out/xcheck1/`, `out/xcheck2/` (crate requests/receipts + xcheck.json), `out/widths*.jsonl|log`,
`out/followup16.json|log`, `out/analysis_all.md`, `out/table_condensed.md`, `out/widths.svg`,
`MANIFEST.sha256`. Crate binary built at `../target-shared/release/tensor-structured` from
/home/user/crypto/crates/hashed-ecdl-tensor (structured.rs sha256 675bedac…).
