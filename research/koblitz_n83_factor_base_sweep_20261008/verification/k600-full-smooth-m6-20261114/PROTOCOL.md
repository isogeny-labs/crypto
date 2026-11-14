# N83 a=0 K600 full-base six-summand admission — prepared 2026-11-14 UTC

## Contribution and question

This protocol admits a full-base six-summand chained-S3 search on the
retained K600 hash base. It tests whether the smaller orbit domain and
larger six-summand decomposition count provide a usable ordinary-relation
producer on the exact primary N83 curve. The [static screen](../orbit-domain-frontier-20261011/RESULT.md)
gives 120,614 projected variables, 323,796 orbit-domain clauses, and a
uniform-target unordered-multiset mean of about 561. Those figures are
construction and counting inputs, not measured relation yield or cost.
The [hosted K600 six-summand pilot](../hosted-sixsum-access-20261025/README.md)
used a 4+2 split with a separate residual pool; its two capped searches
do not exercise this full-base circuit. The earlier [K64 m6 run](../two-hour-extension-20261010/REPORT.md)
passed construction, while its one-trial container exited without a worker
summary. Preserve these distinctions in every comparison.

Sources checked (KB): `paper:de6c5ee2488dc9c3dca664f3306f680696ed20ad279309d4f11ed7532a806b6b`
(`research-pdfs:files/two-hour-extension-20261010__REPORT.pdf`) supplied the
K64 m6 construction and failure boundary;
`source:6d34c3cba1e4349394daa48e758671e0`
(`crypto-autoresearcher:experiments/EXP-ICPERF-554db4/specification.yaml`)
motivated paired unsorted controls before any solver-time inference. The
repository reports linked above are the primary execution receipts.

## Frozen mathematical input and stored object

- Curve: `icv1-f2m83-tm6151469093347-debefd74`,
  `y²+xy=x³+1` over `F₂[z]/(z⁸³+z⁴⁵+z²+z+1)`, subgroup order
  `2417851639230796216685689`, cofactor 4.
- Base: `a=0`, `public_x_hash`, seed `2026100801`, 600 signed-Frobenius
  columns and 99,600 stored subgroup points. Point-set BLAKE3 is
  `ac664f5f0c7122c52a08c0a288b543efb709111a99dc2a5cb2bef025ffffee21`.
  The compressed object is
  `objects/b4dda45e6c3edfca673db6f3f82c16cce8c860f5e62c092155258df399a0363e.jsonl.gz`,
  3,025,798 bytes with SHA-256
  `3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`.
  Obtain its versioned S3 bytes or a byte-identical retained copy and
  recheck this hash before staging. The panel manifest, replay, upload
  receipt, public corpus and validation sidecar SHA-256 values are,
  respectively, `5bd862987844f709e03beeab9fd265e3d2024011bcadcb82f5af9c72018df6a6`,
  `fa5e545c27dd64c32a47ddae05a2a0b3a031545d700897f3e0e34f59abc0d72e`,
  `0eae40ffa0e938470671cf796633c1e47b61ff676df0807e7a7e8592e56bb828`,
  `760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd`,
  and `1c6d3a45c8b4654ff208751258fa2e34f0cc60e1c30ba48e9214db62be60472f`.
  The [local staging receipt](staging.json) records byte counts and SHA-256
  for the six copied inputs under source commit `0c38b582caa71aa9b94e0f0a5bb4e92ff72d19fd`.
- Target: first sealed public corpus point, supplied as a point to the
  worker. The known-answer sidecar is read only after scalar production.
  Identity pattern 0 is the all-affine chained-S3 arm; other patterns
  require separate coverage and remain outside this first admission.

## Admission sequence and proposed local resource envelope

The earlier local pilot allowance is exhausted. **This protocol grants no
new execution budget.** The proposed new ceiling is 3,600 local charged
seconds for one source-matched Linux ARM worker build, one zero-trial
capacity preflight, one constructed-target positive control, and at most
one ordinary public-point trial. Each stage must fit one pinned CPU, a
4 GiB hard cgroup, zero swap, no network, read-only inputs and root, and
an outer wall guard. Charge build, imports, failed stages and cleanup to
the ceiling. Preserve every attempted stage and stop when a prerequisite
fails or the remaining allowance cannot cover the next stage.

1. Commit the exact source and build the Linux ARM worker from it. Freeze
   the binary hash, Docker image ID, object, panel, fixture, and source
   hashes in a pre-execution lock. The current generic entry point is
   `primary-orbit-chain-cold`; the [supervisor](../../chain_search_supervisor.py)
   enforces source cleanliness and the cgroup and input gates. The source
   commit for any *executed* gate is the final committed build source,
   recorded in its lock, rather than this preparation document's parent.
2. Stage the hash-matched compressed object in a fresh external panel copy.
   Run exactly one zero-trial preflight with K=600, hash policy, seed
   2026100801, m=6, orbit-domain mode, 125,000 maximum variables,
   350,000 maximum domain clauses, one model, and a 10,000-conflict ceiling.
   Record actual CNF/XOR shape, cgroup peak, source, archive and terminal
   status. A static threshold pass alone does not admit search.
3. Use the new `primary-k600-m6-xcnf positive-pinned` and
   `positive-unpinned` exports on this *same* retained base and circuit.
   The release example suite passes 22 tests [pcc:unverified];
   [the raw test log](cargo-test.txt) is retained, but the new exporter
   has not yet been run on the full archive. Preserve its actual six points,
   proper partial sums and public target. Require a checked model against
   the original CNF/XOR rows, complete point sum, subgroup, and
   signed-Frobenius relation row. Freeze any source addition before running
   it. A capped or invalid control stops the ordinary arm.
4. Export `primary-k600-m6-xcnf natural` and run at most one ordinary
   fixture-1 query with the same source, base,
   circuit and resource caps. Independently replay any solver model against
   the original formula, all point lifts and group equation, and the
   relation row. Record rank contribution. Retain timeout, OOM,
   `UNKNOWN_solver_cap` and producer failure as distinct outcomes.

The admission result can justify expanding to multiple ordinary relations
and then a full same-point single-target IC/rho comparison. Select a base
only after relation collection, rank and logarithm recovery, sparse linear
algebra, target work, and matched rho are measured under the repository's
candidate and tier contracts for both exact N83 curves. Keep this gate's
`selected_best_total_runtime` null.
