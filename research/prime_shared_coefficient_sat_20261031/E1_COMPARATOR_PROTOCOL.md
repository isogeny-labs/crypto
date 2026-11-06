# E1 external algebra-system comparator protocol

Status: **frozen before msolve or cvc5 saw a study instance**. This protocol
covers a correctness-only msolve base-field cross-check and records why cvc5
1.4.2 cannot be a proof-bearing E1 arm. It changes neither the separately
frozen SAT panel nor its source revision.

The checkpoint gate is a launch prerequisite. If
`scripts/checkpoint.sh gate` does not exit zero, no command in the fixed study
panel may run.

## Question and evidence boundary

Does pinned msolve 0.11.0 accept the canonical quadratic polynomial export and
return a rational parametrization whose F17 points project to exactly the
already-frozen E0 model sets?

This is an external representation and model cross-check. msolve solves over
an algebraic closure and emits no independently checkable UNSAT certificate.
Therefore even a perfect match receives only
`msolve_base_field_crosscheck_passed_no_unsat_certificate`; it cannot complete
E1, show that F4 is fast, or establish an index-calculus improvement.

No timing, memory, work counter, or solver statistic is collected. Process
deadlines are correctness guards and are not measurements.

## Frozen implementation and tool

The runner is `native/src/e1_msolve.rs`; the strict output parser and
base-field extractor are in `native/src/msolve.rs`. Launch supplies the exact
40-hex source revision, and the aggregate records that revision and the
running executable's SHA-256.

| Item | Frozen value |
|---|---|
| msolve release | 0.11.0 |
| source commit | `cc044551f8e202d00691990fd3fd0b1226a88ea2` |
| binary SHA-256 | `7548742eb77d5f235dce7f20ae94343926f36e93edb5f371d4b11c283adb2b3c` |
| dependencies | MPFR 4.2.2, FLINT 3.6.0, system GMP/libc; static-only msolve/FLINT/MPFR build |
| upstream validation | 71/71 msolve tests passed |
| invocation | `msolve -t 1 -c 2 -d 4 --random-seed 0 -f INPUT -o OUTPUT` |
| process policy | fresh process; one thread; 300-second deadline; no scientific retry |

The runner rejects a binary-hash mismatch, an existing output root, a timeout,
a signal or nonzero exit, missing/non-UTF-8 output, malformed output, any
semantic mismatch, and every write-once artifact collision.

After the comparator implementation is committed and the checkpoint gate
passes, build that exact commit with Rust 1.78 and launch exactly:

```text
prime-shared-coeff-ir e1-msolve-panel MSOLVE OUTPUT_DIR SOURCE_REVISION
```

`MSOLVE` must resolve to the binary hash above, `OUTPUT_DIR` must not exist,
and `SOURCE_REVISION` must be the exact 40-character lowercase commit used for
the build. No current command is authorized because the checkpoint gate is
still blocked.

## Output semantics and extraction

msolve's finite-field output parametrizes algebraic-closure solutions. The
runner accepts only these complete top-level forms:

- `[-1]:` for an algebraic-closure-empty ideal;
- `[1,nvars,-1,[]]:` for a positive-dimensional ideal, which fails this arm;
- one finite `[0,[...]]:` rational parametrization.

For a finite result, the parser checks the characteristic, positive quotient
dimension, unique variable names, linear-form length, exactly one
parametrization, polynomial degree/coefficient consistency, canonical
polynomial coefficients, nonzero denominator, and one coordinate polynomial
for every listed variable except the final parameter variable. msolve may
reorder input variables and may add its documented separating variable `A`.
No other missing or extra name is accepted. Printed signed 32-bit linear-form
coefficients are normalized modulo the characteristic because msolve's
documented genericity path can print unreduced integer powers; polynomial
coefficients must already be canonical. For a non-radical zero-dimensional
ideal, the positive elimination degree may be smaller than the quotient
dimension but may not exceed it.

For every `t` in F17 satisfying the elimination polynomial, the denominator
must be nonzero. Each preceding coordinate is reconstructed as
`-coordinate(t)/denominator(t)`, while the final listed variable equals `t`.
Every recovered canonical assignment is then replayed against every equation
in the canonical IR. Only after that replay is its declared semantic
projection compared with the E0 references. A finite algebraic-closure result
with no F17 parameter root is an empty F17 model set, not a solver failure.

An earlier non-study semantic smoke fixed this distinction: over F3, msolve
returns a degree-two parametrization for `x^2+1`, although it has no F3 root;
adding `x^3-x` returns `[-1]`. Bare msolve SAT/UNSAT language is therefore
inadmissible for this experiment.

## Frozen panel and controls

The target panel is all 18 affine points on
`y^2=x^3+2x+2` over F17 at `(m,t,c)=(3,3,1)`. Before each external launch, the
coefficient enumerator and the algorithmically separate elliptic
group/interpolation oracle must still agree. The msolve projected model set
must then equal that exact set target by target: six nonempty targets, twelve
empty targets, and six projected models in total.

Fixed controls are the historical F17 `t=4` witnesses at `m=3,5,7`:

1. append one polynomial fixing every canonical field variable to the known
   assignment and require exactly that full assignment;
2. replace only `a0` by `(a0+1) mod 17`, fix every field again, and require no
   F17 assignment;
3. require the source witness to pass canonical replay and an independently
   implemented affine group-law replay before either solver launch.

Parser controls accept one complete finite synthetic parametrization and
reject a missing terminator, trailing token, duplicate/missing canonical name,
wrong characteristic, zero denominator, undocumented extra variable, and the
positive-dimensional marker. Unit tests additionally check coordinate signs,
algebraic-closure roots with no base-field root, malformed degree and
coefficient mutations, fixed-input construction, and output trailing data.

## Pass and stop rules

Pass only if all 18 target sets match exactly, all three fixed positives
recover exactly the requested full assignment, all three fixed mutations have
no base-field assignment, every canonical/group replay succeeds, every parser
control passes, and every hash/write-once invariant holds.

Stop at the first mismatch and preserve the partial run tree. Do not change
flags, parser policy, target order, deadline, fixed controls, or references and
continue under this freeze. A timeout or unsupported positive-dimensional
result is a failed comparator arm, not evidence for either solver family.

Even on pass, every empty result has
`unsat_certificate_checked=false`, the aggregate has
`independent_unsat_certificate_available=false` and
`closes_e1_unsat_gate=false`, and E2 remains blocked until the proof-bearing
SAT subgate and all other launch requirements are resolved.

## cvc5 1.4.2 disposition

The pinned cvc5 binary has SHA-256
`6aecddf466552b83bb5883756c7955e0c9d5ddcc6d7f45b48c2f6e06a741b102`
and source commit `282d5e0c84baf49c780704c532f87ba66d8b513d`. A non-study finite-field
contradiction exposed these proof limitations:

- the default checked proof contains trusted `DISTINCT_VALUES` steps;
- `--check-proofs --check-proofs-complete --proof-pedantic=1
  --proof-format-mode=alethe --no-proof-allow-trust` aborts on a trusted
  `THEORY_FF` rewrite;
- with trust allowed, Alethe export rejects `CONST_FINITE_FIELD`.

Thus cvc5 1.4.2 may later serve as an optional SAT-model cross-check, but an
empty or `unsat` answer cannot satisfy the frozen independently checked UNSAT
rule. It is excluded from this mandatory comparator run rather than silently
credited as proof-bearing evidence.

## Artifacts

The fresh run root contains, for each case, exact input, solver output,
stdout/stderr, SHA-256 values and a JSON attempt receipt. The aggregate is
`e1-msolve-crosscheck.json`. A result may be reported only with the raw durable
location, a descriptive report, a quantitative target-by-target diagram and
an inspected PDF. No aggregate may hide a failed, timed-out, malformed, or
empty case.
