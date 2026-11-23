// Document/plot layout of recorded native aggregates. No research aggregation.
import fs from 'node:fs';
import crypto from 'node:crypto';
const s='research/twist_routes_1_3_20261021',v=s+'/execution-v4',root=s+'/reports/progression-20261027';
const read=p=>JSON.parse(fs.readFileSync(p)),sha=p=>crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const cost=read(v+'/cost-audit-v1/AUDIT.json'),geo=read(v+'/geometry-audit-v1/AUDIT.json'),pre=read(v+'/PREFLIGHT_RUNTIME.json'),strata=read(v+'/export-v3/STRATIFIED.json');
const admission=read(v+'/cost-audit-v1/ADMISSION.json'),publication=read(v+'/RAW_VERIFICATION.json');
if(fs.existsSync(root))throw Error('fresh report directory required');
if(publication.status!=='archive-and-all-members-and-repack-verified')throw Error('raw verification required');
fs.mkdirSync(root,{recursive:true});
const esc=s=>String(s).replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('>','&gt;');
const texEscape=s=>String(s).replace(/[\\{}_$%&#~^]/g,c=>({'\\':'\\textbackslash{}','{':'\\{','}':'\\}','$':'\\$','_':'\\_','%':'\\%','&':'\\&','#':'\\#','~':'\\textasciitilde{}','^':'\\textasciicircum{}'}[c]));
const tex=s=>texEscape(s).replace(/[a-f0-9]{40,64}/g,m=>m.match(/.{1,8}/g).join('\\allowbreak{}'));
const svg=(height,body)=>`<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="${height}" viewBox="0 0 1000 ${height}"><rect width="1000" height="${height}" fill="white"/><g font-family="DejaVu Sans,sans-serif" font-size="18" fill="#223544">${body}</g></svg>\n`;
const table=(heads,rows)=>'| '+heads.join(' | ')+' |\n| '+heads.map(()=>'---').join(' | ')+' |\n'+rows.map(r=>'| '+r.join(' | ')+' |').join('\n');
const fmt=x=>Number(x).toPrecision(6);
const short={'direct-aa':'Direct A/A','direct-chain':'Direct F4','hybrid-eight':'Eight helpers','companion-curve':'Companion','group-aa':'Group A/A','group-mitm':'Group MITM'};
const label=x=>short[x]||x;
function counts(file,rows,title,caption){
  let b=`<text x="24" y="32" font-size="25" font-weight="bold">${esc(title)}</text><text x="24" y="61" font-size="16">${esc(caption)}</text>`;
  rows.forEach((r,i)=>{const y=93+46*i;b+=`<text x="24" y="${y+21}">${esc(label(r.arm))}</text><rect x="255" y="${y}" width="${r.requested*3.5}" height="28" fill="#dce3e7"/><rect x="255" y="${y}" width="${r.accepted*3.5}" height="28" fill="#1a7c72"/><text x="835" y="${y+21}">${r.accepted}/${r.requested}</text>`;});
  b+=`<text x="24" y="${119+46*rows.length}" font-size="16">Teal: independently accepted; gray: remaining attempts. Counts are processes.</text>`;
  fs.writeFileSync(file,svg(145+46*rows.length,b));
}
function ratios(file,rows){
  const xpos=x=>340+(Math.log10(x)+1)*125;
  let b='<text x="24" y="32" font-size="25" font-weight="bold">Same-base worker instruction ratios</text><text x="24" y="61" font-size="16">Candidate / control Ir. Dot: median; line: observed min–max, not uncertainty.</text>';
  for(const x of [.1,1,10,100,1000])b+=`<path d="M${xpos(x)} 78 V390" stroke="${x===1?'#af5642':'#dce3e7'}" stroke-dasharray="5 4"/><text x="${xpos(x)}" y="420" text-anchor="middle">${x}</text>`;
  rows.forEach((r,i)=>{const y=106+i*58;const [a,c]=r.comparison.split('/');b+=`<text x="24" y="${y}">${esc(label(a))} / ${esc(label(c))}</text><text x="24" y="${y+19}" font-size="14">${r.ratio.pairs} admitted pairs; median ${fmt(r.ratio.median)}</text><path d="M${xpos(r.ratio.min)} ${y} H${xpos(r.ratio.max)}" stroke="#1a7c72" stroke-width="4"/><circle cx="${xpos(r.ratio.median)}" cy="${y}" r="6" fill="#254e86"/>`;});
  b+='<text x="24" y="461" font-size="16">Frozen A/A gates and exact equal-output checks; correlated toy configurations.</text><text x="24" y="486" font-size="16">No admitted wall time, GAE, full IC or fastest-external claim. cost-v1, 2026-10-27.</text>';
  fs.writeFileSync(file,svg(510,b));
}
const common=[
  'Scientific source d01d7bb63cc577a4299200ae3c560767b6d68650; binary hashes and build inputs are pinned in execution-v4/FROZEN.json. All ten prior public odd-prime-order fixtures are retained with their registered source/twist identities. No outcome-based seed or fixture selection; final E28 labels 32–63 remain unopened.',
  'Backend: authorized local fallback after CodeBuild monitor and ECR readiness reads were denied; no EKS Job or paid launch. Intel Xeon Platinum 8573C, five visible logical CPUs, KVM, one worker/thread; L0. Host metadata captured after the run. Native wall time, RSS, calibrated operations and cycles are not admitted.',
  'Raw files: execution-v4/runs.tar.xz, artifact SHA256 '+publication.sha256+'. Fresh Setec download verifies all 5,867 member hashes and identical deterministic repack. Extract the archive before native replay. The independent verifier shares this host/author; separate code and PARI are not independent-host review.',
  'Structured exports pass standalone schema/file validation. Full binding to the frozen generic factorial protocol is rejected: configuration count and separate controls were encoded incorrectly for its balanced-panel model. The native 880-cell plan was fixed before data and is unchanged. See EXPORT_LIMITS.md; no validation-gated row is promoted.',
  'Prior work: Semaev, Summation polynomials and the discrete logarithm problem on elliptic curves, IACR ePrint 2004/031; established quadratic-twist/Frobenius theory and prior execution-v3 reports. Primary IACR PDF requests returned HTTP 403, so the literature/novelty review has an access gap. The atlas leads on generic-table decomposition, structured-base coincidences and special-j x-line symmetry were reviewed. No novelty is established.'
];
function report(slug,title,run,paragraphs,graphs,sourceFiles,rows=''){
  const dir=root+'/'+slug;fs.mkdirSync(dir,{recursive:true});
  const intro=[...paragraphs,...common];
  fs.writeFileSync(dir+'/RESULTS.md',`# ${title}\n\nRun: **${run}**.\n\n`+intro.join('\n\n')+'\n\n'+rows+'\n\n'+graphs.map(g=>`![${g.caption}](${g.name}.svg)`).join('\n\n')+'\n\nSources: '+sourceFiles.map(p=>'['+p+'](../../../'+p+')').join(', ')+'.\n');
  fs.writeFileSync(dir+'/RESULTS.tex',String.raw`\documentclass[10pt]{article}
\usepackage[T1]{fontenc}\usepackage[utf8]{inputenc}\usepackage{lmodern}
\usepackage[margin=18mm]{geometry}\usepackage{graphicx,hyperref,parskip}
\hypersetup{hidelinks}\setlength{\emergencystretch}{3em}
\begin{document}
`+'\\section*{'+tex(title)+'}\n\\textbf{Run:} '+tex(run)+'\\par\n'+intro.map(p=>tex(p)+'\n\n').join('')+'\\textbf{Sources:} '+sourceFiles.map(p=>'\\path{'+p+'}').join('; ')+'\n'+graphs.map(g=>'\\clearpage\n\\section*{'+tex(g.caption)+'}\n\\includegraphics[width=\\linewidth]{'+g.name+'.pdf}\n').join('')+'\\end{document}\n');
  fs.writeFileSync(dir+'/SOURCES.json',JSON.stringify({run,files:sourceFiles.map(p=>({path:s+'/'+p,sha256:sha(s+'/'+p)})),scientific_aggregation_in_javascript:false},null,2)+'\n');
  return dir;
}
const gd=report('geometry','Shifted progression bases: exact finite census','geometry-v1',[
  'The native producer emits 240 constructions and 7,680 finite query records in one execution. Independent square-table, Euclidean-inverse and affine-group code reproduces every base and all query counts. Of the requested bases, 214 fill; the other 26 remain explicitly unsupported. B is 8 or 16, shifts are 0, 1 and 3, arities are 2 and 3, and public target labels are 0–15.',
  'The unshifted twist produces no nonzero source lifts, as required. Shifted twist progressions do produce valid original-curve bases. Ordinary source progressions also change coverage after a shift. This finite census establishes construction and relation counts, not a twist-specific solving advantage. Sorted-first-pair tuple counts are not independent relations or matrix rank.',
  'Command: frozen worker followup progression-census execution-v3/decomposition-20261026/PLAN.json execution-v4/runs/geometry-v1; frozen harness audit-geometry execution-v4/PLAN.json CENSUS AUDIT. Geometry cap: 600 seconds, 4 GiB, 64 MiB/file. Measured quantity: exact finite counts only. All zero and underfilled cases are retained.'
],[{name:'COVERAGE',caption:'B=8, arity 3: nonempty queries among 160 requested per row'}],['execution-v4/geometry.protocol.json','execution-v4/geometry-audit-v1/AUDIT.json']);
const grow=geo.by_constructor.filter(r=>r.B===8&&r.arity===3&&((r.family.includes('orbit')&&[0,1,3].includes(r.shift))||(!r.family.includes('orbit')&&r.shift===1)));
fs.writeFileSync(gd+'/PLOT_DATA.json',JSON.stringify(grow,null,2)+'\n');
let gb='<text x="24" y="32" font-size="25" font-weight="bold">Finite target coverage changes with ordinary shifts too</text><text x="24" y="61" font-size="16">B=8; arity 3; labels 0–15 on 10 curves; each row has 160 requested queries.</text>';
grow.forEach((r,i)=>{const y=90+i*42;gb+=`<text x="24" y="${y+20}">${esc(r.family)} c=${r.shift}</text><rect x="285" y="${y}" width="480" height="27" fill="#e5e8eb"/><rect x="285" y="${y}" width="${r.supported_queries*3}" height="27" fill="#a8c9d6"/><rect x="285" y="${y}" width="${r.nonempty_queries*3}" height="27" fill="#1a7c72"/><text x="785" y="${y+20}">${r.nonempty_queries} / ${r.supported_queries} supported</text>`;});
gb+=`<text x="24" y="${119+grow.length*42}" font-size="16">Teal: nonempty; blue: supported empty; gray: underfilled. geometry-v1.</text>`;
fs.writeFileSync(gd+'/COVERAGE.svg',svg(147+grow.length*42,gb));
const pd=report('preflight','Operational sizing: separate preflight','preflight-v1',[
  'The predeclared smallest fixture in each original field-bit class, ordinary curve progression, shift 1, B=8 and both arities supplies 60 worker attempts over six applicable arms. This is operational sizing only, excluded from primary cost analysis. No primary metric, threshold, fixture or sample size was selected from these outcomes.',
  `The native preflight estimator records mean process duration ${pre.per_sample_seconds} seconds and projected worker-duration sum ${pre.estimated_880_worker_seconds} seconds for the already fixed 880 cells. These L0 durations include startup, instrumentation and contention. They are not native performance observations or whole-IC time. The full per-call range and raw process outcomes remain in PREFLIGHT_RUNTIME.json and RECORDS.json.`,
  'Command: frozen harness run PREFLIGHT_PLAN.json WORKER runs/preflight-v1, then preflight-runtime PREFLIGHT_PLAN.json RECORDS.json PREFLIGHT_RUNTIME.json. Limits: 2-second cooperative solver budget after base construction, 4-second hard limit plus 1-second grace, 4 GiB, 32 MiB/file; prospective outer preflight watchdog 360 seconds. No independent correctness/performance admission is claimed for this operational phase.'
],[{name:'DURATIONS',caption:'Recorded operational durations; excluded from primary performance'}],['execution-v4/preflight.protocol.json','execution-v4/PREFLIGHT_RUNTIME.json']);
fs.writeFileSync(pd+'/PLOT_DATA.json',JSON.stringify(pre.classes,null,2)+'\n');
let pb='<text x="24" y="32" font-size="25" font-weight="bold">60 separate sizing attempts; no admitted performance time</text><text x="24" y="61" font-size="16">Operational elapsed seconds, startup and instrumentation included; preflight-v1.</text>';
for(let k=0;k<=5;k++)pb+=`<path d="M85 ${355-k*50} H955" stroke="#e0e5e8"/><text x="30" y="${361-k*50}">${k}s</text>`;
pre.classes.forEach((r,i)=>{pb+=`<circle cx="${93+i*14.3}" cy="${355-r.operational_elapsed_s*50}" r="4" fill="#1a7c72"/>`;});
pb+='<text x="85" y="397">Attempt index 0</text><text x="835" y="397">59</text><text x="24" y="439" font-size="16">Full fixture/arm/arity mapping in PLOT_DATA.json. No inference or confidence interval.</text>';
fs.writeFileSync(pd+'/DURATIONS.svg',svg(465,pb));
const cd=report('cost','Progression bases and companion equations: cold worker costs','cost-v1',[
  'All 880 planned cells were attempted: 751 independently accepted outputs, 105 cooperative-budget exits and 24 unsupported underfilled bases. The standardized export counts 775 conclusive process completions, including those 24 unsupported failures; scientific acceptance remains 751. By arm: direct F4 124/160, its A/A 124/160, helpers 153/160, companion 38/80, group MITM 156/160 and its A/A 156/160.',
  'The companion completes 38 arity-two cases and no arity-three cases under the registered budget. In the 38 admitted same-base comparisons it costs 1.2943715012885497–4.948801810455357 times direct F4 (median 1.352375384073076). Missing comparisons are not zero-cost results. Redundant auxiliary square equations do not impose a zero sum on auxiliary twist points.',
  'Eight helpers cost 0.12837712863374748–0.9085067663602162 times direct F4 across 124 admitted pairs (median 0.8937482746077081). This is an existing ordinary encoding effect. Direct F4 and helpers both lose to ordinary group MITM in every admitted comparison, medians 2.6611915882806296 and 2.3585326491871 times group Ir. Companion/group median is 3.4524782516950037 on 38 pairs.',
  'Within-base outputs are exactly equal under the independent signed-tuple census; decisive ratios require direct and group A/A checks within 5%. Across different constructors the answer sets differ. Their near-one raw cost ratios are descriptive only; they are not same-output speedups or costs per independent useful relation.',
  'Callgrind guest instructions Ir include cold process startup/parsing, target-blind base discovery, conversion and lifts, equation construction/F4, roots/signs, exceptional charts, deduplication, source verification and serialization. Parent/kernel, independent replay, relation-log linear algebra and full scalar recovery are excluded. Grevlex D32, GM1, linear roots enabled. Per worker: 2-second cooperative solver budget after construction, 4-second hard limit plus 1-second grace, 4 GiB, 32 MiB/file; global 2400 seconds/1 GiB.',
  `Actual strict PARI admission checks ${admission.PARI_positive_and_wrong_sign_each} positive sums and the same number of wrong-sign controls. Frozen masked analysis preceded labelled inspection; native unmasking verifies equal distributions. Numerical signatures can reveal method roles; the analyst and host are not independent. Command: frozen harness run PLAN.json WORKER runs/cost-v1, audit PLAN COST CENSUS cost-audit-v1, blind, unmask, then strict admit after the unchanged GP replay with a larger initial stack.`,
  'No new twist-specific index-calculus speedup is established. Larger fields, other transformations, arity four, scalar-preserving maps, fastest-external full-root comparison, independent-host replay, calibrated full-IC costs and matched rho remain open. The original complete campaign cards are not closed by this bounded diagnostic.'
],[{name:'COUNTS',caption:'Accepted outputs and all remaining attempts'},{name:'RATIOS',caption:'Admitted same-base Ir ratios, with sample sizes and spread'}],['execution-v4/cost.protocol.json','execution-v4/cost-audit-v1/AUDIT.json','execution-v4/cost-audit-v1/ADMISSION.json','execution-v4/blind-v1/UNMASKING.json','execution-v4/EXPORT_LIMITS.md'],table(['Candidate/control','Pairs','Min','Median','Max'],cost.ratio_distributions.map(r=>[r.comparison,r.ratio.pairs,r.ratio.min,r.ratio.median,r.ratio.max])));
counts(cd+'/COUNTS.svg',cost.by_arm,'751 scientifically accepted outputs / 880 attempts','cost-v1; no cells silently retried, refilled or removed');
ratios(cd+'/RATIOS.svg',cost.ratio_distributions);
fs.writeFileSync(cd+'/PLOT_DATA.json',JSON.stringify({by_arm:cost.by_arm,ratios:cost.ratio_distributions,strata:strata.by_family_arm_arity},null,2)+'\n');
fs.writeFileSync(root+'/RESULTS.md',`# Twist-derived progression follow-up, 2026-10-27\n\nShifted twist progressions supply valid original-curve bases, but this frozen experiment establishes no twist-specific solving gain. Ordinary shifts change coverage too. Group enumeration is cheaper in every admitted same-base comparison; adding companion equations costs more in all 38 admitted pairs.\n\nThe fixed 880-cell run has 751 accepted outputs, 105 cooperative caps and 24 underfilled bases. All raw attempts are preserved and independently audited; strict PARI verifies 1,841 positive and 1,841 wrong-sign checks. These are toy finite-decomposition results, not whole-IC or NIST-size costs.\n\n- [Exact geometry census](geometry/RESULTS.md), [PDF](geometry/RESULTS.pdf).\n- [Separate operational preflight](preflight/RESULTS.md), [PDF](preflight/RESULTS.pdf).\n- [Cold-worker cost comparisons](cost/RESULTS.md), [PDF](cost/RESULTS.pdf).\n- [Protocol binding limitation](../../execution-v4/EXPORT_LIMITS.md).\n- [Reproduction and artifacts](../../execution-v4/README.md).\n- [Unchanged incoming root test regressions](../../execution-v4/ROOT_CHECKS.md).\n\nThe benchmark copies are unadmitted diagnostics. Canonical IC leaderboard/scoreboard, progress timeline and curve/cover graphs were checked: no new validated pipeline row or scalar-preserving map changes their data, so they stay unchanged. Frozen earlier reports are preserved.\n\nThe next discriminating check is an exact all-shift source/twist overlap census and independent genus-two character-sum verification, under a new pre-data source/protocol freeze. It does not assume that a different progression transports a scalar.\n`);
fs.writeFileSync(root+'/REPORT_INDEX.json',JSON.stringify({schema:'twist.progression-report-index/v1',reports:['geometry','preflight','cost'],raw_archive_sha256:publication.sha256,scientific_aggregation_in_javascript:false},null,2)+'\n');
