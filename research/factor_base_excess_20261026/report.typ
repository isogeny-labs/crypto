#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt, font: "DejaVu Sans")
#set heading(numbering: "1.")
= Factor-base product-profile screening
2026-10-26 · implementation and literature-led next experiment\
*Status: no new research panel executed; factor-base improvement unestablished.*

The user requested continued work toward a useful factor base. The current
lead atlas points to linearization excess as an unexamined transfer question.
This change adds native kernel-based progression construction and exact
profiling to `factor-base-sweep`; a score does not establish successful solving.

#image("method.svg", width: 100%)

== What the command computes
Input: an explicit binary field modulus and independent polynomial-basis
columns of an abscissa subspace V. Output: canonical bases for product spans
V, V^(2), ... through the requested power, their dimensions, a basis-invariant
subspace identity and the two-summand expression
`e = n - dim(V) - dim(V^(2))`.

The modulus is checked for irreducibility; invalid/dependent inputs are
rejected and output is write-once. `construct` solves trace constraints,
XOR-samples a scale and records the kernel/mask/derivation. Seeded draws do
not imply uniform coverage of the entire kernel. Field structure through
degree 571 is supported, not wide point decomposition. Product dimensions
do not prove valid splitting; partial counters cannot rank whole methods.
Predicted degree, regularity and cost winner remain null.

== Why this lead, and what is already known
The sibling PDP study reports a low-error empirical two-summand degree fit
over 656 bases. That is historical evidence from its own solver, not a native
measurement here or a rule for m=3/4. We read its definitions, fitting scope,
projection policy and failure limits; none of its Python experiment code ran.

Bachoc–Serra–Zémor, Theorem 3, characterizes minimal product growth in finite
prime-degree extensions under dimensions at least two and product dimension
at most n-2. Geometric progressions are known structure in that range, not
a new discovery. The theorem proves no solver-complexity or regularity bound.
The full source/version record is in `LITERATURE.md`.

#pagebreak()
== The pending experiment
Pair high-excess and median-excess candidates with random references. Use
exact subgroup-qualified inventories and separately record x-counts, actual
points and distinct signed projected columns. Equal subspace dimension alone
does not match factor-base size. Training selects; fresh holdout tests the
selection; conditioned controls never count as random yield.

Primary exploratory model: `icv1-f2m31-tm90707-c95f16f5`, r=1439393,h=1492.
Secondary model: `icv1-f2m19-t797-b6cf2467`, r=130873,h=4. Registry field and
generator records are named in `PROTOCOL_DRAFT.md`. Degree 31's Frobenius
module differs from 83/131; it cannot replace the required degree 83 gate.

The first native comparison is m=2. An m=3/4 follow-up needs a native adapter,
valid exact descended systems and its own controls before a runnable protocol
exists. No old XP card is promoted or dispatched.

Acceptance needs complete charged successful decomposition work and useful
independent relations, including construction, scoring, failures, verification
and rank. The same native-operation pricing band applies to every arm.
Unemitted counters prevent a total-cost ranking. Fastest claims additionally
owe matched external solvers. There is no end-to-end, rho or speed claim.

== Concrete execution blocker
Current AGENTS requires an S3 resume checkpoint before an experiment starts
and tells Codex not to launch if `scripts/checkpoint.sh gate` fails. The
existing user restriction forbids S3 operations. The local gate returned 2
because no resume file/checkpoint exists, before any upload was attempted.
No experiment ran. This is an authorization/policy conflict, not evidence of
an unavailable mathematical route or a failed cloud credential.

The implementation and correctness validation can be reviewed now. New
experiment execution requires checkpoint authorization or an explicit
owner-approved alternative. The broader goal remains open.

== Sources and evidence
- Native command: `src/bin/factor_base_sweep/profile.rs`.
- Current atlas: `LEAD-ICE-linearization-excess-fb-prior`.
- Sibling source: `cryptanalysis:experiments/pdp-degree-heuristics/README.md`.
- Primary theorem: arXiv 1501.00602v2, pp. 1–3; DOI 10.1017/S0305004117000044.
- Full source assumptions and correctness outcomes: `LITERATURE.md`, `VALIDATION.md`.

Canonical dashboards and sealed research are unchanged: no new measured
solver result, Lean proof or identity certificate is claimed.
