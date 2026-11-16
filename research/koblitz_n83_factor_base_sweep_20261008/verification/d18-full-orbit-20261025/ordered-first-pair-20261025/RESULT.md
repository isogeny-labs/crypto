# Full-D18 ordered first-pair result

Source commit `c2d39e4948ab3a9dbf2e67ad705476441c6a5d02`, release example
SHA-256 `371b32ca7ebe1bacd679182b7ee0984b0d6697ae7ab74a1619896249c49a464b`.
The input was freshly fetched from the frozen S3 factor-base generation;
compressed-object SHA-256
`eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`.
The 32,643 signed-Frobenius columns represent 5,418,738 point rows. The
positive and natural formulas retained 329,808 variables and 3,901 native
XOR rows, with 2,807,999 and 2,807,501 CNF clauses respectively.

The constructed control passed in 1.077 seconds of search after 66.994
seconds of import. It returned row indices `[0,166,332,498]`, rank one,
and an independently checked original-field group sum. Independent replay
rejected changed-target and changed-row-digest controls.

The first public natural target ran until the worker's own 2,400-second
wall cap. The preserved `result.json.cap.json` says `UNKNOWN_wall_cap`;
the process exited 124 after 2,401.126 seconds under the 2,410-second
outer supervisor. Natural import took 60.847 seconds and model construction
0.303 seconds. Maximum resident set was 4,401,053,696 bytes, with zero
swaps in `/usr/bin/time -l`. The worker emitted no solver terminal result,
model, conflict count, or natural relation. The absence of those fields is
part of the censoring, not a zero measurement.

The initial supervisor incorrectly called the expected internal exit 124
`PRODUCER_FAILURE`; its original `summary.json` is retained verbatim.
The amended runner recognizes a fully checked internal cap, and the
independent auditor records both the historical misclassification and the
audited `PASS_CENSORED_NATURAL` result. It checks the committed source and
binary, S3 provider receipts, formula shapes, cap provenance, positive
relation, and both relation tamper controls. A separate altered cap-status
control is also rejected. The raw cap and summary have not been rewritten.

Two five-second live `/usr/bin/sample` captures put most main-thread samples
in `Solver::propagate`. They identify a profiling lead, not a measured XOR
pass count or a speedup. This is one shared-host L0 natural query with no
hard memory cgroup. It provides no natural relation yield, useful rank,
factor-base logarithms, unseen-target descent, fully charged cold runtime,
or matched same-target rho. `selected_best_total_runtime` remains null.

`evidence/` contains the raw worker and supervisor receipts, independent
audit, S3 GET metadata, both samples, required test logs, and SHA-256 manifest.
