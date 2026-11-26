# Frozen protocol: single-target norm-one-torus m=7

Date: 2026-11-26
Lane: draft PR #5254
Status: frozen before target construction, implementation outcomes, or timing

## Question

For one public target on the production Prime31 curve, how cheaply can an exact
seven-term decomposition over the p+1 norm-one-torus Dickson factor base be
found when every retained table, normalization, lookup, and byte is charged to
that same target?

This is a generic point-decomposition component experiment. It is not a full
ECDLP recovery, a PKM/Amadori polynomial-system solver, or evidence of an
asymptotic improvement over generic discrete-log algorithms. [pcc:unverified]

## Literature boundary and prediction

Petit--Kosters--Messeng use a factor-base selector expressed as a composition
of low-degree rational maps, then solve the resulting Semaev system. Their
published p-1 and auxiliary-isogeny constructions do not supply a p+1 elliptic
group homomorphism or a seven-sum extractor. Amadori--Pintore--Sala reduce
relation collection to one factor-base dependency and report experiments only
for m in {3,4,5}; their m=5 split uses S3 and S4 with an intermediate variable.
Neither paper measures an exact generic m=7 meet-in-the-middle kernel.

Wagner's generalized-birthday filtering requires usable quotient information
at each merge. The prime-order elliptic subgroup in this fixture has no known
nontrivial efficiently visible small quotient, so this protocol does not apply
Wagner filtering. Repository reviews reaching the same boundary are retained
as prior evidence, not treated as a new theorem.

Primary sources:

- https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf
- https://iris.unitn.it/retrieve/handle/11572/202653/186850/Amadori%2C%20Pintore%2C%20Sala%20-%20On%20DLP%20for%20prime-field%20EC%20%28rev%29.pdf
- https://www.iacr.org/archive/crypto2002/24420288/24420288.pdf

The shared research KB and Conductor CLI are unavailable in this runtime. The
fallback review used the primary papers above, repository source, and the
existing p+1 and k-list reports. This unavailability is recorded in the session
log.

## Fixed instance and one target

- Curve: `fp_p31_subexp_mersenne_instance()` over `p = 2^31 - 1`.
- Factor base: production `dickson_factor_base(instance, 6, 0, 0)`.
- Cardinality contract: 34 abscissae and 68 signed points, with no fixed point
  under the recorded negation permutation. A mismatch aborts.
- Arity: seven, with repetition allowed by the solver.
- Target count: exactly one planted public target.
- Its seven distinct indices, with no inverse pair, are derived from the
  committed 256-bit master seed. The target is their affine sum.
- Every measured arm starts cold, constructs its own retained table, scans the
  complete streamed side even after finding a witness, and then discards all
  state. No table or target-dependent cache is shared across arms or rounds.

The public manifest is committed and pushed before the seed is expanded into
indices or the target point. No outcome-dependent seed replacement is allowed.

## Exact algorithms

The complete unordered signed three- and four-sum multisets contain
`C(70,3) = 54,740` and `C(71,4) = 971,635` states. Elliptic negation acts on
the retained side. The odd three-sum action has no fixed multiset, giving
27,370 quotient states. The four-sum action has `C(35,2) = 595` fixed
multisets, giving `(971,635 + 595)/2 = 486,115` quotient states.

Four cold arms are fixed:

1. `retain4-hash-sum`: retain the canonical four-sum quotient in a Rust hash
   map keyed by `(x,infinity)`; construct three-sums and then subtract each
   complete sum from the target.
2. `retain4-hash-centered`: use the same retained hash table, but enumerate
   complements directly as `Q-P_i-P_j-P_k`.
3. `retain4-prefix16-centered`: use the centered query and an exact static
   prefix directory. A 65,537-entry offset array selects the bucket given by
   the high 16 bits of a finite 31-bit x-coordinate; compact entries retain
   full x and y coordinates and a packed four-index witness. The selected
   bucket is scanned with full x equality, so the prefix is only a directory,
   never a probabilistic filter.
4. `retain3-prefix16-centered`: retain the canonical three-sum quotient in the
   same exact directory and scan `Q-P_i-P_j-P_k-P_l`. This is the lower-memory
   frontier, not an uncharged preprocessing arm.

All projective states are normalized by Montgomery batch inversion. Query
states are normalized in fixed blocks of 2,048. The retained table is built and
normalized inside every cold arm.

Before exceptional branches, the fixed mixed-addition call counts predict:

| Arm or phase | Setup additions | Query additions | Cold total |
| --- | ---: | ---: | ---: |
| retained-4 setup | 543,201 | - | 543,201 |
| retain4 sum query | - | 111,826 | 655,027 |
| retain4 centered query | - | 57,154 | 600,355 |
| retained-3 setup | 29,716 | - | 29,716 |
| retain3 centered query | - | 1,028,789 | 1,058,505 |

These are preregistered structural predictions, not measured results. The
centered recurrence should remove 54,672 query additions from retained-4 and
971,567 from a sum-then-subtract retained-3 query. The prefix directory may
reduce allocation, hashing, and random-access overhead, but it does not reduce
the number of group sums.

## Inputs, repetitions, and endpoints

The manifest fixes one seed, one target, query block size 2,048, prefix width
16, and nine counterbalanced cold rounds. Repetitions rerun the same single
target from scratch; they are timing repeats, not target batching.

Primary deterministic endpoints:

- retained states, distinct x keys, candidate count, directory comparisons,
  and full hit/miss result;
- complete setup/query vectors of additions, doublings, multiplications,
  squarings, and inversions;
- exact compact payload bytes and directory bytes, plus Rust container
  capacities where applicable;
- replay of the returned seven-index witness.

Secondary diagnostics are non-isolated local cold wall times. No timing ratio
is promoted as an admitted speed claim without the repository isolation route.
That route is unavailable here, so exact operation and memory counts decide the
component result.

## Decision rule

- Centering succeeds structurally only if it returns the same result and
  witness-validity decision while matching the preregistered reduction in
  charged query additions with no hidden target setup.
- The prefix directory succeeds structurally if it is exact, independently
  replayed, and reduces charged table payload/metadata or deterministic lookup
  work. Local wall time remains diagnostic.
- The retained-4/retained-3 comparison reports both cold total work and memory;
  it may not call the retained table free or reuse it for another target.
- A loss, miss, collision, exceptional branch, or prediction mismatch is
  retained and reported.

## Correctness and proof boundary

A separate Rust verifier will rebuild the production factor base and derive the
one target from the committed seed. Using affine group arithmetic, it will
replay the disclosed seven-index witness and independently enumerate an exact
three-versus-four decomposition decision. It will also check all source and
artifact digests.

The precommitted seed, hashes, full-scan counters, public witness, and
independent affine replay are transparent correctness and reproducibility
evidence. They are not a zero-knowledge proof, do not prove unbiased seed
generation, and do not attest which host produced a timing. The witness is
intentionally public, so hiding it would work against independent replay. No ZK
or producer-attestation claim will be made in this lane. [pcc:unverified]

