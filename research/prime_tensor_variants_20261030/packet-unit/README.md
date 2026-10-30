# Exact finite ternary-packet and target-split units

2026-10-30 UTC. These native C++17 DEVELOPMENT checks exercise a supplied
ternary packet gadget and one local mode-coefficient identity. They do not
implement a general recursive contraction algorithm, prove a3-adic precision
theorem, establish novelty, measure scaling or achieve the sub-exponential
research goal.

## Fixed packet results

The panel exhausts all pair classes in F_r^(4a) for(r,a)=(2,1),(2,2),(5,1).
Every level has row coordinates R_i1,R_i2 and column coordinates C_i1,C_i2.
The supplied action is S(R_i1,C_i1), S^-1(R_i2,C_i2), where
S(a,b)=(-a-b,a). Base3 slot digits select its powers.

| Check | Observed result |
| --- | --- |
| Full pair classes |897 |
| Disjoint action orbits |251 |
| Slot pairs including diagonal |26,505 |
| Nonzero first-difference locality checks |22,278, all pass |
| Source tiles |9,990, all have at most two cross-target groups |
| Tiles with exactly two cross targets |1,800 |
| Fresh-endpoint matching edges |913,731, all covered exactly once |
| Full packets with no unused slots |275,393 |
| Actual coordinate/symbol marginals |44 row/column counts, all equal M/r |
| F_67 single-pair panel |4,489 pairs/26,934 selected-direction locality checks |
| Negative controls |4 expected rejections |

The source checks U_i^3=I, inverse directions, distinct-level commutation,
orbit closure/disjointness, size3^k for k nonzero levels, uniform packet
frequencies and integer packet-copy counts. For every x!=y it derives
i=v3(y-x),c=((y-x)/3^i)mod3 in{1,2}, and compares the actual row coordinate
R_i,c at x with column coordinate C_i,c at y.

For every class and source tile it checks actual constant column group and
enumerates the distinct crossed row groups. The F_5 class
(R1,R2,C1,C2)=(0,0,1,0) gives rows[0,4,1], columns[1,0,4]. Column0 has
source group1 and two distinct cross targets0and4. A binary-style
one-target assumption is therefore explicitly rejected. The parent's
proposed mode count must account for this split(18M/D instead of9M/D);
this unit verifies the finite target distinction, not that full global bound.

The F_67 panel checks the actual label-field constant67>48 without
enumerating67^(4a) full classes. Its crossing-target size distribution is
1 pair with zero targets,198 with one,4,290 with two. It checks all single
pairs and both selected directions, not full high-field paired-class graphs.

## Actual finite matching graphs

| r,a | Darr | H | M edges | Packets |
| --- | --- | --- | --- | --- |
|2,1 |16 |33 |528 |176 |
|2,2 |256 |513 |131,328 |14,592 |
|5,1 |625 |1,251 |781,875 |260,625 |

Each class has exactly H=ceil(2Darr/L)L edges. Every edge has one fresh row
and column vertex, so Delta=1. The implementation assigns all slots,
checks an explicit visited-edge array, verifies endpoint degree1 and all
actual marginals, and retains per-class/per-orbit aggregate records.
It does not substitute a count formula for checking the finite edge IDs.

These graphs satisfy the two tested load inequalities
M>=2Darr^2 Delta and M>=L Darr. They **do not satisfy2L<=Delta** from the
primary recursive setup. Thus they are deliberately scoped finite packing
units, not admissible full active recursion calls. They do not implement
the conditional-expectation labeling algorithm on arbitrary graphs.

## Local target-split regression

The separately frozen [mode addendum](MODE-ADDENDUM.md) uses the same F_5
class, L=3, integer Cauchy grid{-3,...,5}, T=10, and C'(0)=-720.
All nine Cauchy entries are computed by checked exact integer division;
their column0 is[-36036,-32760,-30030]. The label field F_5 only chooses
groups; coefficient arithmetic is over the integers modulo3^p. These are
different roles and are not conflated.

Across all64 pinned pairs X,Y in{0,1}^3 and each p=1,2,3:

- Correct source/target-split ordinary coefficient enumeration matches
  the direct affine product in all192 cases.
- The intentionally incorrect operation imposing one source-mode exponent
  at both crossed target-group coefficients disagrees in14 of64 cases
  modulo27. It happens to agree on all cases modulo3 and9.
- The frozen X=(1,0,0),Y=(1,0,0),p3 witness gives direct9, split9 and
  incorrect unsplit0 modulo27. Its surviving split degree-one term is9.

This rejects that specific unsplit contraction operation. It does not
claim that sharing a global polynomial variable and evaluating the full
product directly is invalid. The bad step is forcing its one exponent
at two separate target-group coefficients while evaluating its source
only once. Splitting the variable by target group restores the required
one-target structure in this fixed local identity.

All crossed entries in this matrix are checked3-divisible before
truncating total mode weight below p. The p values are arbitrary local
test moduli, not the derived full Hermite call precisions. This unit does
not test3-adic Hermite interpolation, recursive precision growth,
general source tiles, larger groups or full contraction semantics.

## Protocol, evidence and reproduction

[PROTOCOL.md](PROTOCOL.md) and [MODE-ADDENDUM.md](MODE-ADDENDUM.md) were
frozen and hashed before implementation/execution. Primary read:
AppendixD.4.2-D.4.5, printedpp58-63, in the pinned OpenAI October25,2026
preprint _The Exponential Time Hypothesis is False_. The source remains
the binary construction; the ternary gadget tested here is supplied by
the parent research task, not a claimed theorem from that preprint.

The source PDF/text/protocol hashes are in
`protocol-and-source.sha256`; source paths there preserve original local
provenance. `files.sha256` binds the complete local unit payload. Native
binaries are outside this directory. No Python research computation or
existing Rust backend is used.

Run from a copied unit directory with a C++17 compiler:

```sh
bash check.sh /tmp/next-ternary-unit-replay
```

The script rebuilds both sources under-O2,-Wall,-Wextra,-Wpedantic,-Werror,
executes the fixed panels and compares all12 retained output files
byte-for-byte. Both first strict builds/runs and the exact repeat pass;
compiler identity, commands, source/binary hashes and exits are in
`build-receipt.txt`. No compiler or runtime failure occurred. All negative
controls and wrong-unsplit mismatches are retained as specified tests.

Core records are under `records/`: complete class/orbit inventories,
marginals, graph coverage, source tiles, all F_67 pairs, negative controls
and summary. Optional local-mode records are under `mode-records/`:
the exact matrix, all192 cases including every wrong-result coincidence,
the explicit witness and summary. Counters are correctness diagnostics;
no elapsed time is used as a result.

[Core](review.md) and [local-mode](mode-review.md) read-only reviews inspect
source and records; they do not amount to a formal proof certificate or
independent replay of every operation. Neither found a blocking defect in
the reviewed finite unit.
The original research goal and all global algorithmic/complexity claims
remain unresolved by these units.
