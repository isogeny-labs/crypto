# Same-input open-CAS correctness baseline

2026-10-25, frozen before exporting or solving these cases. Local Singular
4.3.1-p3 was installed in a workspace prefix from official Debian packages,
with each package SHA-256 checked against the package index. Its package
manifest and version are retained. No system policy or library was modified.

Select from the original degree-two structural panel, before new outcomes:
n=13,19; ell=2; all four families; base replicate 0; target index 0. Also
select ell=3 geometric bases at those same n, replicate and target. Export
both eliminated S4 and symmetric-e equations, with exact equation hash
equality to the frozen raw row: twenty cases. Add two explicit positive
algebraic controls, using the first n=13/ell=2 prefix basis and x_r=basis[0],
one per formulation. These constructed controls need not be subgroup queries.
No choice is made by measured timing or solver success.

Exporters and validators are native Rust (ic_cas_correctness). Independently
enumerate every Boolean assignment, up to 20 variables; verify the existing
complete native extraction count and each fresh native solve against that
enumeration. Append all Boolean field equations x_i^2+x_i to the exact same
input ideal for Singular. Use dp order and option(redSB). Run both std and
slimgb, single-threaded, 60-second and 1-GiB limits per command, retaining
errors, caps, stdout/stderr, resource receipt, exact scripts and their hashes.

The Rust validator parses every external basis polynomial, checks the CAS
normal-form certificate for the original generators, and enumerates its full
Boolean zero set. Require exact equality with the independent native zero
set. All field equations are explicit, so this finite check establishes
equality of Boolean vanishing sets for these cases. It is not a general
formal Gröbner-basis proof or an equal-output performance comparison between
a complete basis computation and a root extractor.

This round is correctness only. Internal CAS timers and process resource
times are diagnostics at L0, and no speed ratio, fastest-tool claim or
hardware calibration is admitted. Magma is not licensed in this workspace;
msolve/FGb and other fastest-tool comparisons remain open. A successful
Singular baseline does not discharge the performance requirement or larger
field/guessing/complete-DLP obligations.

Freeze source before execution, retain exact source/package/input/binary
hashes, native results and all external failures, and deliver a descriptive
report, graph/diagram and inspected PDF. No paid provisioning. This small
correctness check does not alter or rebuild the active frozen instrument.
