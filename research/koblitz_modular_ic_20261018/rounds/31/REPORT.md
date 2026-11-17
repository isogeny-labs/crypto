# R31: split-kernel point passage and larger character carriers

Observations and exact computations, without an IC efficiency or attack verdict.
The persistent research goal remains active. R30 was already merged as PR2834;
this is a separate continuation from main17921f950.

## Actual ordinary supports reach the incidence equations

COLOR31 completes the two outstanding ordinary finite-field calculations.
Each has one compatible middle and both complete 49-level transports pass.
The first/second kernel colors (powers of doubling modulo sign) are (4,0)
over F2^284 and (3,0) over F2^83. Every common-j and parameter polynomial has
its full degree accounted for by distinct rational roots. The second cyclic11
kernel therefore has five generator abscissas in the retained field.

REVERSE31 uses those literal accepted coefficients. It converts each of the
five generators to Tate parameters and transforms the selected cyclic7 kernel.
For every generator and both descent forms it checks the SAME five original
R27 source equations and all eight registered units on the entire quotient
algebra. No exceptional branch is removed by an unchecked division. An
independent group calculation gives exact order11 for (0,0). Ten diamond
iterates remain on the source; the fifth is the sign involution and the tenth
is the identity. A perturbed nu is rejected. This reuses the existing R16
diamond twist; that twist and the Tate model are not new constructions.

For degree83, each untwisted orientation quadratic is irreducible and each
diamond-twisted quadratic splits. At degree284 both forms are irreducible.
The twisted source coordinates are descent coordinates: their physical
identification with the original source uses the auxiliary T²+T+1=0.
Thus a rational point on the twist need not be rational on the original curve.

## Closed-point projection, not yet Picard evaluation

FROB31 factors every orientation quadratic completely. On every irreducible
factor it computes the least absolute Frobenius period of the complete source
coordinate tuple and partitions the ten diamond translates into closed points.
It applies the actual integral operator

    E=(1-T^5)(5-sum(i=0..4)T^(2i))

to those closed points. Its output records the signed weights, not merely a
scalar or a generator-field label. In particular, the original degree83 lane
has source degree166 and five closed diamond classes, with E[P]=0 as a
formal divisor. The degree83 diamond twist has source degree83 and ten closed
diamond classes; its ten signed coefficients remain nonzero. Those are actual
candidate formal divisors for the next Picard calculation. Nonzero formal
divisor does not imply nonprincipal class or nonzero modular-factor image.
The generator labels and split orientation choices give repeated candidates,
not ten independent divisor classes. Both degree284 forms have source
degree568, five closed diamond classes and zero formal projection.

## Larger one-prime character data

CM31 reuses R15's construction, not a newly discovered family. The old
n5,11,83,131 cases are regression controls. The frozen bounded conductor scan
also supplies n163,233,283,409,571. Its exact integer Euler polynomial is
certified by a full conjugate-root-label multiset bijection; n5/n11 additionally
have independent dense CM-ring expansion controls. A2 alone is not the check.

| n | conductor prime p | level 7p | character-cover genus | untwisted E1/E0 multiplicities |
|---:|---:|---:|---:|---|
| 5 | 11 | 77 | 61 | 1/1 |
| 11 | 23 | 161 | 309 | 0/2 |
| 83 | 499 | 3493 | 54781 | 1/1 |
| 131 | 263 | 1841 | 45589 | 2/0 |
| 163 | 7499 | 52493 | 1629349 | 1/1 |
| 233 | 9787 | 68509 | 3039253 | 1/1 |
| 283 | 11887 | 83209 | 4483853 | 0/2 |
| 409 | 8999 | 62993 | 4906365 | 0/2 |
| 571 | 33119 | 231833 | 25213077 | 2/0 |

The diamond twist interchanges the two multiplicities. "First" refers only
to the frozen odd k<=99, p<=100000 scan, not a globally minimal conductor.
The native program checks the complete base permutation and fiber cocycles,
S² and (ST)³, connectedness by a retained Bezout chain, and every lifted
cycle count. For controls below the unchanged ten-million-state cap it also
materializes the entire permutation. Above that cap it uses exact cyclic
translation periods on the base states, not a larger materialization budget.
The recorded cycle histograms and deterministic source are retained; contrary
to the prospective wording, the full base permutation arrays themselves were
not serialized. They can be reconstructed from that source, but this is a
certificate-serialization gap, not an independent replay certificate.

The interpretation as modular character covers and their good-reduction
Jacobian factors still uses the written modularity/moduli bridge. There are
no explicit equations or evaluated subgroup injections at these larger sizes.
The genera are recorded exact values, not a claim of practical IC complexity.

## Scope and references

Primary sources checked before choosing the continuation: Molin–Page ANTSXV
Theorem37 (weight/level/nebentype), Schuett math/0511228 Theorem1.4, Milne
Jacobian Varieties12.1 (Torelli), and Stacks53.12 (tame Riemann–Hurwitz).
Zarhin2109.06794 is complex-case motivation only; no characteristic2 theorem
is inferred from it. Internal dedup: R15 CM.md/cmray15.rs, R16 carrier16.rs,
R27 LAMBDA14.sing and R30 REPORT/LEADS/kernel code.

- https://antsmath.org/ANTSXV/papers/ANTS-XV_molin-page.pdf
- https://arxiv.org/html/math/0511228
- https://www.jmilne.org/math/xnotes/JVs.pdf
- https://stacks.math.columbia.edu/tag/0C1B
- https://arxiv.org/html/2109.06794

No change to earlier failed sources or canonical archives, no EKS/Diem/freeze
restart, no fifth Conductor claim, and no merge or auto-merge. The shared
session log was reserved by another task; SESSION.md holds the scoped handoff.

## Final local verification

Seven native attempts accepted, zero native failures or timeouts. Lean compile
and transitive axiom audit both exit0: ten named/audited theorems, zero failing,
zero trusted-native. The unused trailing ring tactic is only a linter warning.
Bash syntax, metadata-assembler byte compilation and final Rust formatting
checks pass. Full root-library CI remains a separate, uncertified scope.
