# Koblitz pair-index local timing screen

Frozen before implementation and timing on 2026-11-16. Source ancestor:
`b13d21e201e673e94d67be8d421febbd260fc3a9`. This bounded L0 screen
asks whether direct first-witness storage can reduce the index component of
the rootfinding collector. It is separate from the frozen complete-call
F4/F5/F6 panel in `../rootfind_f456_batch_combined_20261115/`.
[pcc:unverified]

## Sources and mechanism

The supplied primary rootfinding manuscript and
`../../papers/rootfinding-decomposition/rootfinding_decomposition.tex`
derive the binary pair-table decomposition and charge its construction in a
cold solve. The local program review at
`../../papers/ecdlp-index-calculus-program/sections/rootfinding.tex`
describes the Koblitz pair scan and prior dense-index work. The current
production folded torsion table in `../../src/cryptanalysis/ic_boundary.rs`
retains one first witness per pair-sum abscissa. The source-bound
`../rootfind_inline_page_index_20261116/REPORT.md` and
`../rootfind_paged_pair_index_20261116/REPORT.md` establish storage parity
but have no lookup timing. KB revision `9141e386254340a9bdba3036f2bb706a`
predates these November 16 studies; sources checked (KB):
`source:e9db8cc2be204429e8fb1daa12a3324c` at
`crypto:papers/ecdlp-index-calculus-program/sections/rootfinding.tex` and
`source:17a1b67993e4b781bbf1198fd2598c2f` at
`crypto:papers/rootfinding-decomposition/rootfinding_decomposition.tex`.
No new decomposition or novelty claim is made. [pcc:unverified]

## Fixed inputs and arms

- Exact Koblitz E0 cells n=13/19/23, standard-subspace dimensions 5/7/8,
  signed Frobenius-orbit bases and the full ICV1/base digests pinned by
  `../../tests/rootfind_inline_page_parity.rs`. The ordered folded torsion
  pair stream must have 272/4,830/18,906 insertion attempts, retain
  261/4,800/18,842 first keys, and produce 11/30/64 duplicate-key
  collisions. A full group-equation check of every duplicate is required
  before timing.
- Precompute 64 public residuals per curve using Q=[641]G and
  R_t=Q+[a_t]G. K13 uses a_t=(1000+3t) mod 2003; K19/K23 use
  a_t=105000+3t, for t=0..63. For each residual, compute the key of
  R_t-F_i at every signed base point in order; infinity yields key zero.
  Curve arithmetic and key generation happen before timed sections. The
  fixed probe counts are 2,112/8,896/17,600, including zero keys.
- Timed arms: a reserved-capacity `HashMap<u32,(u32,u32)>` first-witness
  reference and an identical `hash-aa` A/A copy; a flat `2^n+1` array of
  packed first-witness words built **directly**, without an intermediate
  hash; the previously checked 16-key paged index; and its inline-singleton
  version. Every arm inserts the same ordered stream, keeps the first pair
  for each key, and returns exactly the reference answer for every probe.
  The array and page variants reject unsupported indices/keys. No fallback
  or omitted pair is allowed. [pcc:unverified]

## Timing and decision boundary

One release test executable runs seven untimed warmup repetitions followed
by 101 measured repetitions per curve. The five-arm order rotates by
`(repetition + curve_index) mod 5`. Each repetition constructs a fresh
index; record separate `Instant` nanoseconds for index allocation/insertion
and for the complete fixed lookup sequence, then their sum. Use
`std::hint::black_box` on lookup results and verify a deterministic
first-witness checksum outside each timed interval. Emit every measured
arm/repetition row as JSON. Stop on an identity, collision, first-witness,
checksum or allocation mismatch. An outer 600-second watchdog retains any
partial output and does not restart automatically. [pcc:unverified]

The descriptive comparison is each candidate's median build, lookup and
combined time against hash, with the hash-aa/hash combined ratio showing
local noise. A candidate is eligible for production integration only if it
preserves every witness and its combined median is below hash beyond the
absolute A/A median difference on at least one cell. Otherwise prefer the
existing hash layout for this screen. Report all three cells and every
failed/censored row. This L0 component timer excludes pair-sum group
arithmetic, target generation, relation rank, verification, process startup,
peak RSS and the F4/F5/F6 solve. It cannot establish complete-call speed.
[pcc:unverified]

Commit this protocol, then source, before timing. Bind the release binary,
source, Cargo lock, exact command, host, UTC, raw output and derivation hashes.
Generate a run report, editable diagram/chart and inspected PDF, including
negative or censored outcomes. The frozen EKS panel's budget, isolation and
independent-replay gates remain unchanged. [pcc:unverified]
