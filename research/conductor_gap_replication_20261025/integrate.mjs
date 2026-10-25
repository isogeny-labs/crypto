// Render native results into the existing dashboard; never computes a result.
import {readFileSync,writeFileSync} from 'node:fs';
import assert from 'node:assert/strict';
const root='research/conductor_gap_replication_20261025';
const a=JSON.parse(readFileSync(root+'/analysis/panel-A.json','utf8'));
const timelinePath='docs/ic/progress-timeline.json';
// Retain arbitrary-size legacy number tokens; this script only renders metadata.
const parseExact=text=>JSON.parse(text,(_key,value,context)=>
  typeof value==='number'?JSON.rawJSON(context.source):value);
const timeline=parseExact(readFileSync(timelinePath,'utf8'));
if(process.argv[2]==='--repair-timeline'){
  const prior=parseExact(readFileSync(process.argv[3],'utf8'));
  const additions=timeline.non_numeric_results.filter(x=>x.anchor==='#conductor-gap-replication-20261025');
  assert.equal(additions.length,1);
  prior.non_numeric_results.push(...additions);
  prior.updated=timeline.updated;
  const exact=JSON.stringify(prior,null,1)+'\n';
  writeFileSync(timelinePath,exact);
  const scorePath='docs/index-calculus-scoreboard.html';
  const page=readFileSync(scorePath,'utf8');
  const pattern=/<script type="application\/json" id="progress-data">[\s\S]*?<\/script>/;
  assert(pattern.test(page));
  writeFileSync(scorePath,page.replace(pattern,'<script type="application/json" id="progress-data">\n'+exact+'</script>'));
  console.log('Restored all prior timeline content and exact numeric tokens; retained the new non-numeric result.');
  process.exit(0);
}
const anchor='#conductor-gap-replication-20261025';
assert(!timeline.non_numeric_results.some(x=>x.anchor===anchor),'integration is write-once');
timeline.non_numeric_results.push({anchor,date:'2026-10-25',
  label:'Conductor-gap replication across eight classes and CM-aware controls',
  reason_not_plotted:'The rho cohort has no IC/rho ratio. CM construction changes have incomplete setup counts/prices; no complete-cost IC/rho result is admitted.',
  source:'https://github.com/aburan28/crypto/blob/main/'+root+'/REPORT.md',
  status:'43,520 main and 128 excluded calibration attempts verified/replayed; independent arithmetic/graph checks pass; simultaneous crater equivalence remains inconclusive',value:null});
if(timeline.updated<'2026-10-25')timeline.updated='2026-10-25';
const encoded=JSON.stringify(timeline,null,1)+'\n';
writeFileSync(timelinePath,encoded);
const path='docs/index-calculus-scoreboard.html';
let page=readFileSync(path,'utf8');
const marker='<details class="dash-details"><summary>Conductor-gap native controls:';
assert(page.includes(marker));
const f=x=>x.toFixed(5);
const rows=a.levels.map(x=>`<tr><td>${x.level}</td><td>${f(x.ratio)}</td><td>[${x.ci98.map(f).join(', ')}]</td><td>${x.equivalent_5pct_nominal?'passes':'inconclusive'}</td><td>${x.equivalent_5pct_simultaneous?'passes':'inconclusive'}</td></tr>`).join('\n');
const section=`<details class="dash-details"><summary>Conductor-gap replication: eight classes and CM controls · 25 October 2026</summary><div class="dash-detail-body">
<section class="dash-section" id="conductor-gap-replication-20261025" aria-labelledby="conductor-gap-replication-title">
<div class="dash-section-head"><div><p class="dash-kicker">Conductor gaps / Bounded replication</p><h2 id="conductor-gap-replication-title">Balanced rho effort supports parity; corrected crater precision remains inconclusive.</h2></div><span class="status">L0 · incomplete CM setup accounting</span></div>
<p class="panel-summary">All 43,520 main attempts and 128 excluded calibration attempts verified and exactly replayed. Eight classes cover 32 depth-four volcanoes; independent C++ checks verify 1,472 vertices and 1,440 rational 2-isogeny edges. The equal-level secondary S2/S1 counted-rho ratio is ${f(a.equal_level_secondary.ratio)}, 95% [${a.equal_level_secondary.ci95.map(f).join(', ')}].</p>
<div class="table-scroll"><table><caption>Eight fixed classes, three S2 volcanoes per class. 192 separate solves per level/volcano; transported scalar indices are shared. Whole-class bootstrap, not an IID-row or population claim.</caption><thead><tr><th>Depth</th><th>S2/S1</th><th>98% interval</th><th>Nominal ±5%</th><th>Simultaneous rule</th></tr></thead><tbody>${rows}</tbody></table></div>
<p>All 512 scalar/recurrence and 128 scalar/direct-map construction comparisons retain the same seeds, queries, relations, ranks and recovered targets. Differences are in factor-base construction. Small CM self-maps act within craters; hard-gap target transport, prime-field algebraic-oracle coverage and complete native setup accounting remain open. Reported 0/0.1/1 GAE bands do not supply missing operation counts.</p>
<p>No complete-cost IC/rho or leaderboard gain is admitted. These 32/34-bit-field, depth-four observations do not establish a general hardness or scaling theorem. <a href="https://github.com/aburan28/crypto/blob/main/${root}/REPORT.md">Results, graphs and raw-data provenance</a> · <a href="https://github.com/aburan28/crypto/blob/main/${root}/REPORT.pdf">PDF</a></p>
</section></div></details>
`;
page=page.replace(marker,section+marker);
const pattern=/<script type="application\/json" id="progress-data">[\s\S]*?<\/script>/;
assert(pattern.test(page));
page=page.replace(pattern,'<script type="application/json" id="progress-data">\n'+encoded+'</script>');
const old='<p id="conductor-gap-unplotted">';
assert(page.includes(old));
page=page.replace(old,'<p id="conductor-gap-replication-unplotted">25 October: the <a href="#conductor-gap-replication-20261025">eight-class conductor-gap replication</a> has no admitted complete-cost IC/rho point. Its bounded result is retained in the non-numeric ledger.</p>\n'+old);
writeFileSync(path,page);
