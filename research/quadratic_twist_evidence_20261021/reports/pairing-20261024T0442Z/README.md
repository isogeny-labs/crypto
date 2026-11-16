# Twist-assisted pairing transfer to finite-field index calculus

2026-10-24 · `pairing-20261024T0442Z` · PR #2381.

[Inspected report PDF](report.pdf) · [editable report](report.tex) ·
[transfer diagram](bridge.svg) · [recorded-count graph](auxiliaries.svg) ·
[native results](../../native/pairing-20261024T0442Z/results.json) ·
[independent audit](../../native/pairing-20261024T0442Z/audit.json) ·
[validation and failures](../../validation/pairing-20261024T0442Z/README.md).

The user requested work on the positive pairing connection: a twist point can
provide the second pairing argument in a reduction to a finite-field logarithm,
where index calculus applies. This round executes that route and tests its
degenerate cases. The preceding direct mixed-relation obstruction remains true
under its hypotheses; it does not rule out this different transfer.

## Checked finite result

On `icv1-fp6-t0-74779950`, E/F43: y²=x³+x, take r=11, P=(31,18),
Q=(13,24)=7P. The monic twist `icv1-fp6-t0-c6a8a47c` has equation
Y²=X³+4X and point T=(35,12). In K=F43[u]/(u²−2), the existing map
psi(T)=(X/2,Y/(2u)) gives S=(39,3u) on `icv1-fp6k2-tm86-5a651060`.
The explicit Miller formula gives

```
g = e(P,S) = 2 + 25u
h = e(Q,S) = 11 + 19u = g^7
g^11 = 1, g != 1, g^43*g = 1.
```

Lean checks the actual Mathlib elliptic points, source scalar 7, twist-source
binding, rational/minus Frobenius signs and **all 121 generator pairs**:
`weil11([a]P,[b]S) = some(g^(a*b))` for a,b in Fin11. It checks the eleven
powers of g are distinct and proves exact logarithm transport. The function is
defined by line functions and a linear Miller chain, with support collisions
returning `none`; it is not defined by a logarithm lookup. The checks use kernel
evaluation, not trusted native evaluation. The implementation does not prove
equality with an independently defined geometric divisor pairing in general.

The native finite-field IC solver recovers **all eleven scalar logarithms**;
the independent Rust auditor checks every projected factor-base log, all eleven
full-rank relation matrices, target exponentiations, pairings and actual source
points. Rank is six in each matrix. Seeds are 430000+n, n=0,...,10. Attempt
counters are producer records, not an independently rerun random search.
Lean formalizes the finite transfer, not the IC relation collector/solver.

These exact representations already exist in the frozen model source:

| Representation | EC1 and full UID suffix after `urn:ec-record:1:sha256:` |
| --- | --- |
| E/F43 with P, order11, cofactor4 | `EC1P6Cfphdc3583992fa5` · `dc3583992fa5d34a7f998b4786113448898fbb0b6ee8e83c46112ea0671dce65` |
| Monic twist with T, order11, cofactor4 | `EC1P6Cfph475f3b57bdcd` · `475f3b57bdcdbf30323c03190ca556ecd6ec46ef34c2a1101417d8c1df8beba4` |
| E/K with P, order11, cofactor176 | `EC1Q43D2Cfpkha6700eccde45` · `a6700eccde456f4e8c5d1ea6827b866301d5a56f853a516fe695eb2e84309e76` |
| E/K with S, order11, cofactor176 | `EC1Q43D2Cfpkh135d5e693fd5` · `135d5e693fd5f90d7628eef10e8b3b6e204e2f99b5897070297ba73ad7b43a80` |

Source: [existing model/representation records](../../../../docs/curves/sources/quadratic-twist-native-20261021.json).
The registry and those records are unchanged in this round.

## Exhaustive auxiliary-point experiment

The [preregistered protocol](../../PAIRING_20261024_PROTOCOL.md), commit
`dcd89832fc6169dd71dad048cc59a70f694a66bc`, preceded execution. All 942
nonsingular short models over p in {5,7,11,13,17,19} were tested. For each,
d is the least positive nonsquare and the monic twist is
`Y²=X³+d²aX+d³b`. Every odd prime divisor of the enumerated rational order
is retained, including **649 nonshared cases** with no nonzero twist r-torsion.
The F43 fixture is additional to this 942-model population.

For each of 274 shared-prime cases, P is the first nonzero rational cofactor
projection. Every rational twist point T is projected by [#E_d/r] and mapped
to S. Both native implementations verify the point equation, r-torsion,
Frobenius negation, direct and shifted Miller values, exact target order and
norm-one condition. The full census contains 4,616 auxiliary points.

| r | Shared cases | Auxiliary points | Zero cofactor images, value1 | Nonzero images, target order r |
| --- | ---: | ---: | ---: | ---: |
| 3 | 186 | 2,952 | 984 | 1,968 |
| 5 | 72 | 1,440 | 288 | 1,152 |
| 7 | 16 | 224 | 32 | 192 |
| Total | 274 | 4,616 | 1,304 | 3,312 |

These are exact counts in a complete finite population, not sampled estimates.
Repeated source points and images are counted as the protocol specifies; the
3,312 count is not a count of distinct pairing values or independent relations.
Every case attains all r target values when all auxiliary sources are included.

The C++17 producer uses the existing packed-field curve backend and binary
Miller chain. Rust independently enumerates coefficient pairs, computes inverses
by exponentiation, uses a linear Miller chain and independently checks Gaussian
rank. They share JSON/SHA and the IDC1 backend, not field or curve arithmetic.
Both full replay caps are 600 seconds; neither was hit. No elapsed-time sample,
IC/rho comparison, isolation tier or fastest-external-tool comparison is admitted.

## Controls and prerequisite

- All 1,304 zero images pair to one. All 3,312 nonzero images produce nontrivial
  order-r values in the norm-one target.
- All 274 same-source subgroup controls pair to one under a shifted disjoint
  divisor formula. The direct formula is **undefined in all 274 controls**;
  an undefined formula was never counted as the value one. The first legal
  offset in the complete lexicographic E(K) list is recorded and independently
  reproduced. Each evaluation rejects zero intermediate numerators/denominators.
- Valid wrong-subgroup points exist in 270 cases and fail the r-torsion
  precondition. Four cases have no such point and are explicit null controls.
  All points remain on the stated nonsingular curves.
- False pairing value, missing model, false summary count and false IC
  coefficient payloads are rejected, each with exit101 and no audit receipt.
  Exact payloads and diagnostics remain in the validation archive.
- The new norm-product identity has source-bound native certificate
  **IDC1h23c0696a12358b08**, with 32+16 replay points, an arithmetic false-plus-one
  counterexample and a rejected false Lean hash. It covers the polynomial only.

Shared odd r divides both N and Nt, with N+Nt=2(p+1), so p=-1 mod r and
the embedding degree is two. This restriction is checked in every shared case;
the earlier Lean arithmetic implications are retained. A quadratic twist does
not by itself give this small embedding degree for an arbitrary target subgroup.

## Formal status and limits

The two new modules contribute **19 named theorems**: 15 concrete fixture
statements and four conditional character/norm statements. The unchanged scoped
proof gate passes **68 named research theorems / 85 audited declarations**
(17 generated helpers), zero failing and zero extra trusted native axioms.
Only `propext`, `Classical.choice` and `Quot.sound` occur. Lean4.34.1 and
Mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612` remain pinned.
`lean4checker` is unavailable; no separate external replay is claimed.

`PairingTransfer.character_log_recovery` assumes an actual additive-to-
multiplicative homomorphism and a nontrivial image of a prime-order generator,
then proves unique exponent recovery. It does not silently discharge generic
Weil bilinearity or nondegeneracy. `PairingFixture.logarithm_transport` checks
the concrete Miller function directly, so that finite statement does not rely
on those general hypotheses being assumed.

| Requirement | Outcome |
| --- | --- |
| Positive twist-assisted ECDLP to field-DLP connection | Verified in the explicit F43 fixture, including every scalar |
| Auxiliary-point and degeneracy census | Independently verified for the complete registered finite population |
| General character implication | Kernel-checked with explicit homomorphism/nontriviality hypotheses |
| General geometric Weil pairing and uniform reduction | Still partial; not supplied by the finite table |
| Practical or asymptotic advantage | Not measured or established; no speedup/hardness claim |
| Frozen evidence/native-only execution | Preserved; no Python research execution |
| Mandatory library/native/workflow checks | 4,479 library tests pass, 119 existing ignores; 22 backend and four workflow tests pass |
| Shared Conductor | CLI exit127; no reservation or replacement coordinator |
| Merge | Held for conflicts, exact-head CI/access/review and the existing completion gates |

The remaining research obligations include general-model normalization,
exceptional S3/S4 lifting, generic geometric pairing correctness/nondegeneracy,
the uniform DLP reduction, and universal trace-zero geometry. A next pairing
experiment should preregister denominator-safe evaluation on non-k2 subgroup
families and measure destination-solver cost before any advantage claim. That
experiment is not running. See the [existing open problem record](../../../../problems/quadratic-twist-evidence-native-parity.md).

## Reproduction and sources

From the repository root, with C++17/OpenSSL, Rust1.99.1 and the pinned Lean:

```sh
bash research/quadratic_twist_evidence_20261021/run-pairing.sh /tmp/twist-pairing-fresh
tools/lean/proof-gate.sh --out /tmp/twist-pairing-proof research/quadratic_twist_evidence_20261021/lean
```

`run-pairing.sh` runs the complete native census and independent audit, exact
result/audit/certificate byte checks, corruption controls and backend tests.
The separate proof gate compiles and audits all seven modules.
[Proof map](../../lean/README.md); [C++ producer](../../native/pairing.cpp);
[Rust auditor](../../native/pairing-audit/main.rs); source/input/output hashes
and all unsuccessful builds are retained in the validation record. Historical
Python evidence, prior native results, certificates and earlier dated reports
remain unchanged. The study's active proof map and report index are updated.

The canonical IC/rho scoreboard, progress timeline, leaderboard, cover graph
and browser were checked for relevance and left unchanged: no admissible cost
point or generic transfer promotion is claimed. No registry or ledger row changed.
The three conflicted generated/claim files remain untouched pending the existing
Copilot request. Old inherited CI/service failures are not patched here.

Primary reference: Menezes, Okamoto and Vanstone, *Reducing elliptic curve
logarithms to logarithms in a finite field*, IEEE Transactions on Information
Theory39(5), 1993, [doi:10.1109/18.259647](https://doi.org/10.1109/18.259647).
The Crossref title/authors/year were verified; this citation supplies context,
not the proof receipt for our implementation. Actual formal status comes from
the linked Lean declarations and axiom audit. Diagram source is `bridge.dot`;
`plot.py` only lays out the recorded values in `coverage-data.json`.
