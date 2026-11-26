# Oriented four-leaf controls: lower degree, no full-base completion

**2026-11-24 · Local development evidence · [pcc:unverified]**

A new four-leaf composition of Isogeny Labs' oriented-pair S3 identity now
uses the unchanged native F4/root visitor and independent signed replay.
Both exact-base dense membership and quadratic Dickson chains were tested
on all thirteen existing targets. **All 26 full-base calls stopped at the
fixed matrix-allocation limit without an extracted assignment.** The source
component is implemented; the full-base complete-output gate remains open.

![The two membership models feed the same bounded native traversal.](oriented-chain.pdf){width=70%}

## Source and relation scope

The [original pair example](../prime31_ic_20261115/native/examples/prime31_s3_oriented_pair.rs)
is a construction/F4 diagnostic, not a complete four-leaf extraction API.
Its [report](../prime31_ic_20261115/native/ORIENTED_PAIR_S3_F4_20261115.md) already
retains unsuccessful fixed-pair and target-conditioned m8 trials. This new
adapter composes its classical chord/tangent identity; it does not attribute
a complete four-leaf solver or a new complexity theorem to the original work.

For a pair with abscissae x,z, first ordinate y, slope t and sum abscissa w,
the equations are

\[
y^2=x^3+ax+b,\quad t^2=w+x+z,\quad
2yt+(z-x)w=2x^2+xz+a.
\]

They recover the second ordinate y+t(z-x) and pair-sum ordinate t(x-w)-y.
Three further equations place both pair sums and the negative public target
on one line, with the cubic root-sum condition. The
[frozen protocol](ORIENTED_CHAIN.md) gives the composition and affine exceptions.
The intended output is the full signed distinct-x, target-disjoint relation
set used by the cofactor control, rather than only the regular-image subset.

Dense membership restricts each leaf to the actual liftable base polynomial.
The tower uses the original Dickson fibre; the oriented equations enforce
rational curve lifting. Every extracted assignment must pass independent
curve, pair-sum, total-sum and signed-base checks before chart filtering and
deduplication. Full-field and supplied-domain fallback branches are refused.
Native algebraic-root branching alone implies no subenumerative complexity.
[pcc:unverified]

\newpage

## Frozen outcomes

Source **21a7ef9221c38af6328a8bac45a2ce05f82ad973** was published before calls.
The copied executable's SHA-256 is
`b6c293e9af1bb71d9db3a25926de6717d08724edd8166490600350b0670507e4`.
Each query shared one three-second deadline across construction, native
traversal and signed extraction. The native limits were degree 16, 20,000
construction terms, 4,096 assignments, 20,000 pairs, 8,192 symbolic monomials
and 2,000,000 matrix entries. Each panel had one worker, 2 GiB address space,
a 60-second outer limit and five-second kill grace. No retry or cap increase.

| Field / selector degree | Membership | Variables / equations | Input degree | Complete |
|---|---|---:|---:|---:|
| F127 / 16 | Dense | 11 / 13 | 8 | 0 / 4 |
| F127 / 16 | Tower | 23 / 25 | 3 | 0 / 4 |
| F8191 / 16 | Dense | 11 / 13 | 7 | 0 / 4 |
| F8191 / 16 | Tower | 23 / 25 | 3 | 0 / 4 |
| F8191 / 32 | Dense | 11 / 13 | 14 | 0 / 5 |
| F8191 / 32 | Tower | 27 / 29 | 3 | 0 / 5 |

Every dense call refused a matrix admission of **2,006,513** entries; every
tower call refused **2,001,665**, against **2,000,000**. All 26 records report
zero assignments and zero forbidden domain evaluations. All three processes
exited zero after logging `undetermined` outcomes. A zero exit is not an
exact-query result. These repeated early refusals do not establish behavior
beyond the stated caps or a general mathematical obstruction. [pcc:unverified]

[F127 records](evidence/oriented_v1/f127_d4.jsonl) ·
[F8191 degree-16 records](evidence/oriented_v1/f8191_d4.jsonl) ·
[F8191 degree-32 records](evidence/oriented_v1/f8191_d5.jsonl) ·
[Pre-call identity](evidence/oriented_v1/pre_call.txt) ·
[Commands](evidence/oriented_v1/commands.txt) ·
[Dependency inventory](evidence/oriented_v1/production_source_manifest.json)

Degree 16 was chosen before calls to admit the dense degree-14 input. It
does not change the prior cofactor protocol's degree-10 limit. The two new
arms have different algebraic-closure ideals and variable counts. Their
rejected matrix sizes cannot be treated as a complete-cost comparison, and
neither arm supplies an accepted full-base relation set.

## Validation and next discriminator

The final feature suite passes **18 tests** and the default suite **6**.
Both membership models accept and replay all **312 distinct ordered
witnesses** for the existing 13-row F127 target. A separate planted fixed-slot
control completes through the actual native visitor and matches all expected
sign choices. It does not replace any failed full-base call. The initial
converter compilation failure is retained. [pcc:unverified]

The ordered-witness count also explains why this interface uses recursive
native extraction: one F127 parameter cannot distinguish 312 rational
assignments. A single-coordinate RUR backend cannot represent that whole
labelled output in the usual separating-coordinate form. This is a
representation issue, not a barrier to index calculus. [pcc:unverified]

Claims `PCC-20261124-e3fa5b`, `PCC-20261124-d38ee6`,
`PCC-20261124-55d999`, `PCC-20261124-fd921e` and
`PCC-20261124-673137` retain unsigned standing. See
[test logs](evidence/oriented_v1/adapter_tests.txt),
[reproduction](ORIENTED_REPRODUCE.md), and
[development limits](DEVELOPMENT_CHECKS.md). Root-suite compilation and
independent signed replay are not represented as passed.

The lower-degree tower adds variables and does not resolve the native
matrix limit. Further cap increases need a new discriminating hypothesis.
The next work is common cold-recovery accounting on the already qualified
degree-16 cofactor/Refined fixtures, including all failed calls and differing
relation scopes. Complete oriented output and degree-32 cofactor output remain
open. No admitted cost result, EKS job or leaderboard update follows.
The local stream holds 241 records; external delivery still requires the
receiving HTTPS URL. [pcc:unverified]
