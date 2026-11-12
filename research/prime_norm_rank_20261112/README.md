# Prime-field norm/rank search audit

No novel sub-exponential ECDLP algorithm was obtained. Read [REPORT.md](REPORT.md),
[DERIVATION.md](DERIVATION.md), or the [three-page PDF](report/report.pdf).
The classical norm representation isolates an unresolved constructive rank-one
search. A Lean counterexample rejects its unrestricted linear relaxation
[pcc:unverified].

## Reproduce the formal checks

With Lean4.35.0 and the pinned lake-manifest dependencies available:

```sh
lake -d research/prime_norm_rank_20261112/lean build
lake -d research/prime_norm_rank_20261112/lean env leanchecker --fresh --verbose NormRank
```

The existing identity issuer is included unchanged by a small standalone Rust
adapter. It performs certificate checks, not a discrete-log experiment:

```sh
cargo run --release --locked --manifest-path research/prime_norm_rank_20261112/certificate/Cargo.toml -- research/prime_norm_rank_20261112/lean/NormRank.lean /tmp/norm-defect.identity.json
```

Original logs retain failed attempts alongside successful receipts. These are
unsigned producer evidence; certificate checking does not itself execute Lean.
No empirical scaling, new algorithm or independent signed standing is claimed.

The prior nonscalar-origin and rank-directed jet-completion suggestions remain
unproved research proposals. Preserving interpolation node values does not by
itself establish a useful crossing-rank bound or a sub-exponential total cost.
This separate norm audit does not establish either suggestion.

## Structural extraction addition, November12 evening

[RankOneExtraction.lean](lean/RankOneExtraction.lean) checks the postprocessing
step for a supplied matrix. Let K be any field and M an entry function I×I→K.
Assume symmetry, at least one nonzero entry, and

    M(i,j) M(k,l) = M(i,l) M(k,j)  for all i,j,k,l.

These are explicit vanishing-minor assumptions. A diagonal pivot k exists with
M(k,k) nonzero. Set w(i)=M(i,k) and s=1/M(k,k). The proof gives

    M(i,j) = s w(i) w(j),   s != 0,   w(k) != 0.

No square root, quadratic-residue test, positivity assumption or extension field
is needed. In particular, the scalar s need not be a square [pcc:unverified].

For a finite index set, if M annihilates the branch vector ell, then w satisfies
sum_j w(j)ell(j)=0. For any linear map L on the matrix-entry space, a supplied
norm coefficient equation L(M)=c*g transfers to

    L(w w^T) = (c/s)*g.

If c is nonzero then c/s is nonzero. This is the structural bridge used by the
written norm reduction; its coefficient map is abstract in the new theorem.
Instantiating that map with the polynomial norm and proving the complete
curve/divisor equivalence in Lean remain separate obligations [pcc:unverified].

| Statement | Declaration | Hypotheses |
|---|---|---|
| Nonzero diagonal pivot | RankOneExtraction.diagonalPivot | Field, symmetry, nonzero entry, all minors vanish |
| Explicit scaled column factor | RankOneExtraction.pivotFactor | Field, symmetry, all minors vanish, supplied nonzero diagonal pivot |
| Nonzero vector and scale exist | RankOneExtraction.factorExists | Same assumptions as diagonalPivot |
| Branch constraint transfer | RankOneExtraction.extractBranch | Field, finite index set, symmetry, M*ell=0 |
| Free norm scalar stays nonzero | RankOneExtraction.absorbScale | Nonzero field elements c and s |
| Linear coefficient equation transfers | RankOneExtraction.extractLinearImage | Field, vector space, supplied factorization with nonzero scale, linear image equation |

The statements use the vanishing-minor equations directly; this addition does
not formalize an equivalence with Mathlib's numerical matrix-rank definition.
It constructs the factor once those equations hold, rather than assuming a
rank-one oracle [pcc:unverified].

On a finite d×d matrix supplied explicitly over F_p, one can scan the diagonal,
invert the pivot and copy its column. This takes O(d) field operations after
obtaining M. Reading a dense input costs O(d²) field elements; even checking all minors directly takes polynomial work. These
operation counts are a written analysis, not a Lean time theorem or a timing
measurement. They do not bound the work needed to find an admissible M
[pcc:unverified].

The expanded default build passed3141 jobs. The actual tracked-source gate
passed21 declarations with0 additional trusted axioms. All six new declarations
print only propext, Classical.choice and Quot.sound. The import-inclusive fresh
replay receipt is in extraction-evidence/fresh-status.json when finished.
Original source and earlier receipts remain unchanged [pcc:unverified].

Five structural statements have canonical unsigned PCC bindings in
[extraction-evidence/claim-pins.json](extraction-evidence/claim-pins.json).
The nonzero-division helper is included in the source audit but is not promoted
as a separate research claim. All standing is unsigned producer evidence
[pcc:unverified].

## What an existence proof would still require

For the user's classical general prime-field objective, a positive result must
supply a uniform executable algorithm, its correctness on the specified curve
and subgroup family, and a2^{o(n)} bit bound including sampling, failed attempts,
memory access and witness recovery. A conditional implication that assumes such
a search routine does not establish its existence. A nonexistence result would
have to quantify over the stated class of classical coordinate-using algorithms;
a generic-group bound or a restricted XL lower bound does not do that.

The newly merged [prime-field IC closure note](../notes/index-calculus/PRIME_FIELD_IC_CLOSURE_20261112.md)
explicitly leaves F4 and growing towers outside its established bounds and
identifies unproved load-bearing hypotheses. It is prior research context, not
a universal nonexistence certificate. Neither that note nor this extraction
addition decides the full existence question [pcc:unverified].

Sources checked for this addition: pinned Mathlib
LinearAlgebra/Matrix/Rank.lean (rank conventions), the earlier Galbraith norm
references in sources.json, and the newly merged closure note. KB remained
unavailable (no MCP tools and wrapper reported no executable); no novelty is
claimed for elementary field linear algebra.
