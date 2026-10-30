# PROTOCOL: twisted-line applicability on the Oakley EC2N curves

Frozen 2026-10-27 before the final run. Harness: `src/main.rs`, `Cargo.toml`
(std-only Rust; no Python for computation). Master seed `0x0AC1_2026_1027`
(per-curve stream = seed ^ N*0x10000000001B3 ^ d, SplitMix64).

Disclosure: one trial run of the same binary (200 samples, max 2000 lines)
was made while debugging, before this file was written. It showed the same
qualitative outcome as the frozen run (same seed, so the same numbers). Nothing
was tuned afterwards.

## Question

Does the twisted line u*F_q (Lean: `twisted_line_lifts`,
`research/lift_dense_factor_base_20261027/lean/LiftDense.lean`) exist on the
Oakley EC2N groups 3 and 4 with n = 5 (d = 31, 37)? If yes, a Gaudry-Diem
decomposition over that base has every nonzero x lifting, against about half
for the standard base x in F_q: a 2^n = 32x per-trial relation-yield
statement, not a cost measurement.

## Parameters and sources

E: y^2 + xy = x^3 + a x^2 + b over F_{2^N}, a = 0 for both.

| group | N | field polynomial | b | group order (RFC) | generator x |
|:--|:--|:--|:--|:--|:--|
| 3 | 155 | u^155 + u^62 + 1 | 0x07338f | 0x0800000000000000000057db5698537193aef944 | 0x7b |
| 4 | 185 | u^185 + u^69 + 1 | 0x1ee9 | 0x01ffffffffffffffffffffffdbf2f889b73e484175f94ebc | 0x18 |

Source: RFC 2409 sections 6.3 and 6.4 (https://www.rfc-editor.org/rfc/rfc2409.txt,
fetched 2026-10-27, sha256
`5b19aab3886800c3ce7528e0dba4eb05dc6405e03388ed4dd3032fb02f4616d9`). The same
values are recorded and were checked against the RFC on 2026-10-23 in
`research/ghs_oakley_extended_20261023/REPORT.md` and
`research/cover_route_prime_binary_20261008/deployed_binary.txt`. The harness
asserts that the RFC modulus hex equals u^N + u^m + 1.

Positive/negative control: the five ANSI c2pnb curves, parameters from
`research/lift_dense_factor_base_20261027/reports/run-20261027T0356Z/c2pnb_input_binary-curve-params.txt`
(copied into the run directory with its sha256), d = N/4, n = 4.
Expected: 208w1, 368w1 lift 200/200; 176v1, 272w1, 304w1 (Tr_N(a) = 1) lift
0/200 for every valid u.

## Algorithm

q = 2^d, T = Tr_{F_{2^N}/F_q}, c = sqrt(b).
1. Field checks: x^(2^N) = x mod f and gcd(f, x^(2^(N/p)) - x) = 1 for every
   prime p | N.
2. Group-order check: [order * cofactor] P = O for the generator (Oakley) and
   3 random curve points; for Oakley also [order] P = O (cofactor 1 as
   recorded). This is a consistency check on the transcription, not a proof of
   the order.
3. Tr_N(a) computed.
4. Search for u: u0, u1 in ker T drawn as z + z^q (the image of 1 + Frobenius_q
   equals ker T). For u in ker T, T(c/u) = N(u)^-1 G(u), G(u) =
   T(c * prod_{i=1}^{n-1} u^(q^i)). On u = u0 + t u1, t in F_q,
   G = P(t) = sum_k T(coef_k) t^k, coef_k the coefficients of
   c * prod_i (u0^(q^i) + t u1^(q^i)) (t^(q^i) = t). This expands the product
   directly; it is equivalent to evaluating at n+1 points and interpolating,
   and is cross-checked by comparing P(t) with the direct G(u0 + t u1) at 3
   random t per line (a mismatch sets a failure flag).
   Roots of P in F_q: h = t^q mod P (d squarings), g = gcd(P, h - t), then
   splitting by gcd(g, Tr_{F_q/F_2}(beta t)) for random beta in F_q, with F_q
   elements drawn as Tr_{N/d}(random). Fresh lines are drawn until 3 verified
   u are found or 2000 lines are tried.
5. Direct verification of each found u in F_{2^N}, without the derivation:
   T(u) = 0 and T(c * u^-1) = 0 (inverse by u^(2^N - 2)).
6. Lift sampling: 200 random nonzero y in F_q (Tr_{N/d} of random element),
   x = u y for each of the (up to) 3 verified u, and x = y for the standard
   base; lift test Tr_N(x + a + b/x^2) = 0. Control: one random u in ker T
   (T(c/u) != 0 expected), 200 samples.

## Decision rule

Twisted line **applicable** iff a u is found and verified directly (step 5)
and 200/200 sampled line elements lift (step 6, for the first u; the other
two are reported). Otherwise **not applicable / not found**, reported with the
number of lines tried. Positive control passes iff 208w1 and 368w1 give
200/200 and 176v1, 272w1, 304w1 give 0/200 on every valid u.

## Claim limits

200 samples per line is a check of the theorem, not a proof of it. A
per-trial relation-yield factor 2^n = 32 is not a measured cost; the cost of
linear algebra, the factor-base size, the Weil-descent system solving and the
unpriced native work (AGENTS.md section 5a, 0/0.1/1 GAE band) are not
measured here. Not a leaderboard or benchmark row; no VALIDATION.json.

## Addendum (post-run, 2026-10-27): execution route

This run was local on the session host. AGENTS.md §9a makes isolab-eks the
default for x86 CPU experiments, so the result is local evidence, and the §9a
confirmatory EKS run of the same binary and seed is **open**. The blocker was
that this session had no AWS identity or CLI. See
`../lift_dense_followups_20261027/README.md`, "Execution route".
