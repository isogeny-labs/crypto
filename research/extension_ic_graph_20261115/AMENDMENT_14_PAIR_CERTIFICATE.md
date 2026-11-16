# Amendment 14: certify F4 replay through a pair-dependency graph

Frozen before stage19 timing, 16 November2026. Rust, authorized local backend.
Stage18's exhaustive certified trace reuse is complete: all33controlled cases
lose. No full-IC follow-up of that version. [pcc:unverified]

Sources checked (KB): no MCP loaded; wrapper status and two exact mechanism /
negative-cost queries fail with no executable. Receipts
/tmp/ic-trace-ordered-kb-{status,search1,search2}.log. Fallback: native
f4_fp/reuse.rs exhaustive checker, f4_fp.rs product/Gebauer–Möller/reduction
contracts, residual-walk and IC-technique notes; Perry, *An Extension of
Buchberger's Criteria for Groebner Basis Decision*, section2Theorems2/3,
[primary PDF](https://arxiv.org/pdf/0906.4358), SHA256
ab9aa3eeb81128f3f016cff9860f1cae711208de804b3e56986832210a3261ec.
Field coefficients and an admissible monomial order are required. The product
criterion discharges coprime leading terms; the chain criterion uses a path of
certified pairs whose leading monomials divide this pair's LCM. We use the
classical criteria only, not the paper's extended criterion or example gains.
Monagan–Pearce's primary heap-division paper section2 was also read; that
separate kernel change is deferred. No novelty assertion. [pcc:unverified]

## Exact dependency certificate

Keep the default exhaustive checker unchanged. Add explicit TracePool constructor
new_pair_graph and S4TraceState constructor new_pair_graph, full-IC mode
F4TracePairGraph / CLI f4-trace-pair-graph. Native replay row provenance and every
original-input reduction remain necessary. Normalize/check the complete monic
basis. Enumerate all basis pairs, sorted by ascending LCM/admissible order.
Each edge must have one certificate: direct zero reduction, coprime leading
monomials, or a middle vertex k with LM(k) dividing LCM(i,j) and BOTH edges
(i,k),(k,j) already certified. Add an edge only after its obligation succeeds.

Inductively every edge has an LCM-bounded standard representation: direct
reduction gives one, product pairs have the classical representation, and the
chain identity combines two earlier representations with monomial multipliers.
Prior-edge references prevent circular justification even for equal LCMs.
After all obligations succeed, Buchberger's criterion proves that this is a GB.
Combined with retained input reductions and native row provenance this gives
the same ideal. No root-completeness check is replaced by an unchecked proposal.
Finite quotient/borders/recurrence/original equations/cubic/group checks remain.
[pcc:unverified]

Cap at100000pair obligations,1000000witness tests per certificate, existing
1000000term-step/100000pending-term bounds per direct reduction and existing
pool4/1000000retained words. Resource refusal triggers charged ordinary F4,
never acceptance. Counters separately expose product/chain/direct edges,
witness tests/refusals, all pair obligations and actual field products. Preserve
old term-reduction kernel, basis/root RNG and old defaults. [pcc:unverified]

## Frozen stage19: fresh F4 versus cold pair-graph trace

Plan pair-certificate-plan.json; p67/127/1039, fresh seeds191/193/197,
32supplied scalar residuals1..32in four groups of8distinct abscissae per instance.
Reference fresh F4Delayed; candidate new family-bound pair-graph trace pool
per eight-system batch. Four arms fresh_a/trace_a/fresh_b/trace_b, reverse odd
rounds, one retained warmup and five measured rounds. Planned864batches:
144warmup+720measured,6912coordinate calls,36fixtures. [pcc:unverified]

Charge cold pool/graph construction/destruction, learning, all input/direct
pair reductions, product/chain tests, witness scans, refused/failed replays,
fallback and complete coordinate extraction. Shared curve/symbolic/residual
preparation and dense qualification remain outside phase and separately reported;
outer process wall retains them. No trained/repeated-identical-input pool.
Preserve exact starts/completions/censor flow, complete qualifier equality,
source/family/ordered-input/certifier identity and A/A semantic/answer checks.
None is unsupported, never an empty root set. Require integer edge counters
whose product+chain+direct sum equals all pair obligations. [pcc:unverified]

Same CPU0/RAYON1/OMP1,8GiBvirtual,900seconds/process,no per-call deadline,
native degree24,zero retries. Freeze source/binary/checkpoint before sole call;
no compiler while timing. Both median A/A ratios must be within[1/1.10,1.10].
Geometric candidate/reference batch wall,10000whole-paired-round bootstrap draws.
Retain all outcomes. Full-IC viability requires exit0, full correct cohort,
no global integrity failure and at least6of12largest-field cases with controlled
upper interval<1, accepted replay AND positive product/chain discharge in every
measured batch. Otherwise no full-IC trial of this version. Never compose ratios
with earlier runs. Public synthetic Fp^3 fixtures isolate a family absent from
the standard panel; no general curve claim or admitted index row. [pcc:unverified]

Manifest validation/pair-certificate-source.sha256. Create-new local directory
/workspace/ic-local-runs/local-pair-certificate-20261116-19. No stage19 data exists
at this preregistration checkpoint. No canonical encrypted-schema validation,
independent signed verdict, strongest-external or standard lab admission.
[pcc:unverified]
