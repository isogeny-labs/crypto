# Draft protocol: arity-three prime-field CDCL(T) exponent screen

**Status:** hardened draft, not frozen and not scientifically run. The native
development oracle merged in `cryptanalysis#1347`; the production propagator,
certificate route, complete accounting, power source and held-out freeze are
still missing.

## Question and complete-cost gate

At fixed arity `m=3`, does hybrid CDCL(T) with a source-bound `F_p` theory
achieve natural point-decomposition query work `Q(s)=2^(e s)` while reusable
setup costs `S(s)=2^(a s)`, with corrected upper intervals strictly below
`e=0.5` and `a=1.5` on the complete verified panel?

Relation collection has `Theta(2^s)` natural queries, so its work is

```text
S(s) + 2^s Q(s) = 2^(a s) + 2^((e+1)s)
```

up to polynomial factors. Both gates are necessary. This screen measures a
frozen finite-range solver/representation bundle; it cannot prove a
subexponential ECDLP algorithm.

## Semantic instance

One native generator emits a canonical typed IR. Every exporter consumes those
same bytes; no arm regenerates the mathematical instance. The predicate uses:

- a nonsingular `E/F_p` with known odd prime whole-group order and a rational
  non-2-torsion target;
- three disjoint abscissa windows and exactly one selected abscissa per window;
- a rational lift sign bound to each selection;
- two chained `S3` relations through the quadratic lambda lift, with no
  expanded `S4` polynomial;
- every exceptional chart, including `t=0`, represented explicitly; and
- independent signed group-law replay of every model.

Chained `S3` is sign-blind and cannot by itself establish the signed point
relation. The nonexceptional lift equation
`(2t lambda-s0)^2=f(x)` has discriminant `16t^2 f(x)`. Under the stated
odd-prime-order, target and disjoint-window guards, the three-summand
Frobenius argument makes every accepted lift rational. Arity four and higher
retain explicit rationality circuits.

The complete derivation and provenance are in
[`PDP_ARITY_CORRECTION.md`](PDP_ARITY_CORRECTION.md).

## Boolean and range encodings

For `s=6..14`, exactly-one is Sinz sequential at-most-one plus one
at-least-one clause. A window of size `n>=2` has `n-1` auxiliary variables and
`3n-3` clauses. The cryptanalysis development oracle exhaustively checks this
construction through `n=8`.

Field/index membership is the compact semantic predicate
`0 <= word < domain`, exported as an unsigned comparator or native theory
check. One clause per forbidden word is allowed only below a frozen cap that
admits the complete enumeration.

Pure CNF is an encoding-cost control. Its multiplier, gate basis and modular
reduction are frozen before data. The selected schoolbook multiplier may have
`Theta((log p)^2)` gates by construction; this is not a general Boolean
multiplication lower bound. Actual gate, variable, clause and byte counts are
reported.

## Ladder, classes and continuation

The initial ladder is every integer `s=6..14` at `m=3`. The regression
coordinate is actual recorded `log2(p)/3`. At each rung generate eight
independent natural, eight planted and eight exact-UNSAT instances before arm
labels are assigned, for 216 semantic instances. Seeds derive from the frozen
protocol hash.

The selected one-hot encoding has these exact structural sizes at `s=27`:

```text
402,653,184 selector variables
402,653,181 Sinz auxiliary variables
805,306,365 total Boolean variables before theory auxiliaries
1,207,959,543 exactly-one clauses
```

Thus `s=15..27` is not an admitted continuation. A prospective amendment must
either pass a measured resource preflight for this exact representation or
freeze a compact/lazy selector after complete small-cell projected-model-set
equivalence, A/A, setup-exponent, serialized-size and peak-RSS preflights.

## Arms and eligibility

| id | role | requirement |
| --- | --- | --- |
| `enum-ref` | exact reference | two-window projected-model oracle where complete; `e=2` cost control |
| `f4-base` | algebraic baseline | identical polynomial IR to a pinned F4 backend |
| `cdcl-t-fp` | sole slope candidate | CaDiCaL skeleton plus source-bound `F_p` theory and checked explanations/proof |
| `qf-ff` | diagnostic | cvc5 QF_FF SAT models replayed; UNSAT is ineligible at the inspected source |
| `cnf-cost` | encoding control | complete selected schoolbook bit-blast and size/proof ledger |
| `cdcl-t-aa` | A/A | byte-identical fresh-process CDCL(T) run |
| `planted-positive` | positive control | witness, sign binding and explanation path |
| `matched-null` | negative control | curve-free sparse system matched on field, variables, degrees, incidence, encoding and small-cell model count |

The `cdcl-t-fp` theory owns field arithmetic, inversion guards, chained
group-law constraints, sign binding and exceptional-chart consistency. Every
propagation/conflict returns a Boolean explanation replayed by an independent
checker. UNSAT additionally requires a log whose Boolean steps and admitted
theory lemmas both replay.

Inspected cvc5 source head
`02f77af402b4be82f27e5cc00b86b265a90dea3d` has no finite-field proof checker.
Therefore `qf-ff` is a SAT/model diagnostic outside `C-slope`. An unproved
`unsat` is `unknown`. A future proof-capable candidate requires a prospective
amendment before affected data; only then may a multiple-comparison family be
added.

## Controls and mutations

At every enumerable cell compare complete canonical projected model sets.
Elsewhere replay every SAT witness and require an independently checked proof
for UNSAT. Planted instances accept any valid decomposition, not necessarily
the planted tuple. Matched-null generation is bounded and fixed before data;
failure to produce the preregistered match invalidates the pair.

Mutation controls alter one field coefficient, admit one out-of-range word,
overlap two windows, corrupt one lift sign/root, corrupt one theory explanation
and corrupt one UNSAT proof step. The canonical evaluator and certificate
checkers must reject all six.

## Complete accounting

Each fresh process records non-overlapping stages:

1. **cold setup:** process start through target-independent IR construction,
   parsing, solver initialization and preprocessing;
2. **online query:** first target-dependent instruction through backend exit;
3. **certificate/replay:** extraction, explanation/proof checking and signed
   group-law verification.

Primary counters are complete cold instructions, setup instructions,
online-plus-verification instructions and peak RSS over the cold interval.
Also retain field-operation vectors, CDCL decisions/conflicts/propagations/
restarts/learned bytes and per-stage bytes. The stages must reconcile exactly
to the cold total.

The query exponent `e` is fit only from natural instances. Planted and
exact-UNSAT classes are correctness/control strata with separately reported
descriptive slopes; they never enter `e`. The setup exponent `a` is fit from
the target-independent stage once per generated predicate.

## Analysis and decision

Primary Theil--Sen regressions use actual `log2(p)/3`. Bootstrap resampling
units and one-sided intervals are frozen in analysis source before data. There
is one eligible slope candidate and therefore no artificial Holm correction.

**Advance:** every preregistered natural row at every rung completes with a
verified exact status; every planted and exact-UNSAT control completes and
verifies; the frozen power condition holds; upper intervals satisfy `e<0.5`
and `a<1.5`; both are outside A/A spread; and cold/stage ledgers reconcile.

**Do not advance:** the same complete verified panel and power condition hold,
and either corrected lower interval has `e>=0.5` or `a>=1.5`.

**Inconclusive:** anything else, including any natural timeout, OOM, unknown,
unproved UNSAT, incomplete rung, completed-row subset, failed positive
control, explanation/proof rejection, insufficient power or a favorable
conclusion obtained only by excluding setup/theory/verification work.

No imputation or favorable subset can satisfy the advance gate. Censored rows
remain in the export and may support a separately preregistered conservative
lower-bound analysis, but never this decision statistic.

## Freeze blockers

- production typed IR/generator beyond the landed development fixture;
- source-bound prime-field propagator and independent explanation/proof checker;
- pinned backend builds and QF_FF diagnostic/export decision;
- exact cold/setup/online/certificate instrumentation and reconciliation;
- frozen analysis, multiplicity statement, power calculation and resource preflight;
- immutable held-out manifest and protocol hash.

Until every item is present and the machine record validates and freezes in a
reviewed commit, only disclosed development-fixture validation is authorized.
