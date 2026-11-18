# Packed projected S3 rows for index-calculus relation finding

User scope: ECC2K-130 index calculus only. Replace one component of the
existing algebraic trace-zero/subspace relation finder. No change to another
DLP algorithm, factor-base population, accepted relation, or target query.

## Fixed computation and cost chain

The current merged kernel is the reference, byte-bound in baseline_snapshot.rs.
It uses t=y+zH(z) for an actual finite trace-kernel residual(z,y), then processes
v=t+zH(u²)+epsilon*z with u in V and v in W=span(VV). Projecting modulo W
produces coefficient rows with d+1 unknown bits, thirteen at d12. The full
numeric kernel conditions, complete W remainder check, monic roots, supplied
point signs, original model/order/caps and original-group replay stay fixed.

C_old = setup + complete_oracle + post/replay/output.
Replace ONLY evaluation of the fixed binary-linear coefficient/RHS maps in
the complete oracle; charge every new per-query table build and evaluation.
C_new = the same three boundaries with identical answers. No regrouping,
removed common parser, cached base, symbolic preparation or batch tail is a gain.

## Exact replacement

For widths <=16, pack the first up-to16 coefficient rows into four u64 words,
four disjoint16-bit row lanes per word. For each four-bit input chunk, tabulate
all16 XOR combinations of its basis images. RHS maps have a separate u16
image at the same input-chunk addresses. For z and independently for t, XORing
one table entry per chunk is exactly the original F2-linear map. The quadratic
computation t=y+zH(z) itself is unchanged and paid before this evaluation.

Tables cover first16 rows only. Later rows and widths>16 use original scalar
parity evaluation. Row order, elimination, rank, reconstruction and full W
membership remain identical. Unused high bits in the last chunk have zero
images. Tables are prepared only when the existing projector is needed;
immediate first-residual cancellation does not initialize the projector.
Data payload is ceil(n/4)*16*(4*8+2) bytes:17952 at n131,11424 at n83,
excluding existing maps/vector headers. This is a storage formula, not peak RSS.

Validation before primary: compare every input basis bit plus independent
random pairs at widths1/13/16 against scalar parity; widths>16 decline the
map. Force sixteen zero leading equations so actual small S3 systems exercise
the late scalar-row path, and compare packed/table-free answers. Preserve all
65 prior decisions, witness indices/order, models, common/projected diagnostics.

## Controlled comparison

Three arms: baseline, lookup, baseline-aa. One executable, same compact query
bytes, curve/context/field code and lazy symbolic context for every arm.
Six fixed d12 cells: n83/n131 ×planted-0/uniform/cancellation, five repetitions,
90 primary rows. Rotate arm order by field/kind/repetition. These are disclosed
anchors, not fresh heldouts or a population sample. A/A measures repeatability
and branch nuisance, not statistical independence. Native Rust analysis reports
all cells and five-repeat min/median/max, without population p-values/CI.

Predeclared criterion: BOTH uniform cells need lookup/baseline oracle Ir<=0.95
and whole cold-query Ir<=0.99; all exact outputs/phase sums agree, all six A/A
median discrepancies<=1%, and no other cold cell regresses>1%. Preserve failed,
partial or inconclusive results with no threshold relaxation or trial deletion.
All planted/cancellation controls must Found/replay; uniforms retain separate
arithmetic negative proofs. UNKNOWN is never a refutation certificate.

Every execution has fresh native-tally and tally-disabled Callgrind processes.
Whole Ir includes parsing, curve/context/base construction, admission, every
per-query map/table, all residual/solver work, roots/signs/replay and exit.
Setup/oracle/post dump sums are whole Ir; final-binary phased/unphased positive
calibration must agree within0.1%. Field tallies are separate; inversion is
informational. No native-time, fastest-external, pipeline or density inference.

Persistent local exception: CPU0,2GiB,600seconds combined final calibration,
pilot/freeze/primary, zero paid resources/retries. Twelve-call largest-class
pilot: n83/131 ×planted-1/uniform ×three arms. Every call valid and planted
answers Found/replay. Runtime estimate uses max combined pilot cost per
field/arm times15 primary calls plus actual fixed overhead. Stop if infeasible;
no silent sample reduction. Push source before pilot, freeze/push protocol and
native analysis before primary. Preserve all outputs and failures.

## Build qualification and identities

Current main SAT source has unclosed delimiters; the broad library check also
fails in hashed-ecdl-tensor's missing metrics module. The focused build uses
current numeric curve/field/trace-kernel/S3 modules and current unchanged report
interfaces, with byte snapshots of SAT and chain support from d7a006293.
Those support solvers are not executed by this numeric relation backend. Both
comparison arms share exactly this qualified backend. No success of a current
whole-root/SAT build is inferred. Failed initial/facade/workspace builds remain.

An older retained-binary discovery profile is historical: current main adds a
missing square tally tick in general F2m code. A second profile uses the fresh
qualified current-arithmetic baseline. Old studies/source pins are untouched;
new results never treat the old field ledger as the current ledger.

Records identify ECC2K-130 as icv1-f2m131-tm22283658519494248867-115e0dc5.
The matched n83 E0 fixture is icv1-f2m83-tm6151469093347-debefd74. It is used
because this full trace-kernel model supports E0; standard-panel K1-83 is E1.
The standard ECC2K-130 target remains K0-131. Legacy input IDs/paths are retained
byte-exact from the prior workload, including their exact point order.

Sources checked (KB): unavailable, wrapper127. Repository fallback:
binary_semaev S3/Artin–Schreier, certicom131_symlp/sym2 subspace/linear maps,
gf2_elim Four Russians tables and known setup regressions, partial_enlargement
combination tables, prior trace-zero source/results. This adapts an established
binary-linear lookup technique; no novelty or Z[tau]-module claim.

All arithmetic, solver checks and statistics are native Rust. Python only
binds/packages byte metadata or lays out recorded values. Publish source,
rows, separate polynomial audit, strict grid audit, canonical export, one
reproducible raw archive, editable graph/report/PDF and unsigned PCC standing
in a regular continuation PR. Existing benchmark/runtime/scoreboard numbers
remain historical until their independent/full-cost admission requirements hold.

Prose-only companion to the immutable original PROTOCOL.md pinned by675806; wording changes no experiment, equation, threshold or accounting boundary. [pcc:unverified]
