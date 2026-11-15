# Constructor-bound root diagonal pivots

2026-11-02. Derived implementation and static integration; numerical,
corruption, heap and performance trials: zero. Fixed public weights unchanged.

## Source construction and provenance

build_bound freezes the supplied weights into one tuple and passes the same
values to the preserved public companion constructor and compact diagonal
builder. It constructs the source Operator, the geometric metric and its
inverse, and a RootBinding. The original constructor still performs its full
field-sized setup; this component does not remove that cost.

The binding records source identity, meter, dimension/wrap/prime signature,
immutable generator snapshots and the two metric tuples. At root compression,
prepare requires that the source still matches these snapshots and independently
checks the compressor's factorization certificate. It then replaces its
snapshots with the checked output generators. Equal displacement at the same
dimension and distinct wraps determines the same matrix. choose requires the
new Operator to match these checked generators, and requires the original
unpermuted metric view. This binds the diagonal proposal across compression.

States are fresh -> reserved -> ready -> consumed; zero compressed rank takes
reserved -> consumed directly and leaves the old zero-source terminal intact.
Snapshots and diagonal/metric references are released at terminal consumption.
Numerical verification failures cannot re-enter a fresh binding. These are
internal provenance rules, not tamper-proof objects against reflection or
concurrent malicious mutation. The runtime gate is implemented but unexecuted.

## Root proposal and checked column

The compact diagonal gives A_ii=di_i*(C-Delta_z) for z=(i+1) mod r<q,
and di_i*C otherwise. Scan exceptional root indices 0,...,q-2, then use
index q-1 if C!=0, then consider index r-1. This finds the first nonzero
diagonal in the original coordinate order without scanning the constant bulk.
It needs at most q subtractions and q+1 native nonzero tests.

If it finds index i, query A e_i once and require the complete column canonical,
its diagonal entry nonzero and exactly equal to the compact prediction. A
mismatch raises an error. The accepted scalar principal block is invertible.
The existing simultaneous transport, Schur operations and compact lifts apply.
The prediction check costs one field product using the stored inverse metric;
no new exponentiation or inversion is needed for the proposed diagonal value.

RootSelection is a separate record type accepted by a minimally changed paired
transport wrapper. It has zero_prefix=0 (an empty proven-zero set) and
first_column=None (the first nonzero column was not searched for). It never
claims skipped columns are zero. Since the driver sees the empty prefix, this
record cannot enter the zero-prefix deletion/rebinding path. Later iterations
use ordinary full selector records. The wrapper changes only the admitted
record type; all transport formulas and guards remain as before.

If all predicted diagonals vanish, choose invokes the original selector and
returns its ordinary record, retaining scalar/pair/zero behavior. The global
weighted-symmetry assumption remains established by the source construction;
local entry agreement alone is not a symmetry certificate.

## Integration and scientific comparison

public_solve.solve_public(r,q,gamma,weights,meter,events) is the dedicated
constructor entry point. driver_root.solve_or_witness accepts an optional
root_binding; without it the original path remains. The binding is prepared
only on the first compression and cleared after the root selection. It is
never applied to a permutation or Schur child. Early witness rank-unknown
semantics, root equations, zero-coordinate deletion and all later solver
branches remain in the complete declared-substitution AST comparison.

The proposal deliberately can choose a different root pivot from the old
first-nonzero-column selector: a later scalar may replace an earlier pair.
Subsequent Schur dimensions, transport and pivot sequences can therefore differ.
Compare root equations, normalized witness pairing and appropriate rank facts;
do not demand byte-identical nonunique solutions/witnesses or matching pivots.
Feature-annihilation checks remain separate. No rank-profile property is claimed.

## Costs retained and added

The root scalar proposal uses one source query at the compressed root width t.
The all-zero-diagonal fallback retains the original selector's queries and
adds the proposal overhead. This is not a uniform end-to-end query reduction:
a different pivot tree changes downstream work.

Setup includes the original full compiler; the previously derived O(q^2)
compact-diagonal build; a copy of the input weights; and r-1 products to build
the forward metric (the compiler already supplies its inverse). Original and
checked generator snapshots require 2ns and 2nt retained field entries in
successive phases, plus transient copies made by equality checks. These costs
must not be omitted or presented as a total-memory improvement.

The additional independent root compression check has derived logical integer
products 2ng(s+t) and modular reductions n(s+2g+t), where g is its first rank.
Its shape/minor/canonical checks and sums remain native work. These expected
schedules are separate from Meter field arithmetic; runtime counts are not
verified. Compression production itself is unchanged.

At proposal time: at most q field subtractions, q+1 nonzero tests, one field
product and one full source query on the scalar branch. Canonical column,
generator snapshot, metric value and coordinate-label checks add O(n) or
O(ns+nt) native work and allocation. Fallback repeats the ordinary metric-view
entry check as well. No elapsed-time, heap or net arithmetic advantage is measured.

## Static checks and funded validation

Two complete AST comparisons cover the paired transport wrapper and full
driver with declared substitutions. Static checks verify RootSelection's empty
prefix/unknown-first-column defaults and compression verification before the
ready state. All Python sources parse. The report is rendered and inspected.
No producer, checker or numerical solve is executed in this round.

Retain the full existing cohorts. Add first/root-bulk/final exceptional scalar
proposals; all-diagonal-zero pair fallback; q=1 and q=r-1; zero-source terminal;
nonconstant metrics; altered weights, generator entries, wraps, meter/prime,
compression factors/output and predicted entries; consumed binding and wrong
view; scalar-versus-pair pivot changes; both root outcomes and rank semantics;
independent final equations, query widths, checker counters and lifetimes.
A well-formed alternative compression of the same displacement is valid if
its certificate passes and the constructed operator matches it.

Use the existing sibling paths from companion-metric-solver/DERIVATION.md,
plus feature-diagonal-events, companion-selected-reuse, companion-zero-delete,
companion-metric-views and this directory. The new entry is public_solve.solve_public.
No frozen reference is overwritten. Existing execution/cost admission remains.

Next derive a representation for Schur diagonal corrections that can be
updated from accepted pivot columns. It must preserve principal-block metric
closure and charge any retained vectors; the root compact formula cannot
simply be reused at a child.

## Sources reviewed

Local feature-diagonal-events/DERIVATION.md and implementation; preserved
public constructor; companion-selected-reuse driver and compression checker;
companion-zero-delete paired transport; immutable metric views.
Dumas and Pernet (2018), introduction/Section 2:
https://arxiv.org/pdf/1802.10453 . Scalar/pair finite-field symmetric pivots are
established prior art. This proposal does not inherit their rank-profile
strategy, asymptotic guarantee or experiments.
