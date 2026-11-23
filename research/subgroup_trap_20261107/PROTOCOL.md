# PROTOCOL: subgroup traps in lift-dense factor bases

Frozen 2026-11-07, before any computation in this directory. The data under
test was frozen on 2026-10-27, in `research/c1_twisted_line_ladder_20261027/`
(run `reports/run-20261027T1502Z/`, PR #2704). The checks are exact: each
prediction is an algebraic statement, and a single violation refutes it.
There is no statistics.

## Question

The C1 ladder's positive control A0 (E: y² + xy = x³ + 1, so a = 0 and b = 1)
found zero decompositions on every sampled target at k = 7 and 9 for the
standard base and for `tw_a` (u = 1, the same line). The prediction was about
15.5 per target. At k = 5 its counts were extremely clustered (sd 954 for a
mean of 17). H-Y1 and H-Y3 failed on this curve. Why?

## Hypotheses (stated before computing)

Notation: Q = 2^k, the field is F_{Q⁴}, n = 4. A factor base FB(u) is the set
of points P with x(P) ∈ u·F_Q^*. Let N(R) be the ordered count of
4-decompositions of the target R over FB, as computed by the frozen
instrument (`yield_.rs`, `PairTable::count`).

**T1 (subfield trap).** Let K be the field generated over F_2 by the line
u·F_Q and the coefficients a and b. Every point with x ∈ K has y ∈ K₂, the
quadratic extension of K, so FB ⊆ E(K₂), and E(K₂) is a subgroup. When
2[K:F_2] divides N and is less than N, every sum of factor-base points lies
in the proper subgroup E(K₂), and N(R) = 0 for every R outside it. A finer
statement: points with x ∈ F_Q satisfy either σ_Q P = P or σ_Q P = −P, so FB
lies in H = E(F_Q) + ker(σ_Q + 1). Since E(F_Q) ∩ ker(σ_Q + 1) = E(F_Q)[2],
which has order 2, H has index 2 in E(F_{Q²}).

**T2 (2E concentration).** For a binary ordinary curve, P ≠ O lies in
2E(F_{2^N}) if and only if Tr(x_P) = Tr(a). In C1 every factor base has
Tr(x) = 0. On the standard base, Tr(y) = 4·Tr_k(y) = 0. On a valid line,
Tr(yu) = Tr_k(y · Tr_{Q⁴/Q}(u)) = 0. Hence:
- if Tr(a) = 0, FB ⊆ 2E, every 4-sum lies in 2E, and N(R) = 0 whenever
  Tr(x_R) = 1;
- if Tr(a) = 1 (NEG), FB lies in the other coset, a sum of four points from
  that coset lies in 2E, and again N(R) = 0 whenever Tr(x_R) ≠ Tr(a).

## Predictions

- **P1 (A0, k = 5, 7, 9).**
  - Every FB point of A0 `std` and `tw_a` satisfies σ_Q² P = P and lies in H.
  - Every regenerated target R with observed count N(R) > 0 lies in H.
  - Every regenerated target outside E(F_{Q²}) has an observed count of 0.
  - A0 `tw_b` (u ∈ F_16 \ F_2) is not trapped: some FB point is outside
    E(F_{Q²}).
- **P2 (A0, exact mass; k = 5 and 7, where enumeration is cheap).**
  - Over all R ∈ E(F_{Q²}), ΣN(R) = m⁴, where m = |FB|.
  - The support {R : N(R) > 0} is contained in H, and |H| = #E(F_{Q²})/2.
  - The mean count on H is m⁴/|H|.
- **P3 (2E, every frozen yield row at k = 5, 7, 9, 11, every curve and arm).**
  - Every target with Tr(x_R) ≠ Tr(a) has observed count 0.
  - Reported, not a pass/fail condition: the fraction of targets with
    Tr(x_R) = Tr(a), and the mean count on that half divided by `pred_excl`
    (expected near 2).
- **P4 (applicability audit).** For every line that the merged studies call
  applicable, compute [K:F_2] and evaluate the trap condition:
  - c2pnb208w1 and c2pnb368w1, every valid u ∈ F_{2^16} (180 and 330);
  - Oakley groups 3 and 4, the found u;
  - C1 A1, A2, B1–B3, both lines each.

  Predicted: no applicable line is trapped. For c2pnb, the trap needs u, a
  and b all in F_16. For Oakley, n = 5 is prime and u ∉ F_q, so
  F_q(u) = F_{q⁵}. For C1, b ∉ F_4 on A1 and A2, and b is random on B.

## Procedure

- New crate `research/subgroup_trap_20261107/`, std-only Rust.
- The field, curve, factor-base and counting code is reused unchanged from
  the C1 crate (the frozen instrument, checked against its `FREEZE.sha256`).
- Targets are regenerated with the run's seed and tags (`targets-{k}-{id}`).
- Each regenerated target's count is recomputed with `PairTable::count` and
  must equal the frozen count. This is an identity check on the
  regeneration; it tests nothing algebraic.
- The c2pnb parameters are the run's own copy:
  `research/lift_dense_factor_base_20261027/reports/run-20261027T0356Z/c2pnb_input_binary-curve-params.txt`.
- Degrees [F_2(z):F_2] are the least j with z^{2^j} = z.
- Execution: local, on the session host. These are exact algebraic checks of
  frozen data, not timings or benchmarks; there is no isolab-eks
  submission (AGENTS.md §9a), as for the parent studies.

## What each outcome means

- **P1 and P2 hold:** A0 is a subfield-trapped control. It was never a valid
  positive control for the yield identity, and its H-Y1 and H-Y3 failures
  are explained exactly. The verdicts stay as registered; this explains them
  and does not re-score them.
- **P3 holds:** the yield identity averaged over E hides a factor of 2 on
  2E. Cryptographic targets (odd order) lie in 2E. For even n this cancels
  in the twisted/standard ratio. For odd n it does not, which is a derived
  statement and is not tested here.
- **P4 finds a trapped applicable line:** the matching claim in #2686 or
  #2704 is wrong and must be corrected.
- **Not claimed:** any ECDLP cost, or anything about ECC2K-130 beyond the
  degree argument: N = 131 is prime, so K ∈ {F_2, F_{2^131}} and no subfield
  trap exists.

## Amendment 1 (2026-11-07, before any computation): the trap condition

T1 as frozen gives the trap condition as "2[K:F_2] divides N and is less
than N". That is incomplete. A point with x ∈ K that lies on E(F_{2^N}) has
y ∈ K₂ ∩ F_{2^N}:
- this intersection is K₂ when 2[K:F_2] divides N;
- it is K itself when N/[K:F_2] is odd.

So FB ⊆ E(K*), with K* = K₂ ∩ F_{2^N}. The corrected condition: **trap if
and only if [K:F_2] < N and 2[K:F_2] ≠ N.**

In the borderline case 2[K:F_2] = N, FB still lies in
H_K = E(K) + ker(σ_K + 1), the subgroup of index 2 in E(F_{2^N}). When
Tr(a) = 0, every point of H_K has Tr(x) = 0, so H_K ⊆ 2E. Both subgroups have
index 2, so H_K = 2E. This is the T2 case and is benign.

P4 uses the corrected condition. Consequences:
- For odd N (Oakley, N = 155 and 185), K ⊋ F_q gives K = F_{2^N}, so there
  is no trap.
- For c2pnb, [K:F_2] ∈ {4k, 8k, 16k}, and a trap occurs only at 4k (u, a and
  b in F_16); the prediction is unchanged.
- For C1, [K:F_2] ∈ {k, 2k, 4k}, and a trap occurs only at k.
- A0 `tw_b` has u ∈ F_16. If deg u = 2, then [K:F_2] = 2k = N/2, so its FB
  lies in H_K = 2E. P1's statement "some FB point lies outside E(F_{Q²})" still
  holds, and P3 covers its zeros.
