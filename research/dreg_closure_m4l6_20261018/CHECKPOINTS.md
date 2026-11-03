# Resumable dense closures (native checkpoints)

Status: tooling, 2026-10-24. Nothing in PROTOCOL.md §1–§11 changes. A3 (§11) does
not use it.

A closure at degree 5 (C1, §11.3) is a multi-day dense MutantXL run on a Spot
node. `examples/dreg_closure --checkpoint-dir DIR` (or `DREG_CKPT_DIR`) makes the
dense scan resumable, so a loss costs the interval since the last copy to S3, not
the run.

## What is saved, and when

The scan of one degree is: the plain Macaulay basis (XL), the seeded closure basis
(Macaulay matrix build and first elimination), then rounds. A round multiplies the
pending degree falls by monomials, adds the products below the basis, and reduces
the whole matrix (`rref`, the long step). State is written:

| point | what is written | size |
| --- | --- | --- |
| after the XL basis | its refutation flag and rank | bytes |
| after the seed | echelon rows, columns, pending falls, cost counters | the basis |
| between rounds | the same | the basis |
| inside a round's elimination, at a word boundary | the partly reduced matrix (basis rows and products), the column layout, which columns led a row before the round, the elimination's pivot row and word, cost counters | the round's matrix |
| after a degree | its `ClosureDegree` row | bytes |

Saves happen when `DREG_CKPT_SECONDS` (default 900) have passed since the last one
and on SIGTERM/SIGINT, which the process answers by saving at the next word
boundary and exiting 143. Generations follow the `manifest-sha256-v1` contract of
`infra/spot-system/NATIVE_CHECKPOINTS.md` (immutable, SHA-256, manifest written
last by atomic rename). Two are kept. `scripts/ckpt-s3-sync.sh` copies them to S3.

Not saved: the Macaulay matrix build and its first elimination (before the seed's
save) and the XL elimination itself, which are repeated after a loss that falls
in them.

## Why a resumed run is the same computation

- The Four Russians kernel's state at the start of a packed word is the matrix,
  the number of pivot rows found and the word (`gf2_elim::rref_resumable`). A test
  stops it at every word of several shapes, copies the matrix away and continues:
  matrix, rank and charged word operations equal the uninterrupted run's.
- The closure restarts from rows in one layout with no layout history. A test
  stops a scan after every round and at coarse intervals inside eliminations,
  reopening the store from disk each time, on unsatisfiable and satisfiable
  chained systems at degrees 3 and 4: every field of every `ClosureDegree` row
  except `secs` equals the plain scan's, including the cost counters. The test
  fails if the restored pending falls are altered.
- A corrupt newest generation falls back to the previous one; a directory whose
  generations all fail verification refuses to start (a long run is never silently
  restarted); a store refuses a different system.

## How a resumed record reads

`closure_degree`, `saturated`, ranks, column counts, products and word operations
are those of an uninterrupted run. `secs` is the compute time summed over
segments and is **not a cold measurement**: the line carries
`checkpoint_segments`, and a resumed run is never used for a timing claim. The
time limit of §5 and §11.3 becomes the number of segments (40 at most) times the
segment length; a record produced under it says so.
