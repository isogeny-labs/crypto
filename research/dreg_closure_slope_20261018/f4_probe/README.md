# F4 as a second instrument: probe records (not registered, not evidence for the slope)

`examples/dreg_f4_degree` replays a committed draw through the Boolean F4
(`pq_f4_f2::groebner_basis_f4`, field equations included) under a wall
budget and records the degree reached and the solving degree. It was tried
because the sparse MutantXL closure capped at degree 5 on every `m·ℓ = 21`
draw. These records were made before any amendment was registered, so they
are an instrument probe, reported for completeness; none of them enters the
slope fit.

| draw | engine setting | outcome | wall |
|---|---|---|---|
| m3 (18,6) draw 0, N = 36 (anchor; dense and sparse closure 4) | step-matrix cap 1 GiB (default) | refuted, degree reached 4, solving degree 4, largest matrix 88,696 × 66,697, peak 740 MB | 418 s |
| m3 (19,7) draw 0, N = 40 (closure > 4) | cap 1 GiB | stopped by the cap at the degree-5 step (`oversize`), basis of 3,096 elements returned, not a result | 507 s |
| m3 (19,7) draw 0, N = 40 | cap raised to 8 GiB words (`F4_F2_MAX_MATRIX_WORDS=1073741824`), address space 13 GB | killed by the memory cgroup at 12.6 GB RSS in the degree-5 step (`dmesg`: "Memory cgroup out of memory: Killed process … (dreg_f4_degree) total-vm:12641472kB, anon-rss:12633744kB") | — |

`probe-1gib-cap.jsonl` holds the two records of the first two rows exactly
as the probe printed them (its JSON did not yet carry the `oversize` field;
the second row's `refuted:false, basis_len:3096, timed_out:false` is the cap
case, as the source at that commit shows). The third run printed nothing
before it was killed.

What this says: on this 4-core, 15 GB container neither the sparse closure
(entry cap 350 M, reached in about 3 minutes) nor F4 (more than 12.6 GB)
builds the degree-5 step for the chained `S₃` system at 40 Boolean
unknowns; a dense degree-5 Macaulay matrix at N = 40 would need about
72 GB. Degree 5 at `m·ℓ = 21` and above is therefore out of this
container's reach with every engine the repository has.
