// Document rendering from recorded native audit results; no scientific calculations.
import fs from 'node:fs';
const [auditFile,panelFile,output]=process.argv.slice(2);
if(!auditFile||!panelFile||!output||fs.existsSync(output))throw Error('audit, panel and fresh report directory required');
const audit=JSON.parse(fs.readFileSync(auditFile,'utf8')),panel=JSON.parse(fs.readFileSync(panelFile,'utf8'));
if(audit.status!=='raw-receipts-and-aggregates-verified')throw Error('missing native audit');
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
const proofRows=audit.projected_row_information.filter(r=>['icv1-fp6-tm7-fc20da94-b8-m2-s0','icv1-fp8-tm21-71e3bcb7-b8-m3-s0','icv1-fp8-tm5-23216e9c-b16-m4-s0'].includes(r.workload)).map(r=>[r.workload,r.canonical_tuple_witnesses,r.distinct_projected_coefficient_rows,r.rank_mod_source_order]);
const text=`# E26: equivalent encodings and matched ordinary controls

2026-10-25; bounded public toy fixtures; isolation L0.

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
extraction, complete counters and full costs are missing. E27 complete cold
accounting/calibration and E28 final selection are unrun. Labels 32--63 remain
unopened. No novelty, fastest-tool, NIST DLP, independent-host, calibrated
IC speedup or leaderboard promotion follows. All cards remain blocked and
unfunded; no paid host or lease was created.

[Primary assessment](../../execution-v2/runs/block-panel-20261025-v2/ASSESSMENT.json),
[independent audit](../../execution-v2/block-audit-20261025-v1/AUDIT.json),
[protocol](../../execution-v2/BLOCK_PROTOCOL_V1.md),
[matched-control amendment](../../execution-v2/BLOCK_AMENDMENT_V2.md),
[audit protocol](../../execution-v2/BLOCK_AUDIT_PROTOCOL_V1.md),
[raw artifact manifest](../../execution-v2/block-20261025/RAW_ARTIFACTS.json),
[validation logs](../../execution-v2/block-checks-20261025/).

Exact source, native/tool binary hashes, fixture and input hashes, commands,
UTC intervals, caps and raw outputs are retained with the records. GitHub
access returned 403/404 during this run; delivery is tracked separately in
[the access problem](../../../../problems/twist-github-repository-access.md).
The twelve historical oversized Git blobs remain a separate merge hold.
The IC leaderboard, progress timeline and scientific geometry figures remain
unchanged because this wave establishes no complete-cost result. Benchmark
copies are indexed as unadmitted diagnostics without adding a validated row.
`;
fs.writeFileSync(output+'/RESULTS.md',text);
fs.writeFileSync(output+'/.gitattributes','*.stdout -whitespace\n*.stderr -whitespace\n*.log -whitespace\n');
fs.writeFileSync(output+'/.gitignore','/RESULTS.tex\n/RESULTS.aux\n/RESULTS.log\n/RESULTS.out\n');
