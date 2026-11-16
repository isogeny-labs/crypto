#set page(paper: "a4", margin: 16mm)
#set text(font: "New Computer Modern", size: 10pt)
= Retained N83 K = 16,627: four-summand model capacity

*Date:* 2026-10-17 UTC. *Curve:* `icv1-f2m83-tm6151469093347-debefd74`.
The protocol was committed before the adapter change or run (`PROTOCOL.md`,
`CONTROL_ADDENDUM.md`). Raw worker, cgroup and container receipts are under
`negative/` and `full/`. Exact source and input hashes are in
`BUILD_RECEIPT.md`; the detailed interpretation is in `RUN_REPORT.md`.

#image("capacity-diagram.svg", width: 100%)

== Matched input, different caps

#table(
  columns: (auto, auto, auto),
  inset: 5pt,
  [Measure], [One-variable control], [Full-cap model],
  [Status], [`UNKNOWN_variable_cap`], [`PASS_model_construction_only`],
  [Required variables / domain clauses], [201,598 / 5,729,152], [201,598 / 5,729,152],
  [Installed variables / clauses / XOR rows], [none], [201,598 / 5,915,238 / 3,901],
  [Base import], [77.488 s], [67.294 s],
  [Model construction], [skipped], [0.325 s],
  [Whole-process cgroup peak], [2,661,171,200 B], [2,661,126,144 B],
  [Container exit / OOM], [0 / false], [0 / false],
)

Both runs used the same exact 16,627-column, 2,760,082-point stored object,
public target and arm64 worker. The source snapshot binds clean commit
`e54bb7d6a930571ad0e68d51c684ec68f65bda88`. Each fresh container had
one CPU, 4 GiB hard memory, zero swap, no network, read-only root and a
1,800-second outer wall cap. The negative cap stopped before installation;
the full arm built the identity-mask-zero Frobenius-orbit domain. The full
receipt reports 62,001 AND gates and three S3 nodes.

== What this establishes

This exact retained base can be imported and its m = 4 model constructed
under the frozen memory cap. Neither arm ran SAT search, collected a
relation, computed rank or performed target extraction. The cgroup peak is
whole-process, without a separate model-only memory peak. The workers
overlapped other shared-host work; their L0 walls are descriptive and do
not form a speed comparison. Construction, replay, storage transfer,
search, rank, linear algebra and verification are not combined into a
complete IC cost. The IC scoreboard stays unchanged.

Next: check a degree-83 planted four-summand relation on this exact base,
then compare natural yield and useful rank with the retained K = 2,048 and
structured 2,001-column bases under one frozen corpus.
