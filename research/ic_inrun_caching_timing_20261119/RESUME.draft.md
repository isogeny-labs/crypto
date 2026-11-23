# RESUME (draft for .resume/RESUME.md; the lead fills the <…> fields)

## Goal

Time the default-on in-run caching changes of PR #4756 (isogeny-labs/crypto)
on Iso Lab EKS. This is an engineering measurement, old binary vs new, with
counted units first and L2 wall time second. The owner authorised an EKS
timing run on 2026-11-19.

## Lead / hypothesis

The source is PR #4756, "Remove repeated target-independent work inside one
cold IC run". The hypotheses are H5 (batch inversion in prime
`subtract`/`mitm`), H2b (one filter per paired sparse solve) and H4
(`descent-algebraic` S_3 template). Each gives the same answers at lower
counted or wall cost. C1, C2a, C3 and the `petit_quisquater` change cannot
be reached from ecbench (research/ic_inrun_caching_timing_20261119/BLOCKERS.md).

## Spec

- Protocol: `research/ic_inrun_caching_timing_20261119/PROTOCOL.md` (v1).
- Specs: `specs/p24a-batch-sparse.json` (sha256 6def46f9…e41) and
  `specs/k0-13-descent-template.json` (sha256 f88f5956…35a).
- OLD = `<OLD_SHA>` (`ad2a014` + the study directory, branch
  `ic-inrun-caching-timing-baseline`).
- NEW = `<NEW_SHA>` (the PR #4756 head with the study directory; code =
  `a0b215b0d`).
- Inputs: workflow `isolab-experiment.yml` on `main`, c8a.2xlarge,
  seconds=4800, L2. Ceiling USD 15, 1 job at a time, at most 2 re-dispatches.

## State

Nothing has been measured. The protocol and specs are drafted, and the
pre-launch steps of DISPATCH.md §0 are `<done/not done>`. Records:
`<REC paths>`.

## Next steps

Follow DISPATCH.md §1. Run the 8 dispatches in the order O-P24-1, N-P24-1,
N-K13-1, O-K13-1, N-P24-2, O-P24-2, O-K13-2, N-K13-2. Start each only after
the previous one is terminal, and only after a pool check and a budget
readback. After each, download and verify the S3 receipt and session, then
run `benchrec add-raw` and `close`, then `checkpoint push`. When all 8 are
done, run the PROTOCOL.md §10 analysis, the independent validator, the
visuals report, the `benchmarks/` entry and the pcc claims.

## Runs in flight

`<none | for each: job, GitHub run id, gh-RUN-ATTEMPT Job name, dispatch
time, receipt dir research/ic_inrun_caching_timing_20261119/receipts/<job>/,
how to collect: gh run view <id>; private S3 prefix from the receipt>`

## Stop and success conditions

Stop new launches at USD 15 charged plus reserved, when the budget remainder
is unknown, after 2 re-dispatches, or if G2 (identity) fails in a completed
pair. Success means each decision D5s, D5m, D5n, D2b, D4 and S is read
exactly as PROTOCOL.md §7 states, whether it comes out an improvement, a
null or a regression, with G1 and G2 holding. No other reading counts.

## Open questions

- Does the old binary accept `batch=0` (BLOCKERS B10)? Job O-P24-1 answers it.
- B1–B4 need a separate study that runs the `ic` or `icx` binaries.
