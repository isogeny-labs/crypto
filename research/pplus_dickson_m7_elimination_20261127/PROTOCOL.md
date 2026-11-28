# Cold p+1 Dickson-tower m=7 elimination protocol

Date frozen: 2026-11-27 UTC. Branch:
`codex/pplus-dickson-pair-spectrum-20261126`. Draft PR: #5259.

## Question

Can target-specialized PKM/Semaev elimination over the production norm-one
Dickson tower find and recover an exact seven-point decomposition with less
cold single-target work than the archived streamed 3+4 control, without
enumerating the seven-leaf grid or sharing work across targets?

This tests the algebraic alternative requested after the generic m=7 kernel.
It does not test another factor-base family. The production cell uses the
same curve, depth-six Dickson fibre, 34 rational abscissae, 68 signed points,
public planted target, and manifest seed as
`research/norm_one_torus_m7_single_target_20261126`.

## Prior work and boundary

The system follows Petit–Kosters–Messeng, PKC 2016, §§3.1–3.4: factor-base
membership is represented by a composition of low-degree maps and coupled to
Semaev summation equations. Here each leaf has the production p+1 tower

```text
y_(i,j)^2 = y_(i,j+1) + 2,
y_(i,5)^2 = 2,
x_i = y_(i,0),
```

and the seven leaves are joined to the public target by six chained S3
equations. The exact rational-tree/sign recovery obligations from the
oriented Dickson-S3 work remain in force.

Existing repository evidence fixes three negative boundaries:

1. `DICKSON_PAIR_NORM_REVIEW.md` gives the exact leaf-to-pair double-resultant
   and an F7 regular-fibre counterexample to automatic D2 folding of the
   elliptic pair support.
2. `NO_SINGLE_COVER_FFT.md` shows that torus shifts do not automatically lift
   to one rational elliptic-lift cover.
3. `problems/prime31-exact-pair-spectrum-query.md` requires any claimed
   subquadratic query to charge setup, misses, signs, collisions, recovery,
   certificates, and output. A materialized quadratic pair table is the
   baseline, not a breakthrough.

The research-KB wrapper was unavailable (exit 127). These pinned repository
records and their primary PKM, Semaev, and special-resultant sources are the
fallback literature receipt. No novelty priority is claimed.

## Candidate and implementation gate

Use `PkmSemaevOracle` in `mode=chain`, `engine=f4`, `membership=Dickson`, with
descent extraction enabled and exhaustive fallback disabled for measured m=7
cells. Instantiate a fresh oracle for every target. Trace reuse, learned
bases, shared lifting, warm target batches, and cross-target preprocessing are
forbidden.

A seven-leaf left chain needs five free intermediate coordinates. The current
`f4_fp_tower` key stores four free exponents in bits 64..96 and degree above
bit 96. The implementation may move the degree field upward and support five
free exponent bytes, but must preserve monomial order and pass the existing
F4 suite. This capacity change is plumbing, not a speed result.

For every attempted target record:

- all system construction and target specialization work;
- F4 multiply-adds, generated rows/columns, solving degree, stopping reason,
  CPU/wall and peak resident memory;
- descent/root extraction, sign recovery, curve/group replay, and all failed
  branches;
- whether any grid enumeration or fallback occurred.

Any enumerative fallback makes that row a control/failure, not a successful
algebraic candidate.

## Correctness ladder

Before the production cell:

1. Run deterministic public toy curves at Dickson depths one and two. Compare
   every F4 hit/miss and recovered signed witness with a separately written
   exhaustive affine seven-tuple oracle. Include planted hits, empty targets,
   repeated leaves, inverse pairs, and a finite-chart exclusion.
2. Check monomial encode/decode, divisibility, multiplication, lcm and degree
   against a tuple reference with five free variables, while retaining the
   existing randomized four-variable tests.
3. At depth three, run at least one planted and one empty public target with a
   fixed 60-second and 2-GiB per-target cap. A timeout is unresolved, never an
   empty certificate.
4. Only if those checks pass, run the single production Prime31 planted target
   from the archived public manifest. Verify the returned seven indices by an
   independent affine replay and compare its truth value with the archived
   3+4 verifier.

## Cost gate and falsifier

The archived exact-x-first m=7 query uses 971,635 candidates and records
4,613,665 multiplications, 1,254,991 squarings, 476 inversions, and 1,028,789
additions, with its same-target retained-side setup charged separately. The
new arm must report its own setup and query separately and together; no cost
may be assigned to another target.

Promotion requires all of the following:

- exact truth/witness agreement on every finite control and the production
  target;
- zero grid fallback on the promoted rows;
- a complete operation-accounting map that does not compare F4 multiply-adds
  directly with group additions without a declared conversion;
- at least a 4x reduction in the frozen field-equivalent cold total under
  every declared conversion in the sensitivity panel, plus no increase in
  peak memory beyond the archived retained 3+4 arm.

The predeclared field-equivalent sensitivity panel prices a field square at
0.8, 1.0, and 1.0 multiplication; an inversion at 50, 100, and 200
multiplications; and a multiply-add at 1 and 2 multiplications. Both the
candidate and control include setup, extraction, and verification. Local wall
time is diagnostic only.

The algebraic lead is falsified for this ladder if any of these occurs:

- incorrect truth or witness replay;
- required grid fallback at depths one through three;
- two consecutive completed depths have nondecreasing candidate/control cold
  work ratios and the larger ratio is at least one;
- the depth-three or production row reaches its fixed resource cap before a
  decision.

A falsified row is retained with its stopping reason and exact partial work.
It does not prove all Dickson/Semaev eliminations impossible.

## Pair-spectrum contingency

If the algebraic lead is falsified, a pair-spectrum follow-up is admitted only
after a separate frozen amendment supplies an exact algorithm with stated
intermediate dimensions and an asymptotic target query below the explicit
quadratic pair scan. A polynomial norm/resultant alone, an O(B^2) output, a
precomputed pair hash table, or setup charged to another target does not meet
that gate.

## Evidence and proof boundary

Rust source, manifests, raw JSONL, hashes, and verifier output will be archived
before any performance claim. Public seeds make the workload replayable; they
are not zero-knowledge proofs. A result may be described as independently
replayed only after a separate verifier reconstructs the factor base and
witness from public coordinates without reading internal solver state. No
general polynomial identity, asymptotic runtime, or ECDLP speedup is proved by
the finite run.
