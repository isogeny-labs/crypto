#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Supplied-basis collection for RR/F qualification
12 November 2026 · Native development checks · Producer-unverified

The native collector enumerates rational assignments of supplied equations that
reduce through affine pivots and univariate quadratics, validates each assignment
through the input/extractor, and unions signed decompositions. Unsupported,
free-variable and capped residuals remain unknown. Twelve native tests pass.
Claim PCC-20261112-72ed05 is unverified.

#image("collection-flow.svg", width: 100%)

== Supported scope and provenance boundary
The collector validates the RR/F arm, exact prime field, variable order and
input digest before enumeration, including empty results. It records the basis
digest. Constants can prove the supplied equations inconsistent; quadratic
branches enumerate distinct rational roots and affine pivots reconstruct the
other coordinates. Resource caps and any unresolved branch produce unknown,
never a truncated complete answer. Normalization errors remain visible.

Completeness is relative to the supplied equations. The collector does not
certify that this basis generates the original input ideal. Its output always
marks original-input completeness uncertified. A forged unit basis therefore
cannot establish original-input UNSAT. Exit zero alone does not admit a timing
comparison or prove input completeness.

== Development checks
The archived p1151 m3t4 F output contains affine relations and one quadratic.
The corresponding input and public metadata are reused, after removing the
planted coordinate map, support x/y/w values, separator values and seed data.
Both rational assignments are recovered and normalized without rerunning msolve.
Synthetic fixtures compare affine/quadratic assignments against exhaustive
small-field evaluation and distinguish contradiction, unsupported coupled or
higher-degree cases, resource caps, and partially supported branches. Negative
header, input-hash and empty-arm cases are checked. Read-only review found no
remaining concrete bug after the metadata-purge and arm-validation fixes.

== Remaining original objective
Before matched F4 cost measurements, bind each basis to its exact source-built
solver execution and qualify complete input sets against an independent reference.
General unsupported forms and excluded charts remain open. This addition does
not measure or predict retention of RR's lower solving degree, smaller matrices
or lower complete cost against F.

Checkpoint upload transport and live shared budget/pool readback remain absent.
Existing GitHub experiment and fixed-diagnosis workflows were inspected; neither
provides a suitable read-only checkpoint/budget preflight. No workflow, build or
paid experiment was dispatched. A nonsecret existing runner connection was
requested while native work continued. The frozen-ledger merge dependency in
PR 3835 is also unchanged.

== Artifacts and sources
COLLECTION-20261112.md specifies statuses and commands; the final native log is
checks/collection-tests-final-20261112.txt. Source: p1tool/src/collection.rs.
The existing P1 input/basis archives and parser define the concrete format.
#link("https://github.com/algebraic-solving/msolve")[Official msolve documentation]
distinguishes reduced grevlex basis output from rational assignment collection.
KB remains unavailable. Historical reports/scoreboards are unchanged.
#link("https://github.com/isogeny-labs/crypto/pull/3835")[PR 3835].
