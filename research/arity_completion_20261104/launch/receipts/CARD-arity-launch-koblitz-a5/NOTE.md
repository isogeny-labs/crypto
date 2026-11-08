# Attempt 5 card run: local runner interrupted after submission

- 18:26Z: `executor-mechanical` started
  `python3 tools/benchrec.py run benchmarks/records/20261108T1825Z-arity-koblitz-m56-a5.json -- python3 tools/task_card.py run research/arity_completion_20261104/cards/CARD-arity-launch-koblitz-a5.yaml`.
  The gates passed: `source-identity`, `image-identity`, and `budget-readback`, whose output is `../CARD-arity-launch-koblitz-a5-budgets.json`.
  The `submit` step then started launcher build `isolab-experiment-launcher:06fac5b1-5fe5-4b16-8cc8-4d6f2e10d6c4` (`submit/build-start.json`).
- The executor's shell call hit its 600 s tool timeout and was moved to the background. That background process was killed when the executor agent ended (output `[killed]`). So no card receipt (`../CARD-arity-launch-koblitz-a5.receipt.json`) exists, and the bench record has no `runs` entry.
- The remote side completed independently. Launcher build `06fac5b1` reports SUCCEEDED. Job `arity-koblitz-m56-20261104-a5` (uid `c98d3d95-a414-433b-9ec4-2530cc3f79d0`) has pod `e87cf35a-0101-4dcf-aa58-7b97c448cb46` running on `ip-10-84-100-118`, an Intel Xeon Platinum 8488C, the same host model as attempt 4. The pod started at 18:28:22Z, resumed attempt 4's records, and started shards m5 and m6 at 18:28:24Z. The launcher's own receipts were copied from S3 to `submit/launcher-s3/06fac5b1-…/`.
- The card is not re-run: its `must_not_exist` guard now refuses, and a second submit would duplicate the Job. The final evidence is the Job's S3 prefix, checked against its SHA256SUMS after exit.
