# Matrix report replay scope

The adapter decodes the existing bounded-matrix Report field set without adding
or renaming producer fields. The report, counters, reservations, polynomials and
monomials reject unknown fields. The serialized report is capped at 64 KiB;
there are at most eight output rows/columns and 61 original equations in the
three-selector domain. Terms must be unique and in native canonical order.

The checker receives the expected engine, degree, original resource limits, and
original equations already authenticated and independently verified against the
original curve inputs. It checks report identity and bounds, event-count ordering,
completion/rank consistency, and F4's NotRequested criterion status. Full budget
reconciliation across construction, matrix and adapter work remains a separate
pipeline-result obligation; a report within the original cap alone is not that
reconciliation.

For all eight Boolean assignments satisfying the original equations, every
returned row must also vanish. The report explicitly returns the count of such
assignments: zero makes this condition vacuous, not evidence of derivation.
No ideal-membership, full row-space, matrix execution, instruction-count,
PDP completion or fall-degree claim follows. Report completion is a checked
producer report, not an independently verified native execution.

The controls use actual F4 and F5 reports for the fixed positive construction
fixture, require a nonempty original zero set and nonempty rows, reject an
inserted constant-one row, and reject reserialized schema/domain/metadata
mutations. Retain an original failure rather than replacing the fixture.
