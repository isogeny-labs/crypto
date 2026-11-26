# p31 full-graph two-core and weighted connector result

This continuation uses the audited 25,000-point base and all 299,945,714
selected two-large-prime edges from `edges25k_fullspace.bin` on the frozen
Picard model over p=2,147,483,647. The subgroup order is
11,832,483,282,198,303,559. The input edge SHA-256 is recorded in
`audit25k.json`; the source target and transfer are unchanged from the
checkpoint solver bundle. This continuation tests graph relation extraction
and factor-base rank. The graph arm still needs enough rank, linear algebra,
target descent, and original-curve scalar replay.

## Exact graph compression

`xor_core.cpp` first marks occupied large-prime x coordinates, compresses them
to dense IDs, and peels degree-one vertices using degree and XOR of neighbors.
A second pass retains exactly the edges whose endpoints survive. It checks
strict input ordinals, endpoint bounds, and the identity that the core degree
sum is twice its edge count. The resulting `core25k.bin` has SHA-256
`b74888f5ccdcfd9d93779df14acf32a9cfdf1ca45a8ab302813dab0f88db44f3`.

| Quantity | Exact value |
| --- | ---: |
| Input edges | 299,945,714 |
| Input vertices | 371,767,656 |
| Peeled vertices | 297,538,636 |
| Two-core vertices | 2,369,636 |
| Two-core edges | 2,407,078 |
| Two-core components | 3 |
| Two-core cycle rank | 37,445 |
| Degree-two vertices | 2,296,459 |
| Branch vertices | 73,177 |
| Suppressed kernel edges | 110,619 |
| Pure degree-two cycles | 2 |

`core_topology.cpp` independently checks the core's minimum degree and
`E-V+C=37,445`. `compress_kernel.cpp` partitions every core edge exactly once
into an oriented degree-two path. Its chain and path files have SHA-256 values
`fc9bad0024e15ce2acac65c2e4e1209163cbc537bf1484593be068b53d83645e`
and `44c0f4cb96c061ee329eb7e055510200c3eaf1391f79b7623980b935f984f395`.
The 28.9 MB core edge file and 110,619-edge kernel make exact weighted graph
algorithms feasible without the 300-million-edge adjacency structure.

## Short weighted cycles and pivot connectors

`kernel_cycles.cpp` uses a minimum-chain-length spanning forest and extracts
the shortest fundamental cycles, expanding each back to the core edge indexes
and x-coordinate path. Among all 37,445 fundamental cycles, the shortest has
18 original edges and the median has 1,423. The first 200 cycle lengths range
from 18 to 293. `replay_short_cycles.py` independently recomputes every
selected line intersection, C3 orbit coefficient, and weighted cancellation.
Thirty of the 200 cycles cancel all large-prime columns; those 30 rows have
factor-base rank 30 over the selected subgroup field. The shortest complete
row uses 26 source lines and has 52 factor-base entries. The 18-edge cycle
retains a large-prime pivot.

The remaining 170 cycles have distinct pivots. `connect_pivot_forest.cpp`
uses multi-source Dijkstra on the kernel and Kruskal on the resulting Voronoi
boundary graph. It selects 167 connectors among the 170 pivots; the other
two pivots occupy isolated degree-two cycle components. The selected
connectors contain 17,131 original edges in total, with median length 105
and maximum length 156. `replay_pivot_forest.py` checks every connector line
and cancels both endpoint pivots against its two source cycles. All 167
connector-pair rows are valid, and the combined 197 rows have exact modular
rank **197/197** in the 25,000-column factor base. The nearest pair uses an
11-edge connector and yields an independent 1,029-entry row from 530 source
lines; its full witness is in `pivot_pair_relation25k.json`.

This is a concrete graph-algorithm improvement in relation extraction: the
chronological forest's first sampled closure path was 520 edges, whereas the
shortest complete kernel cycle here uses 26. These are different samples and
factor-base sizes, so this is a path-length observation, not a matched
end-to-end speedup. The kernel forest and connector search use established
graph methods; novelty is not asserted.

## Cost and remaining gate

The completed core pass took 814.22 seconds external wall under concurrent
host load and reached 3,030,450,176 bytes maximum RSS. A prior direct-index
version was interrupted after more than six minutes when host swap grew; its
exact completed wall time is unavailable and is additional campaign cost.
The topology, kernel compression, 200-cycle extraction, cycle replay,
connector search, and 167-connector replay took 2.93, 2.04, 0.69, 10.65,
0.28, and 53.60 seconds external wall respectively. These are single
observations, and the graph arm has not yet produced 25,000 independent
rows. The next stage is streaming weighted closure generation across the
full 37,445-cycle space, row-density control, rank completion, sparse linear
algebra, and target descent with scalar verification. The complete cost must
include the interrupted pass and all setup.

The graph baseline follows Diem's double-large-prime plane-curve method:
<https://jtnb.centre-mersenne.org/item/10.5802/jtnb.815.pdf>.
The established width-versus-memory variant and its row-weight motivation are
in <https://eprint.iacr.org/2014/346.pdf>. The graph compression and
shortest-connector choices here are implementation choices within that
framework, tested at the exact frozen p31 instance.

## 1,000-cycle rank-scale continuation

The first 200-cycle inputs and outputs above are preserved in `sample200/`.
The same fixed kernel forest was then extended to its 1,000 shortest
fundamental cycles, with maximum cycle length 509 and median length 396.
Exact line replay found 163 balanced cycles and 837 pivot cycles. Two pivot
coordinates repeat. The generalized Voronoi/Kruskal connector forest handles
those as zero-edge joins and selects 834 connectors in total, with median
length 74, maximum length 142, and 59,342 connector source edges. All
834 connector equations were replayed and saved one at a time in
`pivot_forest_rows25k.jsonl`; `pivot_forest_rows25k.manifest.json` binds that
stream to the 1,000-cycle and connector inputs. Together with the 163
balanced-cycle rows, the 997-row matrix has 1,268,547 nonzeros.

An earlier combined replay/elimination attempt was interrupted by a host
reboot under heavy concurrent swap pressure. Its process and output handle
disappeared, and its stdout and time log are empty. It supplies no rank or
timing result. The checkpointed replay took 2.77 seconds external wall after
the reboot; the 1,000-cycle source replay took 60.08 seconds before it. The
interrupted attempt remains additional campaign cost with no exact final
wall time.

`project_rows.py` maps each of the 25,000 factor-base columns to one of 997
columns using `splitmix64(20261123 xor column) mod 997`, forming an exact
right projection over the subgroup field. The projected 997×997 matrix has
SHA-256 `13a047d98471af0ad8cc548a9b7befd11d163136436a60697151bcfb0260ee96`.
`rank_projection.cpp` computed rank 997 and determinant
**8,660,191,060,892,660,824 mod ell**, which is nonzero. An independent
`python-flint` `nmod_mat` run on the same hashed matrix reproduced both
values exactly; its receipt is `rank_projection25k_flint.json`. Because a
right projection cannot increase row rank, this certifies exact rank
**997/997** for the original 997×25,000 matrix. The C++ rank pass took
4.94 seconds external wall; the independent FLINT replay took less than a
second. This is a rank certificate for the extracted rows, not a complete
25,000-column factor-base solve.

## 2,000-cycle continuation

The next 1,000 shortest fundamental cycles were independently replayed from
the same frozen core (`short_cycles_batch1000_replay25k.json`): 155 balanced
and 845 pivot cycles, with lengths 510–649. Combined with the first batch,
the 2,000 cycles contain 318 balanced and 1,682 pivot cycles. The generalized
connector forest selects 1,679 pair joins. Their exact source replay is
checkpointed in `pivot_forest_rows2000_25k.jsonl` and bound by its manifest.
The resulting 1,997×25,000 factor-base matrix has an exact deterministic
right projection with SHA-256
`d48c6255d5f307baf1efa4ab9b77de1fb0b5d9266bfa3de9d849a149b77d503c`.
The C++ elimination and independent `python-flint` `nmod_mat` computation
both return rank **1,997/1,997** and determinant
**6,121,886,874,340,065,768 mod ell**. The independent receipt is
`rank_projection2000_flint.json`. This proves independence of every
extracted row in this batch. It does not yet establish the 24,999 ranks
needed to determine the 25,000 factor-base logarithms up to scale.

The next implementation step is a streaming cycle-witness format for the
remaining 35,445 fundamental cycles, retaining per-row replay checkpoints
and minimizing intermediate JSON size. Rank certification beyond a few
thousand rows needs sparse finite-field methods; dense projected elimination
is not the intended 25,000-row solver. After rank completion, the existing
checkpointed sparse-kernel solver and the p31 target descent must be adapted
to this 25,000-point base and verified on the source genus-four curve.
