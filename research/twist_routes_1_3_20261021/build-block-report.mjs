// Document rendering from recorded native audit results; no scientific calculations.
import fs from 'node:fs';
const [auditFile,panelFile,output]=process.argv.slice(2);
if(!auditFile||!panelFile||!output||fs.existsSync(output))throw Error('audit, panel and fresh report directory required');
const audit=JSON.parse(fs.readFileSync(auditFile,'utf8')),panel=JSON.parse(fs.readFileSync(panelFile,'utf8'));
if(audit.status!=='raw-receipts-and-aggregates-verified')throw Error('missing native audit');
const execution='research/twist_routes_1_3_20261021/execution-v2';
const separate=JSON.parse(fs.readFileSync(execution+'/runs/block-worker-cost-20261025-v1/COST.json','utf8'));
const batched=JSON.parse(fs.readFileSync(execution+'/runs/block-batch-cost-20261025-v1/COST.json','utf8'));
const batchAudit=JSON.parse(fs.readFileSync(execution+'/block-audit-20261025-v2/AUDIT.json','utf8'));
const publication=JSON.parse(fs.readFileSync(execution+'/block-20261025/RAW_ARTIFACTS.json','utf8'));
if(publication.status!=='remote-object-fresh-download-and-all-members-verified')throw Error('raw publication not verified');
for(const check of [audit,batchAudit])if(check.worker_instruction_probe_audit?.status!=='raw-Ir-counters-and-aggregates-verified')throw Error('raw instruction audit missing');
for(const cost of [separate,batched])if(cost.AA_within_preregistered_5_percent!==true||cost.complete_cost_or_speedup_established!==false)throw Error('instruction scope or A/A gate failed');
const rankGate=JSON.parse(fs.readFileSync('research/twist_routes_1_3_20261021/execution-v2/runs/block-rank-adjudication-20261025-v1/ADJUDICATION.json','utf8'));
if(rankGate.accepted!==true)throw Error('independent projected-rank replay is not accepted');
const controls=fs.readFileSync('research/twist_routes_1_3_20261021/execution-v2/block-checks-20261025/checker-controls.stdout','utf8');
if(!controls.includes('POSITIVE=1')||!controls.includes('TRUNCATED=0')||!controls.includes('VDIM=1')||!controls.includes('E26_CHECKER_CONTROLS_DONE'))throw Error('complete-basis controls missing');
fs.mkdirSync(output,{recursive:true});
const labels={'direct-chain':'Direct addition chain','ordinary-prefix-order':'Ordinary prefix order','frobenius-block':'Frobenius variable blocks','factored-control':'Factored membership','bits-control':'Boolean membership','hybrid-eight':'Eight-bucket helper','first-pair-invariant':'First-pair invariants','direct-aa':'A/A control'};
const escape=s=>String(s).replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('>','&gt;').replaceAll('"','&quot;');
const table=(headers,rows)=>'| '+headers.join(' | ')+' |\n| '+headers.map(()=>'---').join(' | ')+' |\n'+rows.map(r=>'| '+r.join(' | ')+' |').join('\n');
const arms=audit.by_arm.filter(a=>a.arm!=='direct-aa');
const htmlRows=arms.map(a=>`<TR><TD ALIGN="LEFT">${escape(labels[a.arm])}</TD><TD><TABLE BORDER="0" CELLBORDER="0" CELLSPACING="0"><TR>${a.certified_cells?`<TD BGCOLOR="#32826b" WIDTH="${a.certified_cells*3}" HEIGHT="18" FIXEDSIZE="TRUE"> </TD>`:''}${a.incomplete_cells?`<TD BGCOLOR="#c6cdd4" WIDTH="${a.incomplete_cells*3}" HEIGHT="18" FIXEDSIZE="TRUE"> </TD>`:''}</TR></TABLE></TD><TD ALIGN="LEFT">${a.certified_cells} certified / ${a.incomplete_cells} incomplete</TD></TR>`).join('\n');
fs.writeFileSync(output+'/COUNTS.dot',`digraph G {graph[bgcolor="white",pad="0.2"];node[shape=plain,fontname="Helvetica"];a[label=<<TABLE BORDER="0" CELLBORDER="0" CELLSPACING="9"><TR><TD COLSPAN="3"><B>E26: independently certified affine solver cells</B></TD></TR><TR><TD COLSPAN="3">90 workloads per arm; green certified, gray incomplete</TD></TR>${htmlRows}<TR><TD COLSPAN="3">${audit.distinct_fixture_target_pairs} public fixture/target pairs; counts are not independent samples</TD></TR></TABLE>>];}\n`);
fs.writeFileSync(output+'/COUNTS.plot.json',JSON.stringify({source:auditFile,source_field:'by_arm',values:arms,unit:'candidate cells',population:'90 correlated workloads per arm; one target per curve',colors:{certified:'#32826b',incomplete:'#c6cdd4'}},null,2)+'\n');
const example='icv1-fp6-tm7-fc20da94-b8-m2-s0';
const exampleRows=panel.filter(r=>r.workload===example&&r.arm!=='direct-aa').map(r=>[labels[r.arm],r.native_row_reduction_multiplications,r.all_branches_independently_certified?'certified':'incomplete']);
const proofNames={'icv1-fp6-tm7-fc20da94-b8-m2-s0':'p=59, B=8, m=2','icv1-fp8-tm21-71e3bcb7-b8-m3-s0':'p=211, B=8, m=3','icv1-fp8-tm5-23216e9c-b16-m4-s0':'p=251, B=16, m=4'};
const proofRows=audit.projected_row_information.filter(r=>Object.hasOwn(proofNames,r.workload)).map(r=>[proofNames[r.workload],r.canonical_tuple_witnesses,r.distinct_projected_coefficient_rows,r.rank_mod_source_order]);
const instructionRows=arms.map(({arm})=>[labels[arm],separate.by_arm.find(r=>r.arm===arm&&r.engine==='native').guest_instructions_Ir,batched.by_arm.find(r=>r.arm===arm).guest_instructions_Ir,separate.by_arm.find(r=>r.arm===arm&&r.engine==='msolve').guest_instructions_Ir]);
fs.writeFileSync(output+'/INSTRUCTIONS.plot.json',JSON.stringify({sources:[execution+'/runs/block-worker-cost-20261025-v1/COST.json',execution+'/runs/block-batch-cost-20261025-v1/COST.json'],unit:'Callgrind simulated guest instructions Ir',fixed_workload:example,rows:instructionRows,columns:['encoding','native separate branch workers','native one batch worker','msolve separate branch workers'],scope:'solver worker startup, parse, symbolic/F4 work and serialization; excludes construction and complete IC',uncertainty:'one fixed public toy case; same-input A/A controls only'},null,2)+'\n');
const instructionChartRows=instructionRows.map(r=>`<TR><TD ALIGN="LEFT">${escape(r[0])}</TD>${r.slice(1).map(v=>`<TD ALIGN="RIGHT">${v.toLocaleString('en-US')}</TD>`).join('')}</TR>`).join('\n');
fs.writeFileSync(output+'/INSTRUCTIONS.dot',`digraph G {graph[bgcolor="white",pad="0.2"];node[shape=plain,fontname="Helvetica"];a[label=<<TABLE BORDER="0" CELLBORDER="0" CELLSPACING="10"><TR><TD COLSPAN="4"><B>Fixed p=59 case: simulated guest instructions Ir</B></TD></TR><TR><TD>Encoding</TD><TD>Native separate</TD><TD>Native batch</TD><TD>msolve separate</TD></TR>${instructionChartRows}<TR><TD COLSPAN="4">Different process models; no admitted time or complete IC cost</TD></TR></TABLE>>];}\n`);
const text=`# E26: equivalent encodings and matched ordinary controls

2026-10-25; bounded public toy fixtures; Intel Xeon Platinum 8573C;
managed Linux x86_64; isolation L0. No admitted wall time.

The native construction covers all 90 preregistered workloads: ten ordinary
curves, B=8,12,16 abscissae, arities 2,3,4 and original target label 0.
All four candidates and the ordinary controls are retained. The requested
solver population is 631 cells including one A/A control. This is a partial
E26 pilot, not complete IC costing or a speedup claim.

## Verified construction and information checks

Independent PARI replay accepts ${audit.independent_PARI_signed_witness_checks}
retained signed-witness checks and rejects
${audit.independent_PARI_wrong_sign_checks} wrong-sign controls. The Rust gate
checks the script and output hashes, completion marker and clean errors.
Counts repeat point tuples across overlapping base sizes. There are
${audit.distinct_fixture_target_pairs} distinct fixture/target pairs.

All 90 matched-order checks recover the ordinary polynomial coefficients
exactly after setting anti-rational variables to zero. This is an input-level
identity check for the frozen family, not a universal statement about every
twist encoding. The invariant diagonal and identity-prefix exceptions remain
in the construction and full relation census.

The same-input A/A polynomial check passes; equal independently certified
row-multiplication counts: **${audit.AA_equal_certified_row_multiplications}**.
Native step-log counter coverage failures: **${audit.trace_coverage_failures}**.
Any failed or missing certificate remains incomplete.

## Complete and incomplete solver cells

${table(['Arm','Requested','Certified','Incomplete','msolve certified branches'],audit.by_arm.map(a=>[labels[a.arm],a.requested_cells,a.certified_cells,a.incomplete_cells,a.msolve_independently_certified_branches]))}

![Complete affine-basis certificates within the fixed budgets. Gray includes timeouts, truncation, failed checks and cells not started after a global cap.](COUNTS.svg)

Native F4 uses grevlex, degree bound 32, GM=1 and a shared 30-second budget
per candidate cell, including every helper/diagonal branch. Stock msolve q0
and Singular share a separate 30-second cell budget. The native and external
phases each have a 1,800-second global cap, with checkpoints. Global-cap
omissions remain not attempted. Every process has one worker and a 4 GiB
address-space cap. Raw failure outputs and exact inputs are retained.

Singular computes a reference basis and checks both ideal containments,
equality of leading ideals and vdim against the complete native affine
assignment census. These are complete-basis checks. vdim is the quotient
vector-space dimension, not Krull dimension. Parsed stock msolve bases have
their own independent certificate; an msolve failure does not erase a valid
native certificate. The small positive/truncated-basis control is retained.

## Fixed first workload: what the recorded counts measure

The p=59, B=8, arity-2 example was fixed before inspecting outcomes.
Counts below are multiplications in native row reduction only. A smaller
number is not a measured full-solver or IC speedup.
Incomplete counter sums are partial records, not zero-cost successes.

${table(['Encoding','Recorded row multiplications','Basis status'],exampleRows)}

Construction, symbolic preprocessing, normalization, root extraction,
process startup, signs, deduplication, exception enumeration, transfer,
verification and complete target recovery are not priced by this counter.
All eight helper solves are included in its row-multiplication sum, while
their remaining overhead is missing. No admitted wall time is reported.
The ordinary comparison is the preregistered hindsight minimum among four
fixed controls; it is not an executable selector or an IC pipeline baseline.

## Whole-worker instruction diagnostics

Callgrind records simulated guest instructions Ir, not cycles, wall time,
field operations or GAE. The first probe starts a separate process for each
helper/diagonal branch. All 31 native and stock-msolve worker outputs match
the previously certified bases (and native row counters). The second probe
runs every native candidate through the same batch interface, one process
per candidate; all eight process outputs and every branch match. Independent
audits reopen the raw Ir records and recompute their aggregates.

${table(['Encoding','Native separate Ir','Native batch Ir','msolve separate Ir'],instructionRows)}

![Exact recorded instruction totals for the fixed first workload. One toy case, with same-input A/A repeats; the two native process models are separate experiments.](INSTRUCTIONS.svg)

Separate-worker A/A relative spread is ${separate.AA_relative_spread};
batch-worker A/A relative spread is ${batched.AA_relative_spread}.
Both meet the frozen 5-percent check. These are same-input repeats, not
confidence intervals or an independent-host replication.

The helper's 458 row multiplications omit enough other work to change its
ranking: separate helper workers use 5,784,840 Ir versus the direct worker's
2,423,541. Batching the helper branches records 2,458,250 Ir versus the
matched direct batch's 2,431,742. The pair-invariant batch records 1,847,549
Ir on this fixed case; its stock-msolve separate-worker total is 20,283,359
Ir versus direct's 11,030,733. Process model and solver therefore matter.
These are observations of this prototype, not an inherent ranking or a
twist-specific advantage. No fastest-implementation claim is made.

The worker interval includes startup, input parsing, symbolic preprocessing,
F4 and output serialization. It excludes encoder construction, parent
orchestration, OS/kernel work, certification, root/sign lifting, duplicate
removal, linear algebra over relations and target recovery. The batch
follow-up was motivated by the first probe's training result; it is not heldout.
There is no batched msolve measurement. Complete-cost and calibrated IC
comparisons remain open.

## Tuple multiplicity, coefficient rows and rank

${table(['Frozen workload','Tuple witnesses','Distinct rows','Rank mod source order'],proofRows)}

The native auditor deduplicates signed coefficient vectors and computes rank
over the prime source subgroup order, rather than the curve coordinate field.
A separate PARI input replays all 90 ranks. Full-task ranks include exceptional
relations; they are not used as denominators for affine-solver cost. Exactly
one retained deterministic sign witness per canonical tuple is used, not every
possible sign assignment. The first-pair permutation is folded consistently.

## Scope, sources and remaining work

This wave implements and runs bounded E26 encodings. The genus-two transfer
result remains the prior-art BHLS construction with 24 verified toy targets;
no new transfer-speed result is claimed. Earlier extension/rational helper
row equalities remain valid. The 270-cell E30 pilot is a separate experiment,
and its full solver-cost comparison is still open.

E26 remains partial: broader training/heldout cases, production root
extraction, matrix nonzeros, complete counters and full costs are missing. E27 complete cold
accounting/calibration and E28 final selection are unrun; these worker probes
do not replace either requirement. Labels 32--63 remain
unopened. No novelty, fastest-tool, NIST DLP, independent-host, calibrated
IC speedup or leaderboard promotion follows. All cards remain blocked and
unfunded; no paid host or lease was created.

[Primary assessment](../../execution-v2/runs/block-panel-20261025-v2/ASSESSMENT.json),
[independent audit](../../execution-v2/block-audit-20261025-v1/AUDIT.json),
[batch instruction audit](../../execution-v2/block-audit-20261025-v2/AUDIT.json),
[separate-worker probe](../../execution-v2/runs/block-worker-cost-20261025-v1/COST.json),
[batch-worker probe](../../execution-v2/runs/block-batch-cost-20261025-v1/COST.json),
[protocol](../../execution-v2/BLOCK_PROTOCOL_V1.md),
[matched-control amendment](../../execution-v2/BLOCK_AMENDMENT_V2.md),
[audit protocol](../../execution-v2/BLOCK_AUDIT_PROTOCOL_V1.md),
[worker protocol](../../execution-v2/BLOCK_WORKER_COST_PROTOCOL_V1.md),
[batch protocol](../../execution-v2/BLOCK_BATCH_COST_PROTOCOL_V1.md),
[raw artifact manifest](../../execution-v2/block-20261025/RAW_ARTIFACTS.json),
[validation logs](../../execution-v2/block-checks-20261025/).

The new raw bundle contains ${publication.archived_files} archived files and
${publication.loose_files} separately tracked metadata records. Its
${publication.archive.bytes}-byte immutable object is published in the
user-confirmed Setec bucket. Fresh download, remote verification and every
member hash pass; extraction/repacking reproduces the exact archive digest.
[Publication receipt](../../execution-v2/block-publication-20261025-setec-v1/RECEIPT.json)
and [downloaded-member check](../../execution-v2/block-publication-20261025-setec-v1/archive-members/ARCHIVE_CHECK.json).
The native index entry accompanies these records. The separate
[delivery receipt](../../execution-v2/block-checks-20261025/DELIVERY.json)
tracks the pushed head and CI without changing the scientific measurements.
The earlier archive and index entry are unchanged.

Exact source, native/tool binary hashes, fixture and input hashes, commands,
caps and raw outputs are retained with the records. UTC intervals are retained
where captured. The batch probe's launch timestamps were not captured; its
post-run receipt records that gap and the saved actual child exit explicitly.
GitHub access initially returned 403/404. A later CLI fetch and PR read succeed
and resolve the existing PR to isogeny-labs/crypto; the connector's old path
still returns 404. Delivery and its exact head are tracked separately in
[the access problem](../../../../problems/twist-github-repository-access.md).
The twelve historical oversized Git blobs remain a separate merge hold.
The IC leaderboard, progress timeline and scientific geometry figures remain
unchanged because this wave establishes no complete-cost result. Benchmark
copies are indexed as unadmitted diagnostics without adding a validated row.

## Requirement status

${table(['Requirement','Status','Remaining acceptance gap'],[
['Genus-two route','Partial; earlier 24 toy transfers unchanged','Full transfer and destination-solving costs; polarized graph identification'],
['Extension helpers','Tested families retain matched rows','A new restriction or cheaper solver computing the same projected task'],
['E26 encodings','Bounded pilot verified; full experiment partial','Broader training/heldout, root extraction, nonzeros and complete costs'],
['E30 finite incidence','270 correctness cells verified earlier','Separate full solver-cost comparison remains unrun'],
['E27 complete cold cost','Unrun; worker probes are separate diagnostics','Original 24-arm instrumentation and all 1,920 attempts, calibrated matched rho'],
['E28 heldout selector','Unrun; labels 32–63 unopened','Freeze selector only after complete training and E27 counters'],
['Delivery','Tracked by separate commit/push receipt','Current-head CI/review; historical size hold remains']])}

[Next discriminating experiments](../../execution-v2/block-20261025/NEXT_EXPERIMENTS.md)
retain the original blocked cards and distinguish proposed runs from this
completed bounded wave. No subsequent job is implied to be running.
`;
fs.writeFileSync(output+'/RESULTS.md',text);
fs.writeFileSync(output+'/.gitattributes','*.stdout -whitespace\n*.stderr -whitespace\n*.log -whitespace\n');
fs.writeFileSync(output+'/.gitignore','/RESULTS.tex\n/RESULTS.aux\n/RESULTS.log\n/RESULTS.out\n');
