# Exact original Gröbner-system checks

All18registered native calls complete: two calibration calls and eight
source/control systems, each twice. All16msolve outputs match an independent
native Buchberger computation, S-pair/input-remainder checks, and all eight
repeats have identical GB bytes. The frozen analysis and canonical export
pass full file checks. This is unsigned same-host evidence. [pcc:unverified]

The inputs are a complete-intersection control; the actual F25 Edwards
counterexample, its highest parts and its literal homogenization; and both
F101³ worked examples printed on page7 of the Joux–Vitse author manuscript,
affine and homogenized. These are known in-sample fixtures, not a generic
random population or a full m,n sweep. [pcc:unverified]

| Fixture | Returned GB degree | F4 selected-pair degree | Finite affine length | Rational zeros |
|:--|--:|--:|--:|--:|
| x²,y² control | 2 | no round | 4 | 1 |
| Edwards affine | 2 | 2 | 2 | 1 |
| Edwards highest parts | 2 | 2 | infinite | 9 |
| Edwards homogenized | 2 | 2 | infinite | 13 |
| JV empty, affine | 0 | 3 | 0 | 0 |
| JV empty, homogenized | 4 | 5 | 8 | 1 |
| JV split, affine | 1 | 3 | 1 | 1 |
| JV split, homogenized | 3 | 4 | infinite | 101 |

The rows above are the first pass; the repeat gives the same exact outputs.
For homogeneous systems, the listed rational zeros count affine cone points,
including the origin. They are not projective point counts. “Infinite”
refers to a positive-dimensional coordinate algebra over an algebraic closure,
not an infinite set over a finite field. The empty affine ideal is the unit
ideal, with quotient length0. [pcc:unverified]

## The degree-five diagnostic

The JV empty homogenized input consists of three quadratics, with stated
Macaulay bound4. The returned basis has maximal degree4. Its raw F4 log
also records a degree5 selected-pair round, with a5×3matrix, zero new
polynomials and two zero reductions. Those facts must remain distinct.
This does not establish that a nonzero new basis polynomial exceeded the
bound, or audit every internal polynomial in the source's intended
Assumption2 computation. No generic/JV refutation is claimed from it.
[pcc:unverified]

The independent Buchberger implementation reaches degree6 on both JV
homogenized cases. It has a different pair schedule and reduction strategy.
Its peak is not silently substituted for an F4 statistic. The exact initial
monomial ideal Hilbert functions are reported through degree12; their finite
prefixes are not treated as stabilization proofs. [pcc:unverified]

## Source and semantic checks

The native checker reconstructs the exact source coefficients in F25 and
F101[t]/(t³+t+1), checks the cubic's irreducibility by its absence of roots,
checks the actual JV targets and nonzero discriminant, and verifies the
split example's actual factor-base point sum. It binds the reconstructed
coefficients to the literal input files and checks every homogenization.
Original target scalar decompositions and the source's full curve order
are not claimed as checked. [pcc:unverified]

The positive calibration distinguishes dual-number length2 from simple
length1. The deliberately wrong y−1→y−2 constant is rejected by the same
remainder code. The solver control x²,y² has length4/support1. No controls
or source outcomes were taken before the official freeze. [pcc:unverified]

The [separate Lean55 project](../edwards_descent_audit_20261113/README.md)
proves the actual Edwards input/sum, literal source symmetrization and
coordinate descent, nonzero quadratic equations, actual dual-number quotient
equivalence and failure of highest-part regularity. Exact local
PCC-20261113-bc105f replay passes; signed standing remains unverified.
Native execution authenticity, Joux–Vitse source-field descent and its
complete computation-degree bound have no Lean refinement here.
[pcc:unverified]

## Protocol and reproduction

Protocol787b3fdd68e377e35974c358067bc2faa3f8481ea5cf3ea1dcb231490969db3e
was frozen against source checkpoint3b23168414f8f5a94767c75820da309072ebe306
and committed8be3fae3 before data at01:43UTC. The rejected predata flag-type
draft and both failed native build sources/logs/exits are retained. The
instrument, fixtures, analysis and canonical export are unchanged after data.
All scientific clocks are invalid; paid compute and agent storage uploads0.
[pcc:unverified]

```sh
cd research/groebner_system_audit_20261113
sha256sum -c inputs.sha256
python3 /workspace/crypto-autoresearcher-current/tools/experiment_protocol.py \
  check-export export.json --protocol protocol.json --verify-files
```

The manifest includes local executable/runtime paths whose exact bytes are
required for that in-place check. Rebuild/reproduce under a new output
directory/protocol; `run.sh` deliberately refuses an existing canonical
cohort. Do not rerun or overwrite these records. Verifier-named raw paths
remain loose under the repository13aexception. Any subsequent repository
artifact-bot manifests must be hydrated before file verification.
[pcc:unverified]

Primary read scopes/hashes and the unavailable-KB fallback are recorded in
[the source review](../edwards_descent_audit_20261113/evidence/source-review.json).
FGHR generic/n>=3 variants and larger specific-system sweeps, general
ideal-degree laws and qualified Joux–Vitse complexity remain open in
[the original fourteen-item tracker](../../problems/fourteen-target-audit-proof-and-execution-gaps.md).
No novelty, independent-host or compiled-refinement claim. [pcc:unverified]
