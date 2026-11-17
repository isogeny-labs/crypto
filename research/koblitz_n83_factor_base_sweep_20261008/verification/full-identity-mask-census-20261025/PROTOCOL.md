# Primary N83 complete identity-mask formula census

The [machine protocol](protocol.json) freezes all 16 identity masks for each
of public trials 0, 1 and 2, both qualified hash/Gray residual pools, and
the 5+1 and 4+2 summand splits. The external worker checks the mask against
`2^(m-2)` for the **total** `m=6` summands, so the Cartesian set contains
`3 × 2 × 2 × 16 = 192` formulas. The earlier hosted panel contains only masks
0 and 1 for trials 1 and 2. This census measures exact XCNF byte digests,
variables, original CNF clauses, XOR rows, and export wall; it launches no
relation solver. Formula shape is a screening measurement, not relation yield
or total index-calculus runtime.

The runner requires the pinned Darwin ARM worker binary and its unchanged
mathematical source, the public corpus, the exact three archive bytes from
the 11-object S3 manifest, and the previously frozen 16-formula panel.
It rejects any disagreement with one of those 16 prior XCNF digests. Each
formula is exported into a new directory, hashed and inspected, then its
large XCNF is deleted after the receipt is saved. A JSONL progress stream is
fsynced after each successful export. The complete `census.json` is written
only after all 192 exports pass. A second invocation can reproduce and compare
every exact row against the committed result.

The pinned `ChainedS3Encoding::build` has an unconditional contradiction when
two adjacent mask bits are set: an intermediate identity would have to remain
the identity after adding an affine summand. The separate [structural analyzer](analyze.py)
marks those eight masks per trial/pool/split as `UNSAT_BY_ADJACENT_IDENTITIES`.
It reports formula-size minima only among the other eight masks, which still
may be unsatisfiable for the particular target and domains. The census keeps
all 16 masks so the finite Cartesian coverage and the source behavior remain
auditable; small impossible formulas are never promoted as candidates.
The [pool-domain screen](domain_screen.py) applies a second necessary
condition to these exact public trials: if the final intermediate sum is the
identity (mask bit 3), the sixth summand must have the target x-coordinate.
The sixth summand is always drawn from the residual pool. Each target x is
absent from both exact pool archives, excluding masks 8, 9 and 10 in addition
to the adjacent-identity cases. The remaining masks 0, 1, 2, 4 and 5 have
no proven contradiction from these two checks; this does not establish that
they yield a relation.
The [exact pair-sum screen](pair_screen.py) then applies the next necessary
condition. In masks 4 and 5 the first four summands sum to the identity, so
the final two must sum to the public trial target. It checks every signed
base-plus-residual and residual-plus-residual pair by group subtraction and
hash lookup against the exact archive sets. A zero-hit result excludes these
masks for that trial and pool. It does not characterize masks 0, 1 or 2.
The [support screen](support_screen.py) counts unordered multisets of the
summands remaining after each cancellation. Dividing by the nonidentity
subgroup size bounds the fraction of targets that the resulting sumset could
cover. This is exact combinatorial accounting conditional on the factor-base
sets, with possible collisions making the true coverage smaller. It is not a
probability claim for the pinned deterministic trial targets.

The source-pinned worker command for each row is
`export-cross-xcnf-pattern BASE HASH MANIFEST ARCHIVE POOL HASH MANIFEST
ARCHIVE CORPUS BASE_SUMMANDS RESIDUAL_SUMMANDS TRIAL MASK OUTPUT` using the
base and pool object identities from the earlier panel protocol. The run
order is trial, pool, split, mask. The output directory must not exist.

From this worktree, after the 11 S3 inputs have been fetched and replayed:

```sh
python3 research/koblitz_n83_factor_base_sweep_20261008/verification/full-identity-mask-census-20261025/run.py \
  --inputs /private/tmp/n83-complete-s3-get-5_f2e7o_ \
  --worker "$PWD/target/release/examples/koblitz_n83_mixed_orbit_worker" \
  --source "$PWD" \
  --out /Volumes/SSD990/llm/tmp/n83-mask-census-20261025-01
```

To replay a committed census, use a fresh output directory and pass the
committed `census.json` as `--expected`. The runner compares every deterministic
field and leaves host wall time as a separate observation.

This census does not test any other public trials, summand counts, base sizes,
source-point generators, solver configurations, or combined symmetry
operations. No winner is selected from formula size. A later bounded natural
relation experiment must preregister any mask selected from this screen and
measure failures, verification, closure, rank, linear algebra, unseen-target
descent, full cold cost and matched rho separately.
