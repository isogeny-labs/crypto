# H116 chiral marked-halving Frobenius-orbit parity protocol

Protocol ID: `H116-CHIRAL-MARKED-HALVING-FROBENIUS-ORBIT-PARITY-001`

Filed after marked rational-halving existence was measured null and before
evaluating any complete halving-fiber orbit predicate against scalar parity.
[pcc:unverified]

## Related-work and novelty boundary

Division by two, two-isogeny descent and Frobenius actions on torsion are
standard.  Bekker--Zarhin, *Division by 2 of rational points on elliptic
curves*, St. Petersburg Math. J. 29 (2018), 683--713,
<https://doi.org/10.1090/spmj/1512>, gives explicit halving criteria.  The
use of Frobenius cycle types to describe finite étale fibers is classical.
Targeted repository, Crossref, arXiv, ePrint and journal searches found no
generic prime-field ECDLP scalar-parity decoder from these target-tethered
marked halving-fiber cycle types.  This is a prior-art screen, not a novelty
proof. [pcc:unverified]

The preceding panel retained only whether Frobenius fixed a point in each
four-point fiber `[2]^{-1}(T)`.  This experiment retains its complete
permutation type, distinguishing the non-rational `2^2` and `4` cases and the
partially rational `1^2 2` case.

## Frozen observable and rules

Use the same four marked branch-pair quartics and both the source and dual
kernel point, giving eight ordered four-point halving fibers.  For each smooth
fiber, classify Frobenius as one of

```text
0: 1^4     four rational halves,
1: 1^2 2   two rational halves and one conjugate pair,
2: 2^2     two conjugate pairs,
3: 4       one orbit of length four.
```

Expand each type to three Boolean predicates in this order:

```text
has_fixed_point = type in {0,1},
all_rational    = type == 0,
has_four_cycle  = type == 3.
```

The 24-predicate order is source then dual for model 0, source then dual for
model 1, and so on, with the three predicates above adjacent within a fiber.
A selected predicate abstains if its branch-pair quartic is singular.

Test every single predicate and every xor of two distinct predicates,
optionally complemented.  This gives
`2*(24+binom(24,2))=600` fixed Boolean rules predicting canonical scalar
parity on their covered subset of the 1,520 frozen nonidentity targets.

## Frozen controls

Retain every inherited branch, resultant, binary-quartic, genus-one,
resolvent, two-isogeny, exhaustive base-field doubling and point-count
control.  For each admitted source and dual fiber:

1. construct a deterministic degree-four extension of `F_p`;
2. obtain all four halves by two successive square roots and verify they are
   distinct and double to the marked kernel point;
3. apply coordinatewise `p`-power Frobenius, require a permutation of the
   four points, and read its cycle partition;
4. independently derive the type from the base-field characters of `B` and
   `A^2-4B` plus the rational-point census when `B` is a square;
5. require the extension permutation type and character formula to agree.

Report the four-type histogram, admitted predicate census, and target versus
negation distinction count.  Require a 1,520/1,520 planted scorer and
byte-identical replay.

## Calibration and conditional algorithm

Rank by `(2*M-C)^2/C` when `2*M>C`.  In each of 4,096 deterministic
within-curve parity shuffles, reselect across all 600 rules.  Retain total and
covered exact counts, all controls, and per-curve scores.

A total exact rule with a generic proof would give the same public binary
recursion

```text
d=k mod 2,
Q'=[2^(-1) mod r](Q-[d]P)=[floor(k/2)]P.
```

The fixed branch construction and bounded torsion-fiber factorization cost
polynomial time in `log p`; the degree-four field is constant degree.  Thus a
proved generic rule would be polynomial-time and hence subexponential.  A
finite total exact fit is only a candidate.  A null closes only this frozen
24-predicate single/pair-xor family. [pcc:unverified]
