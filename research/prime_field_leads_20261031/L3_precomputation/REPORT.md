# L3: Precomputation and multi-target attacks on fixed curves (write-up)

**Lead.** L3 of `research/notes/ecdlp-general/RESEARCH_PRIME_FIELD_LEADS_20261031.md`. **Decision.** writeup.
**Status.** Derived from published bounds and the atlas cards; no measurement in this directory.

## Question and scope

Which exponents do the known fixed-group precomputation results imply for P-256, Curve25519 and ECCp-131,
and how do they compare with rho? Units are powers of two of group operations. No wall time.

## Sources (as cited in the atlas and in state-of-the-art section 1a)

- Bernstein–Lange 2012, *Computing small discrete logarithms faster*: online about ℓ^{1/3} after
  precomputation about ℓ^{2/3}. Card `KR-RHO-ea34b8` (status proven; constants ≈1.2·ℓ^{2/3} precomputation,
  ≈1.77·ℓ^{1/3} online, table of ℓ^{1/3} distinguished points).
- Corrigan-Gibbs–Kogan 2018 (eprint 2017/1113): S·T² = Ω̃(ℓ) for preprocessing algorithms, S the advice size,
  T the online time. Card `KR-RHO-ea34b8`.
- Kuhn–Struik 2001: L logarithms in one group cost about √(2Lℓ) in total, tight by the generic lower bound
  (Yun 2015). Card `KR-RHO-46c2c6` (status proven).

## Correction to the task wording

The task said "per-curve table of about ℓ^{2/3}". That conflates two quantities. The stored table
(advice) is about ℓ^{1/3} entries. The precomputation work is about ℓ^{2/3} group operations. Both are
reported separately below. Section 1a of the state-of-the-art note and card `KR-RHO-ea34b8` agree with this split.

## Table of implied exponents (log2 of group operations, or of entries where stated)

ℓ is the prime group order. log2 ℓ: P-256 = 256.00, Curve25519 = 252.00, ECCp-131 = 130.19
(computed by bc from the constants in `src/ecc/curve.rs`, `eccp131()`; the task's "≈2^131" is 130.19 bits).

| Curve | rho per target | BL table (entries) | BL precomputation | BL online per target | BL online, k=2^10 total | Kuhn–Struik total, k=2^10 | Kuhn–Struik per target |
|---|---|---|---|---|---|---|---|
| P-256 | 128.00 | 85.33 | 170.67 | 85.33 | 95.33 | 133.50 | 123.50 |
| Curve25519 | 126.00 | 84.00 | 168.00 | 84.00 | 94.00 | 131.50 | 121.50 |
| ECCp-131 | 65.09 | 43.40 | 86.79 | 43.40 | 53.40 | 70.60 | 60.60 |

Column definitions:
- rho per target: ℓ^{1/2}.
- BL table: ℓ^{1/3} stored entries, that is (1/3)·log2 ℓ.
- BL precomputation: ℓ^{2/3} group operations, that is (2/3)·log2 ℓ. Per curve, paid once.
- BL online per target: ℓ^{1/3}. Its check against the CGK line is S·T² = 2^{(1/3)L}·2^{(2/3)L} = 2^{L}, so this point lies on the CGK bound.
- BL online, k=2^10: 2^10 online solves after one precomputation, total = 10 + (1/3)·log2 ℓ.
- Kuhn–Struik total: √(2·k·ℓ), exponent (log2 ℓ + log2 k + 1)/2 = (log2 ℓ + 11)/2 for k = 2^10.
- Kuhn–Struik per target: total minus 10.

Check: the CGK column S + 2T equals log2 ℓ for both endpoints (see `L3_exponents.csv`, last column).

## Graph

`L3_tradeoff.svg` (data `L3_tradeoff_line.csv`, plot script `plot_l3.py`). The x-axis is the table size
exponent t, the y-axis is online exponent (log2 ℓ − t)/2, the CGK line S·T² = ℓ. Circles mark the rho point (t=0,
online = (log2 ℓ)/2), squares the Bernstein–Lange point (t=(log2 ℓ)/3, online = (log2 ℓ)/3). Intermediate points are
derived from the CGK line and are not claimed as attained by any algorithm.

## Status of the claim

This is the only known sub-rho online cost on a standardised curve (P-256, Curve25519) of the kind listed here.
It is **not a break**: the precomputation is 2^{170.67} for P-256 and 2^{168} for Curve25519, larger than rho by about
2^{42.7} and 2^{42} respectively, and about 2^{21.7} for ECCp-131. The per-curve precomputation must be paid before
any online cost counts. The Kuhn–Struik multi-target bound is also sub-rho per target, but its total is √(2kℓ) and it
is tight, so it gives no total below √(2kℓ).

## Limits

- Exponents only; the constants 1.2 and 1.77 from the card, and the √2 in Kuhn–Struik, are dropped in the table.
- Intermediate tradeoff points on the CGK line are not an attained algorithm.
- The standardised-curve claim is restricted to the two standards named. ECCp-131 is a non-standard challenge curve.
- Count of attempted cells: 3 curves × 2 model columns, all computed; no failures.
- A matplotlib cache warning wrote to /tmp (matplotlib default cache). The plot was regenerated only as a
  rendering step from recorded values; no result depends on it. Later plots set MPLCONFIGDIR under TMPDIR.

## Reading

The bounds in the atlas cards fix a tradeoff line, S·T² = ℓ. Rho and the Bernstein–Lange point are its two ends.
The sub-rho online cost is paid for by a precomputation that is itself far above rho, so the line does not give an
attack on any of the three curves.

## Cross-references

- Atlas cards: `KR-RHO-ea34b8` (cited, consistent), `KR-RHO-46c2c6` (cited, consistent).
- State of the art, section 1a of `RESEARCH_ECDLP_STATE_OF_THE_ART.md`; new section appended at its end.
