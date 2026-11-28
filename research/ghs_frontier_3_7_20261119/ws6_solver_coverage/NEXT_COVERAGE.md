# Exact gap to a live `--require-all` campaign

[`missing-coverage.json`](missing-coverage.json) is the machine-readable delta.
The historical 92-spec development set is structurally valid but not a live
closed census. It covers 20/111 top-level methods and 17/37 nested algorithms;
91 methods and 20 nested algorithms remain missing. The audit exits 1, as it
must.

The next development spec set must add applicable correctness/accounting cells
for every ID in those two arrays. It must retain, rather than replace, the
existing cells. At minimum, the nested additions include:

- the three newly surfaced PDP3 root engines:
  `rootfind-negfold-batch`, `rootfind-negfold-batch-harvest`, and
  `rootfind-negfold-batch-harvest-torsion`;
- `f6-ic-pair`, the two windowed root routes, and all four PKM-tower engines;
- M4GB in-process/server, inherited-F4 next/legacy/batched, both
  FES-then-* hybrids, GPU FES, PRIMO-SMT in-process/file, and every remaining
  system solver listed by the audit.

Each development spec needs a reference, a baseline or candidate, an exact A/A
control, even rounds, a warmup, explicit resource caps, and a deterministic
seed. Development and holdout target/measurement seeds remain disjoint. The
winner-only holdout stays public-target and L2. WD-SAT is not added: it is not a
live `SystemSolver`, no binary is installed here, and the retained 1-of-16
false-UNSAT result blocks promotion until repaired and independently replayed.

Passing `--require-all` would freeze coverage only. It would not establish a
solver speedup. F4 word work, SAT conflicts, scalar/vector FES XORs, PKM
multiply-adds, and root-table work remain unlike units. Cross-ranking requires
complete common pricing or matched verified L2 end-to-end execution with cold
setup, relation work, solving, recovery, verification, failures, memory, replay,
and the strongest applicable rho reference `[pcc:unverified]`.

