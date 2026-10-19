# Protocol: round 7, three leads from round 6

Frozen 2026-10-19, before any round-7 measurement.  Status and background:
[`STATUS.md`](STATUS.md).

**Disclosure.**
- In round 6's conversation, the estimate "coset at d against default at
  d + 1" was computed from E8 records, and the coset came out cheaper.  So
  E11's per-system reading on the E8 curves is not blind.
- E11 therefore tests the lead on new data: random (non-Koblitz) binary
  curves, and a whole-method `ecbench` session.

## E10. Early-stop threshold (lead 1)

- **Inputs.** The 6,000 E8 systems, regenerated from their frozen seed
  (same curves, bases, dimensions and targets).
- **Arms.** `f4-f2-early` with free ∈ {0, 2, 4}.  The full-F4 work and
  solution counts come from `../isogeny_conductor_gap_round6_20261018/e8/records.jsonl`.
- **Gate.** Each early run's solution count equals E8's full-run count on
  every system.
- **P-E10a.** With free = 0, the saving `1 − Σ early / Σ full` is ≥ 0 on
  every configuration with ≥ 1 solvable system.  At most a −1 % loss is
  allowed, since the affine elimination is charged.
- **P-E10b.** Free = 0 keeps at least 80 % of the free = 8 saving on
  configurations where free = 8 saved ≥ 20 %.
- **Refutation.** If free = 0 is negative beyond −1 % anywhere, the loss
  is not the enumeration.

## E11. 2E half of the default span (lead 2)

- **Definition.** "Half base at d" = `trace_coset_basis(d − 1)`, the points
  of `⟨1, …, z^{d−1}⟩` with `Tr(x) = Tr(a)`.
- **E11a (per system, new curves).**
  - **Curves.** `random_binary_instance(n, a, seed)` for n ∈ {19, 23},
    a ∈ {0, 1}, seeds 1–2 (8 curves), keeping curves with cofactor 2 or 4.
  - **Comparison.** The default base at d ∈ {9, 10} against the half base
    at the same d (dimension d − 1).
  - **Measurement.** 200 shared targets, full `f4-f2`, the same profiler
    as E8.
  - **P-E11a.** The half base's maximum solving degree is ≤ the default's
    on every (curve, d).
  - **P-E11b.** Estimated F4 work for a full relation set (columns × work
    per system ÷ lifted decompositions per system) is lower for the half
    base on ≥ 75 % of (curve, d) pairs with ≥ 10 default decompositions.
- **E11b (whole method, `ecbench`, audited).**
  - **Curves.** a = 0 and a = 1 Koblitz over F_2^19 and F_2^23.
  - **Arms.** Rho; MITM and `f4-f2` over `binary-subspace:dimension=9`
    and over `binary-subspace:dimension=8,trace_kernel=1` (the half base
    of d = 9).
  - **Measurement.** 4 targets × 2 rounds.
  - **P-E11c.** On every curve, half-base F4 S is below default-base F4 S
    at weights 0.1 and 1.  Half-base MITM S is below default MITM S across
    the band.

## E12. Hit rate against decomposition count (lead 3)

- **Inputs.** E8 records only, with no new runs.  The hit rate per
  configuration is the fraction of the 200 targets with ≥ 1 lifted
  decomposition.
- **P-E12a.** At F_2^19, a = 0, d = 9, the coset ÷ default hit-rate ratio
  has a 95 % interval (Poisson on hit counts) containing 1.48, round 5's
  `ecbench` yield.
- **P-E12b.** Across E8 configurations, the decomposition-count ratio
  exceeds the hit-rate ratio wherever the default hit rate is ≥ 0.3.
  Saturation compresses the hit rate.
- **Refutation.** If the hit-rate ratio at F_2^19 d = 9 excludes 1.48, the
  discrepancy is not saturation.  The next suspect is `ecbench`'s target
  distribution.

## Reporting

- Each run leaves a report, an SVG graph and a PDF.
- Work ratios are ratios of sums.  The audited session is priced across
  the §5a band (weights 0, 0.1, 1).
- Every IC row reports its distance to matched rho.
