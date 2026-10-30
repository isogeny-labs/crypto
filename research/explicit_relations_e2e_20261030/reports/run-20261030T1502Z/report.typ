#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt)
#set heading(numbering: none)
= One 61-bit recovery and a memory boundary

This observation report covers every execution of frozen protocol-v9 in the
explicit-base end-to-end study. It records bounded public-input recovery and
limits, without a scientific verdict, certified speedup, absolute hardware
maximum or matched-rho claim. The GPU oracle is generic meet-in-the-middle;
the paper's conditional tensor-contraction backend is not implemented on GPU.

== Recorded outcomes
Source `e1a38ce4bc586050c3faa67a3ab79f312eb549b5` ran on one RTX PRO 6000
Blackwell Server Edition, pod `nd5qev6envru7z`, created 2026-10-30T15:02:46 UTC.
Eight executions produced seven verified recoveries and one memory refusal.
Two 62-bit cells were unattempted under the frozen stopping rule: complete both
targets at a field size, then stop higher sizes if either failed. V9's largest
complete prescribed rung is 52 bits; its largest individual recovery is 61 bits.
The separate v8 panel completed both 60-bit targets. These are different panels.

#table(columns:(0.5fr,0.6fr,1fr,1fr,1fr),inset:5pt,
 [Bits],[Seed],[Original index b (s)],[Wide index u (s)],[Identical x (s)],
 [52],[101],[29.179238],[17.727554],[17.724473],
 [52],[202],[31.296758],[22.094780],[22.084471],
 [61],[101],[Not prescribed],[0.263632; limited],[Not prescribed],
 [61],[202],[Not prescribed],[1537.899248],[Not prescribed],
)
Measured cold solve seconds, rounded from `results-v9/rows.csv`. All clocks
are shared-host L0 diagnostics, with wall_valid=false. No confidence interval
or certified timing ratio is available. Matched 52-bit certificates are identical.

The successful 61-bit case used 1591 relations from 2432 attempts. GPU-oracle time
was 1536.244543 s and linear elimination 1.360546 s. The total including fresh CPU
certificate replay and scalar-corruption rejection was 1537.918073 s. The other
case refused a conservative admission estimate of 85,969,611,408 bytes for
1,788,236,560left rows; it did not perform a failed exhaustive search. The
successful case admitted 63,161,668,896 bytes for 1,313,071,104 left rows. These
estimates include sort reserve and other allocations, and are not peak VRAM.

#image("scaling-v9.svg",width:100%)
Each point corresponds to a recorded row; the limited row is retained.
Run IDs use pod plus field bits, seed and arm, as in `results-v9/export.json`.
There is one invocation per prescribed arm/target; x repeats the wide binary.

#pagebreak()
= Contract, validation and custody

The driver accepts certified prime-order curves with `p < 2^62` and `r < 2^63`;
hash primes stay in [2p,4p] and below 2^63. The fast Montgomery path remains
for smaller moduli; wider hash multiplication uses canonical add/double.
Checked scalar/field products use u128, inverse coefficients i128. The field
extension changes arithmetic guards, not the relation-finding algorithm.

Command: `bash research/explicit_relations_e2e_20261030/run-boundary.sh`,
with EPHEMERAL_OUT=/results, CUDA 13.2/sm_120 and Rust 1.88.0. Seeds 101/202,
m=8, tau=2ceil(r^(1/8))+2, `rows <= 2^32`, `work <= 2^34`, chunk 2^20, columns 16384,
trials 16. Cold solve caps 180 s at 52 bits, 2700 s at 61 bits and 5400 s at 62 bits;
fresh replay 60 s, PARI fixture generation 180 s, worker 18000 s/controller 18300 s.
The unchanged admission rule preserves 25% of reported free GPU memory.

Public targets are coordinate lifts; no target scalar is generated or supplied.
Each successful certificate passes fresh CPU replay and rejects scalar+1.
Curve equation, primality and the unique Hasse-interval order witness are
checked. All 19 Rust tests, host/full/cached/compact device checks, Clippy,
maximum-cap headers and full/compact/wide query memcheck, racecheck and
synccheck passed. Compile/preflight and fixture generation are outside cold
solve timing. Cold solve includes startup, checks, all relation trials,
elimination, extraction, in-process replay, serialization and teardown;
the additional fresh replay and negative control are included in total audit.
CPU user/system time and measured peak VRAM were not captured.

Frozen protocol hash:
#text(size:8pt)[`bf1e4a2dddbfdadb2cb8de32a0c659bbcdd528cc2851c63864c66d009f4e5142`]
Raw archive SHA256:
#text(size:8pt)[`cf5f4094f472ab688c11d14823df598ce6141d8ae2ee1ad70513e7881a96d0f4`]
Archive: `s3://setec-astronomy-artifacts/validation/ecbench-explicit-x/e2e-20261030-v9/results.tgz`.
Raw records were committed/pushed in `6e850f7dfc14` before this report.
The archive manifest hashes every member; certificate-audit.json records
native replay and rejection outputs. Those manifests do not replace replay.
Worker/controller exited zero, archive digest matched, deletion was verified,
and a later provider listing independently confirmed the pod's absence.

Schema, flow and file hashes pass. The frozen native analyzer and strict
frontier audit pass; the latter rejects eight coverage/status/input mutations.
The canonical protocol checker retains five rectangular-coverage findings:
18 assumed cells versus 8 actual, u covers 2 rather than 3 sizes, b and x each
cover 1 rather than 3, and arm instance sets differ. This is not a full canonical
checker pass or independent scientific validation. Details are retained in
`results-v9/CHECKER-NOTE.md`; no failed row was excluded.

== Literature and remaining implementation work
The checked Fp64 arithmetic contract, the supplied paper's inverse-hash
construction, HAC algorithms 2.107/2.142 and PARI ellcard documentation informed
the width extension. See `gpu/explicit-relations/WIDE-HASH.md` and
#link("https://pari.math.u-bordeaux.fr/dochtml/html-stable/Elliptic_curves.html")[PARI's elliptic-curve documentation].
V10 was frozen but never executed. A separate v11 protocol tests exact 12-byte
rows and queried CUB sort scratch with distinct 52-bit controls before larger
probes. Its planned results are not included here. Neither this report nor a
merged PR settles the paper's conditional backend or the hardware maximum.
