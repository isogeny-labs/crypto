#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
#set heading(numbering: none)
= R62: complete square decomposition and generator route
Exact Koblitz modular-carrier field construction. Source field: the retained
R61 degree32 extension of F2(u), with generators lambda and v. These are
whole-source identities, with no scalar-fiber or series approximation.

== Recorded computation
The matrix construction binds all 32 standard monomials and squared columns
and checks both inverse products. The repaired complete cohort checks435
whole-source decompositions and13,920 rational coefficient parity identities.
The fresh source replay reloads every input and root, binds all 435 functions
to R61 and rechecks the complete operator and negative controls.

#image("cohort62.svg", width: 100%)
Recorded counts from split62-02; all 435 rows, seed62, one exact source instance.
Zero odd components are field-operator outputs. Their complete supported
divisors and global differential regularity are separate obligations.

== Terminal receipts
#table(columns: (2fr,1fr,1.4fr), inset: 4pt,
[Attempt], [Status], [Native wall seconds],
[check62‑01], [accepted], [68.0],
[direct62‑01], [accepted], [87.0],
[kernel62‑01], [accepted], [282.0],
[matrix62‑01], [accepted], [4.0],
[rustcheck62‑01], [accepted], [0.0],
[split62‑01], [rejected], [84.0],
[split62‑02], [accepted], [104.0],
[square‑lean62‑01], [rejected], [not recorded],
[square‑lean62‑02], [rejected], [not recorded],
[square‑lean62‑03], [accepted], [not recorded],
)
Wall times are native receipt intervals, not controlled performance comparisons.
Lean compile and audit intervals were not recorded by the proof driver.

#image("pipeline62.svg", width: 100%)
Source identities, complete kernels and roots are separately checked stages.

#pagebreak()
= Complete square subspaces and generator route
#image("kernels62.svg", width: 100%)
All76mixed and324 cubic functions are retained in the complete F2 kernel.
Native checked denominator clearing and binary row reconstruction precede
rank/nullspace computation; every resulting root is checked in the full source.
The independent Rust checker replays both full binary matrices and kernels.

== Direct generator route and proof scope
For a grouped characteristic-two source polynomial,
$ P(t)=a^2+u b^2+t c^2+u t d^2 $.
Put $q=c^2+u d^2$.
With $P(t)=0$ and a checked nonzero derivative/inverse,
$ t=((a c+u b d)/q)^2+u((a d+b c)/q)^2. $
The direct route uses pair arithmetic rather than reading the squared-basis
matrix. It checks the actual source grouping, derivative inverse, both
source generators, all 32 generated basis pairs and all 435 functions, and
compares 870 roots with the matrix output. The rational scalar parity helper
is shared; this is not a separately implemented scalar parity algorithm.

== Lean statements
The pinned Lean 4.34.1/mathlib d13f23b toolchain checks twelve authored
claims: rational parity, uniqueness under nonsquare-u and characteristic-two
premises, the zero-odd square criterion, five cyclic5 norm/projector/control
claims, three pair arithmetic identities and the generator formula. Native
field/model and nonsquare-parameter inputs, normality, local valuations,
complete holomorphic regularity, actual deck/Cartier action and target-factor
action are not inferred from these abstract implications.

== Failures retained
The first complete 435 attempt used unsupported ideal equality and is rejected
in full despite a late marker. The first Lean compile failed on a lemma name
and numeral-cast normalization; the second lacked a Ring tactic import.
Both artifacts are rejected. All later failures,
if present, are listed in results.json with their actual errors. An early
raw-index mistake was corrected without deleting local bytes or rewriting
Git history. Accepted producers were not repeated for publication.

== Scaling and next construction
GENERATOR-LEAD62.md and SCALE112-DESIGN62.md bind the proposed larger rung
to the marked23 degree 7 base followed by the degree 16 lambda extension.
The larger field has degree 112;991 denotes the selected factor action.
Next: apply the generator operator to this exact larger source and complete
the normal-order/infinity bridge needed for the full 61-dimensional
holomorphic space. Unit divisors/saturation, actual primitive cyclic block,
Hecke/order-11 action, proper polarization, quotient saturation and arbitrary
reverse divisors retain their separate obligations.

== Reproduction and provenance
PROTOCOL.md, protocol.json, donors.json and per-attempt source revisions
freeze the affected inputs before data. results.json verifies historical
source bytes. Native limits: 1200seconds, 4 GiB virtual, one Singular/FLINT
thread, seed62, one math job. Lean: 1200seconds separately per compile/audit,
CPU0, -j1 -M4096, unlimited virtual address space. No timing claim.

Sources: R61/SQUARE-DECOMPOSITION61.md; R52/CM.md;
#link("https://www.singular.uni-kl.de/index.php/singular.pdf")[Singular manual].
