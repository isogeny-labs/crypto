# Pushed-Git run checkpoint

## Goal

Measure the exact oriented quadratic-norm pair transducer through Dickson
depths 1--6 and promote to m=7 only if every preregistered correctness, strict
subquadratic-growth, and cold one-target budget gate passes.

## Frozen inputs

- Protocol plus amendments 1--4 in this directory.
- Curve `icv1-fp31-tm3989-ae9db972`.
- Active source custody: `SOURCE_MANIFEST_4.json`.
- Deterministic targets: first finite planted four-leaf multiset, `[k]G` for
  `k=1..8`, and infinity at each depth. There is no random seed.
- Projection: 27,370 residual queries, with no cross-target amortization.

## State before launch

Candidate, independently coded direct-state verifier, and frozen analyzer are
implemented in dependency-free Rust. The verifier independently recomputes
every level's support, semantic circuit totals, and exact `GF(2)` incidence
rank, plus all fixed queries, witnesses, and misses. Formatting, strict Clippy,
and all nine development tests pass. Failed attempt 1 stopped before candidate
execution because `/usr/bin/time` was absent; its empty output and shell error
are retained and hash-bound. No admitted census output exists.

## Sole launch command

From the repository root:

```sh
research/pplus_oriented_pair_transducer_20261127/run_census.sh
```

The runner refuses an existing evidence directory, dirty or unpushed Git
state, a source-hash mismatch, a failed candidate, a failed verifier, or an
invalid analyzer record. It writes to a unique temporary directory and only
renames it to `evidence/` after every stage and SHA-256 check succeeds.

## Expected artifacts

`evidence/` contains candidate/verifier transcripts and stderr, timing/RSS
diagnostics, environment metadata, analyzer JSON, and `SHA256SUMS`. The
evidence is public deterministic replay, not a zero-knowledge proof or host
attestation.

## Stop and success conditions

Stop before m=7 integration unless all six supports and 60 queries verify,
every hit replays, at least one finite depth-six miss is complete, all three
metrics pass both strict adjacent-depth tests and slope `<2`, every cold field-
equivalent panel is below 1.6 million, and the optimistic support-probe floor
is also below 1.6 million. Failure is a retained scoped null result, not
permission to swap targets, amortize across DLP instances, or infer a general
lower bound.
