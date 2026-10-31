# Audit gate passes; source-block oracle scope corrected before use

`external-audit-v2` compiles its test binary and passes 75 native tests,
zero failed or ignored. It was then explicitly stopped while its busy
wrapper waited to invoke the runner build. The inspected service cgroup
contained only the shell driver and waiting Python wrapper; no runner
compile or external-basis audit had started. Original test outputs,
source hashes and the stop reconciliation remain in its run metadata.

![Independent verification obligations](../../figures/joint-mechanism.svg)

Source review caught an oracle-scope error: a basis for a single source
block must be compared with the subset of physical relations admitting a
source in that block. For mixed arms this is the image factor's source.
For pure L0, the same physical four-tuple can have several source
presentations; the actual positive fixture has presentations in all three
blocks. Comparing a block with an unrestricted oracle could falsely label
a complete block as incomplete.

The correction is accompanied by a test using actual p37 public points
and frozen independently audited pure/phi relations. It verifies their
group sums, the pure tuple's three allowed blocks, and the phi tuple's
single T=10 source block. The fresh version-3 gate passes all 76 native
tests, zero failed or ignored; the saved test execution reports 8.47 s,
which is an unadmitted diagnostic and not a solver speed comparison. No external
result is accepted from the stopped version, and no msolve computation or
signed-set oracle is repeated by this correction.

The independent external checker separately verifies input reductions and
all S-pairs, exhaustively branches over the basis's Fp variety, decodes
physical factors and signs, checks group sums and independently reduces
matrix rows. Input containment and S-pairs do not establish reverse ideal
containment. A dedicated false-unit-ideal control preserves that limit;
complete physical sets require equality with the independent group oracle.
The software gate is not itself a successful external solve, complete
collector or timing result.
