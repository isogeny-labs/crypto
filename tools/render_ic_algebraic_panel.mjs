#!/usr/bin/env node
// Presentation only. All ratios/counts come from native raw-data recomputation.
import {readFileSync, writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {dirname, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import assert from 'node:assert/strict';

const root=resolve(dirname(fileURLToPath(import.meta.url)),'..');
const study='research/ic_algebraic_crossover_20261023/';
const raw=readFileSync(resolve(root,study+'report-data.json'));
const data=JSON.parse(raw);
assert.equal(data.schema,'ic-campaign-report-data/v1');
assert.equal(data.status,'recomputed_from_raw');
assert.equal(data.complete_dlp_cost,null);
assert.equal(data.rho_ratio,null);
const hash=createHash('sha256').update(raw).digest('hex');
const zero=JSON.parse(readFileSync(resolve(root,study+'pipeline-receipts/guess-zero-draw6-20261025/row.json')));
const quartic=JSON.parse(readFileSync(resolve(root,study+'pipeline-receipts/quartic-smoke-20261025-02.json')));
const subgroup=JSON.parse(readFileSync(resolve(root,study+'pipeline-receipts/fixture-subgroups-enumerated-20261025/receipt.json')));
const qualified=JSON.parse(readFileSync(resolve(root,study+'pipeline-receipts/qualified-queries-20261025/summary.json')));
const closures=JSON.parse(readFileSync(resolve(root,study+'pipeline-receipts/qualified-closure-replay-20261025/summary.json')));
const wide=[16,17,18].map(m=>JSON.parse(readFileSync(resolve(root,study+`pipeline-receipts/wide-core-audit-20261025/m${m}/output.json`))));
assert.equal(zero.status,'refuted');
assert.equal(zero.draw,6);
assert.deepEqual(zero.guessed_variables,[]);
assert.equal(quartic.verified_records,24);
assert.equal(quartic.replays.length,24);
assert.ok(quartic.replays.every(r=>r.reproduced));
assert.equal(subgroup.schema,'ic-fixture-subgroups/v2');
assert.equal(subgroup.status,'all_lifts_trace_orders_and_membership_checks_agree');
assert.equal(qualified.rows,2048);
assert.equal(qualified.independent_point_arithmetic_rows_verified,2048);
assert.equal(closures.recorded_attempts,158);
assert.ok(closures.groups.every(g=>g.independent_hky_verified===g.attempts && g.refuted===0 && g.unknown===0));
assert.ok(wide.every(r=>r.status==='verified' && typeof r.r_decimal==='string' && typeof r.group_order_decimal==='string'));
const fmt=n=>Number(n).toLocaleString('en-US',{maximumFractionDigits:8});
const base='https://github.com/aburan28/crypto/blob/main/'+study;
const begin='<!-- BEGIN IC ALGEBRAIC FOLLOWUP -->';
const end='<!-- END IC ALGEBRAIC FOLLOWUP -->';
const block=`${begin}
<section class="dash-section" id="ic-algebraic-followup-20261025" style="max-width:1180px;min-width:0;margin:24px auto;padding:18px;box-sizing:border-box;overflow-wrap:anywhere" aria-labelledby="ic-algebraic-followup-title">
<div class="dash-section-head"><div><p class="dash-kicker">Structured bases and closure / finite native counts / 2026-10-25</p><h2 id="ic-algebraic-followup-title">The symmetric formulation lowers solver work at degree two; extra closure reverses that result.</h2></div><span class="status neutral">Engineering diagnostic · L0</span></div>
<p class="panel-summary">Finds lower symmetric solver work in 263/304 degree-two pairs and zero degree-three/four pairs; no complete-price IC improvement is established.</p>
<p>Each panel has 1,152 raw rows, 336 unique small public queries and 304 completed direct/symmetric pairs. The 32 incomplete or unpaired inputs remain unknown. Duplicate prefix replicates are retained and excluded from the primary comparison. These are closure-cap ablations: the direct hybrid solver raises its extraction degree to the input degree, up to six.</p>
<div class="table-scroll"><table><caption>Recorded solver word operations only; construction, splitting and physical conversion are not priced.</caption><thead><tr><th>Closure cap</th><th>Complete extractions, all forms</th><th>Symmetric lower words</th><th>Symmetric/direct word ratio, observed range</th></tr></thead><tbody>
${data.panels.map(p=>`<tr><td>${p.degree}</td><td>${fmt(p.complete_extractions)}</td><td>${p.symmetric_lower_words}/${p.paired_queries}</td><td>${fmt(p.ratio_min)}–${fmt(p.ratio_max)}</td></tr>`).join('\n')}
</tbody></table></div>
<figure><img src="ic/algebraic-crossover-closure-ratios.svg" alt="Paired symmetric to direct solver-word ratios at closure caps two, three and four, over fields of degree thirteen to thirty-one" style="display:block;width:100%;max-width:900px;margin:auto;background:white"><figcaption>304 completed matched pairs per cap; native deterministic counts, no population interval or wall-time comparison. All completed extractions agree with an independent point oracle.</figcaption></figure>
<p>At n=31, ell=4, the recorded geometric product dimensions are 4/7/10, versus 4/10/20 for random bases. That representation advantage does not establish a full-query gain. In the completed symmetric queries, 8,100 algebraic roots yield four verified x-multisets; 8,096 fail splitting or signed lifting.</p>
<p>The balanced ell=8 three-set [7,15,23] fails on all eight assignments of original draw 0: each saturates degree four without refuting, charging ${fmt(data.balanced_guess_draw0.total_word_ops)} solver word operations in total. This rejects that set on that draw, not a global minimum. Smaller capped attempts and other draws remain separate obligations.</p>
<p>The x-only guessing fixtures do not specify a full curve or subgroup. A native audit checks explicit E0 and trace-one-a twist models in each exact field: ${fmt(subgroup.totals.in_generated_subgroup)} of ${fmt(subgroup.totals.lifted_target_models)} original/fresh target x-values lie in the declared cyclic subgroups, ${fmt(subgroup.totals.outside_generated_subgroup)} lie outside and ${fmt(subgroup.totals.generated_subgroup_unknown)} remain unknown. The initial 152 nulls are retained in a separate receipt; exact enumeration of an 11-point subgroup resolves them in the amended audit. This remains a uniform field-x fixture law, not an ordinary subgroup-query law.</p>
<p>All seven original E0 factor bases have no points in its largest prime subgroup (r=991); draw 6 has none on either chosen model. The three selected SAT witnesses give signed curve relations, but none with every summand in the declared subgroup. This materially qualifies the ECDLP interpretation while preserving the finite Boolean results. <a href="${base}reports/run-fixture-subgroups-enumerated-20261025/report.pdf">Inspected subgroup report</a>.</p>
<p>The larger-cap continuation observes a zero-guess degree-four refutation on original draw 6: ${fmt(zero.columns)} columns and ${fmt(zero.closure_word_ops)} solver word operations. Independent solver/cost replay is outstanding, and the remaining balanced-branch campaign is active. Larger N=58/60/100 probes saturate without refuting at degrees two and three; all degree-four outcomes remain unknown after allocation or entry caps.</p>
<p>A separate qualified panel retains ${fmt(qualified.rows)} public hash-to-subgroup queries on four explicitly selected a=0, full-degree-b models. All finite-chain and independent point-arithmetic replays agree: ${fmt(qualified.boolean_sat)} have Boolean roots and ${fmt(qualified.point_sat)} have actual signed subgroup decompositions. These different contracts are preserved, including empty bases and quota shortages. The selected 55 negative and 24 positive controls give ${fmt(closures.recorded_attempts)} nonrefuting D2/D3 fixed points, all independently rank/verdict checked with HKY. Degree four, complete guessing minima and full prices remain open. <a href="${base}reports/run-qualified-queries-20261025/report.pdf">Qualified queries</a> · <a href="${base}reports/run-qualified-closures-20261025/report.pdf">Closure report</a>.</p>
<p>The separate quartic smoke now has 24/24 verified records and 24/24 identical deterministic replays over base-field sizes 6/8/10/12. Its prices use deterministic count conventions; construction, physical conversion and the wider crossover ladder remain separate obligations.</p>
<p>The standalone widened quartic core now passes independent arithmetic, exact group-order, high-scalar and encoding controls at extension degrees ${wide.map(r=>r.n).join('/')}, with identical replays. Full wide factor-base/oracle/relation/linear-algebra and ecbench adapters are still missing. The separate supported m14/15 capped L0 diagnostics are genuinely queued after balanced guessing; original L2 requirements remain unchanged. Neither core correctness nor the constructor instruction attribution establishes a measured crossover. <a href="${base}reports/run-wide-core-audit-20261025/report.pdf">Wide-core report</a>.</p>
<p>No new complete-price IC/rho ratio is admitted. Completed n13/n23 solver charges use pinned word rates; the earlier wall-pricing explanation was incorrect and has an explicit native accounting audit. Construction/split costs remain incomplete in both raw S and the native bands. The progress ratio chart, leaderboard, best-of and exponent figures retain their existing values. Larger closure/guess panels and the quartic crossover ladder remain open.</p>
<p class="dash-footnote"><a href="${base}README.md">Requirement statuses and evidence</a> · <a href="${base}reports/local-structural-20261025/report.pdf">Inspected results PDF</a> · <a href="${base}report-data.json">Native recomputation</a>. Dataset SHA-256 <code>${hash}</code>. Raw rows and failed attempts are in the hash-checked evidence archive; no fastest-CAS, GPU or asymptotic claim.</p>
</section>
${end}`;
const pagePath=resolve(root,'docs/index-calculus-scoreboard.html');
const current=readFileSync(pagePath,'utf8');
let page;
if(current.includes(begin)) {
  assert.equal(current.split(begin).length,2);
  assert.equal(current.split(end).length,2);
  page=current.slice(0,current.indexOf(begin))+block+current.slice(current.indexOf(end)+end.length);
} else {
  const anchor='<!-- END IC DASHBOARD OVERVIEW -->';
  assert.equal(current.split(anchor).length,2);
  page=current.replace(anchor,anchor+'\n'+block);
}
const svg=readFileSync(resolve(root,study+'reports/local-structural-20261025/closure-ratios.svg'));
const copy=resolve(root,'docs/ic/algebraic-crossover-closure-ratios.svg');
if(process.argv.includes('--check')) {
  assert.equal(current,page,'Algebraic panel is stale');
  assert.deepEqual(readFileSync(copy),svg,'Algebraic graph copy is stale');
  console.log('PASS: native evidence panel and graph copy agree');
} else {
  writeFileSync(pagePath,page);
  writeFileSync(copy,svg);
  console.log('Rendered finite algebraic panel and graph copy');
}
