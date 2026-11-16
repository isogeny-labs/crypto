# Target-preserving filtering: exact boundary and native gate

Protocolf847c25ba8preceded code. Singleton peeling is known filtering; this
work specifies the target-preservation condition and catches an invalid
support-only inference before any new timing campaign.

For [B C][z;x]=d, feasible core assignments are exactly those satisfying
y^T C x=y^T d for every y in ker(B^T). The proof is finite-field linear
algebra: d-Cxmust belong to im(B), whose annihilator is ker(B^T).
At most M-rank(B)projected independent equations remain. In a two-endpoint
weighted graph a connected balanced component leaves one free root value;
an unbalanced gain cycle or one-endpoint pin removes that freedom. Therefore
unsigned graph cycle counts alone do not determine projected rank.

The explicit F5same-support pair is retained in FLOW.svg and gate.rs:
x+y+k=0withx+y=0forcesk0; replace the second equation byx+2y=0and allkare
possible. Both off-base supports and full coefficient supports are identical.
Exhaustive feasible sets and solve_fastagree. The existing affine forest
already accounts for coefficient weights; this does not identify a forest bug.

## Checked singleton rule
Normalize duplicate column terms first. Only a NON-TARGETcolumn of occurrence
one may eliminate its sole row. Store that pivot; reconstruct in reverse
order. This preserves the complete target projection, even when inconsistent.
Target-onlyk=3must remain; an inconsistent empty row0=1must remain.

Native gate passes384exhaustively enumerated systems overF5, including246
inconsistent systems. Feasible target sets agree and every satisfying kept
assignment extends back through pivot witnesses.119rowsremoved in this gate.
Fields5/7/101,widths1/3/8/20,32seeds and allprefixes0..2n+4give8064consistent
prefix checks, with28473rowspeeled across checks. solve_fastreturns identical
target results before/after; every returned value matches the planted target.
These removal counts repeat related prefixes and are not a performance ratio.

No curve stream timing, asymptotic improvement or10xfactor is established.
The full target objective remains active. Singleton pruning and harvesting
are prior art; read Cavallar Section2.1/2.2at
https://ir.cwi.nl/pub/2078/2078D.pdf. HECpublisher abstract only:
https://www.aimsciences.org/article/doi/10.3934/amc.2010.4.189.
Do not infer the full HECpaper's doubleLPcoverage or transfer quantitative
benefits without its text and a matched cost model.

## Decision
Use exact weighted target feasibility as a mandatory selector gate. Do not
launch a graph-only cycle-count selector based on support connectivity.
The preceding fixed-work free-setup+forest maximum2.190still blocks a10xcase
for a downstream-only replacement under that assumption. A different relation
selection/distribution must show how it changes required harvesting, core size,
row weights and rank jointly, including filtering overhead and losses.

gate.shbuilds the existing native library, discovers its rlib, runs native
enumeration and emits source hashes. No scientific Python, benchmark record,
cloud worker or default/leaderboard promotion. Full raw output and inspected
PDF/diagram accompany this mathematical gate. Independent review remains open.
