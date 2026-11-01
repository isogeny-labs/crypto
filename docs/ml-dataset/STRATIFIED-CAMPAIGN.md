# Stratified full-campaign progression (design draft)

This is a scheduling and accounting design for the requested ECDLP/PDP corpus,
not a frozen execution protocol or a spending grant. The authoritative
unexpanded target remains
`research/ml_dataset_20261028/full-campaign-plan.json`: 24 sizes (23–31 and
47–61) × 14 arities (2–15) × 3 families (Koblitz, other binary, prime) × 2
twist positions × 16 curve seeds × 8 generators × 64 keys × 10 methods =
**165,150,720 method cells**. At prime n=61 the modulus is exactly
2^61−1. Each position is retained even if curve construction or a backend is
unsupported. A terminal status is never silently replaced by another draw.

## Why stage the grid

Semaev's summation-polynomial construction supplies the decomposition systems;
PKM's prime-field approach and binary SAT/Weil-descent approaches motivate
separate method strata rather than one pooled success rate. The applicability
of last-fall bounds depends on the exact system and hypotheses. Double-large-
prime graph work must retain partial relations and cycle certificates, not just
a final relation count. These are design motivations, not performance claims:
[Semaev](https://eprint.iacr.org/2004/031.pdf),
[Petit–Kosters–Messeng](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
[binary SAT study](https://pmc.ncbi.nlm.nih.gov/articles/PMC7334981/),
[Huang–Kosters–Yang–Yeo](https://arxiv.org/abs/1505.02532), and
[Diem](https://arxiv.org/abs/math/0606607).

The existing 252-cell local compatibility matrix covers n=23, 47, 61,
all three families, both twist positions and m=2–15 with one `enumerate`
configuration. It found 68 verified PDP witnesses and 184 budget-exhausted
cells under its fixed small work limit. It is a software/ingestion check,
not a random sample of the full method grid or an independent scalar-recovery
panel; its build identity was explicitly overridden. Never use its completion
rate to price the campaign or estimate an ECDLP success probability.

## Deterministic expansion and quotas

The scheduler traverses one fixed, public, mixed-radix cell ID. Expansion
levels are nested; completed IDs are not recomputed. At each level, include
all ten named methods (`enumerate`, `bsgs`, `sat`, `weil`, `f4`, `f5`, `f6`,
`root`, `pkm`, `double-large-prime`) on the *same* curve/generator/key instance.
Keep factor-base choice and polynomial encoding as explicit sub-arm IDs with
separate artifacts and budgets; they do not silently multiply or replace the
ten-method plan.

| Level | Added axes at full width | Cumulative method cells | Purpose |
|---|---|---:|---|
| 0: compatibility | n=23/47/61, one generator/key, `enumerate` | 252 observed local cells | Parser, certificate and ingestion smoke only; outside the frozen full campaign |
| 1: breadth | 24 sizes, 14 arities, 3 families, 2 twists; one seed/generator/key, all methods | 20,160 | Detect unsupported and censored strata before replication |
| 2: curve variation | All 16 curve seeds | 322,560 | Estimate between-curve variation within each parameter stratum |
| 3: generator variation | All 8 generators | 2,580,480 | Test generator invariance and split leakage |
| 4: target variation | All 64 keys | 165,150,720 | Complete requested plan and key-level variation |

Level 1 is a proposed first *coverage* wave, not a promise that one grant can
run 20,160 cells. Every level must be partitioned into finite, pre-admitted
shards. If a level cannot be funded, its remaining IDs stay `not_attempted`
with a recorded admission reason. No lower level is relabelled as the full
campaign. To avoid a cheap-method or small-n prefix, order each admitted wave
by a frozen hash of `(campaign ID, level, cell ID)` and use balanced quotas
for `(n, family, twist, m, method)` before a second replicate in any stratum.
Publish the candidate ID list, hash seed, inclusion probability, and all
unselected IDs *before* results; never adapt the draw to witness yield.
Adaptive follow-up is a separate preregistered cohort with its own selection
probabilities, not a replacement for the fixed panel.

Curve seed, generator multiplier and key must be independent domain-separated
uniform draws with rejection sampling over the declared subgroup. Preserve RNG
version and rejection counts in restricted provenance. Prime and binary twist
pairs need independent group/order/generator certificates. An even-degree
binary `a+1` construction is not automatically a nontrivial twist; certify
the intended class or report `unsupported` for that cell. Assign all linked
models, generators, keys and both twists to one split component, and keep
whole n/family/field-polynomial holdouts for generalization checks.

## Admission and measured cost

Before **each** shard, freeze the complete protocol file hash, plan hash,
source/binary/image hashes, method configuration, exact cell IDs, randomization,
per-cell CPU/wall/memory/trace limits, retry policy and analysis. The existing
full plan has `protocol_sha256: null`; it cannot authorize generation. Pin a
current region-specific compute price and billable node-lifetime upper bound,
first-month S3 storage/transfer/request prices, current ledger obligations,
worker identity/grant and finalization reserve. Historical USD 200 language
and the old USD 11.6112 balance are not a current Ohio allocation.

Use integer micro-USD and checked arithmetic. For a shard, the admitted upper
cost is `node_seconds_ceiling × price_per_node_second + reserved PUT/GET/HEAD
attempts × their pinned unit prices + reserved output/storage/transfer bytes ×
their pinned unit prices + control/finalization allowance`. Round each term
up. Include retries, failed/ambiguous writes, restore reads and idle/billable
node time; solver CPU seconds alone are insufficient. The allocator must prove
`existing obligations + outstanding grants + new upper cost <= current cap`
and similarly for all request/byte/local limits before issuing a unique grant.
An unknown obligation or missing worker grant means no admission. The
native budget journal charges attempts before transport, retains ambiguous
outcomes, and never refunds on a mere timeout. Reconcile actual Job/Pod UID,
remote receipts and billing before a new wave. A dry-run cost estimate is
read-only and is not launch authority.

Per-cell records must keep separately: attempted and completed status;
construction and known-answer verification; PDP witness and lift verification;
relation collection/filter/rank; scalar recovery without witness; independent
scalar replay; per-phase wall/CPU/typed operations; byte/request charges;
counter provenance; and exact stop cause (`budget_exhausted`, timeout, OOM,
unsupported, invalid, infrastructure failure). `budget_exhausted` and timeout
are right-censored observations at their actual charged work, never failures
or zero-cost successes. SAT timeout is unknown, not UNSAT. An attempted cell
whose trace is truncated retains a trace-coverage flag and its last durable
event. Instruction-to-solve stays null until a same-process, migration-safe
counter interval covers every required phase and replay; native operation
counts remain named backend counters. First-/last-fall exact labels remain
unknown without complete certificates; store observed degree events and
bounds separately.

Publish every shard's immutable manifest, event stream, algebraic objects,
partial relations, matrix/root/SAT traces within the finite trace budget,
recovery certificates, budget journal checkpoints and hash/readback receipts.
Use a lossless replay tier and a derived public training view. Restricted
scalar/provenance records remain behind the approved access boundary; public
features expose only information available at the decision prefix. S3 object
existence alone does not mark a cell verified: run `dataset verify`, an
independent replay/consumer check, and a byte-equal remote readback. Preserve
the full target and the exact unattempted/unsupported/censored map in each
coverage report.
