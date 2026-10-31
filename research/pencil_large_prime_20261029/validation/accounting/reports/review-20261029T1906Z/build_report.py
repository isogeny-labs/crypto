from pathlib import Path
import json,hashlib,datetime,re,subprocess
HERE=Path(__file__).resolve().parent
STUDY=HERE.parents[3]
def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()
rels=['validation/accounting/aggregate/actual-plan-only-summary-final.json','validation/accounting/finite-nine-category-source-audit.json','validation/accounting/consolidated-fixes/final-copy-producer-tests.log','validation/accounting/aggregate/integrated-tests.log','validation/accounting/consolidated-fixes/final-copy-exclusive-test.log','prepared_native_plans/result.json','validation/accounting/COST_BOUNDARY.md','validation/accounting/post-closure-root-release-lib-20261029T1855Z.source-manifest.json','validation/accounting/post-closure-root-release-lib-20261029T1855Z.source-hash-assertion.json']
evidence=[{'id':f'E{i+1}','path':r,'sha256':sha(STUDY/r)} for i,r in enumerate(rels)]
a=json.loads((STUDY/rels[0]).read_text()); audit=json.loads((STUDY/rels[1]).read_text())
assert a['expected_workloads']==1260 and a['observed_executions']==0
assert len(a['aggregate_cells'])==36 and all(c['algorithm_ratio_of_sums'] is None for c in a['aggregate_cells'])
tests=[]
for i,name in [(2,'producer'),(3,'native analyzer'),(4,'isolated strict measurement')]:
 t=(STUDY/rels[i]).read_text();m=re.search(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;',t);assert m
 tests.append({'name':name,'passed':int(m[1]),'failed':int(m[2]),'ignored':int(m[3]),'evidence':f'E{i+1}'})
report={'schema':'pencil-accounting-review-report/v1','created_at_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'scope':'Public synthetic known-answer mathematics; existing software and source review evidence only. No new cohort execution.','observation_boundary':'Observed execution counts apply to this plan-only input, not to historical experiments elsewhere.','cohort':{k:a[k] for k in ['expected_workloads','expected_measured_pairs','expected_executions','expected_warmup_executions','observed_executions','missing_executions_count','pair_rows_count']},'new_cohort_targets_executed':0,'ratios':a['aggregate_cells'],'software_tests':tests,'full_root_release_tests':{'status':'passed','passed':5277,'failed':0,'ignored':123,'source_files_unchanged':20,'owner':'root/ci_audit','session_id':15062,'log_prefix':'validation/accounting/post-closure-root-release-lib-20261029T1855Z','terminal_result':json.loads((STUDY/rels[8]).read_text())['validation']},'source_review':{'status':audit['status'],'categories':[{'category':c['category'],'disposition':c['disposition'],'uncovered_sites':c['uncovered_sites']} for c in audit['categories']],'reviewed_complete_emitted':audit['reviewed_complete_emitted'],'cohort_price_ready':audit['cohort_price_ready'],'admission_pending':audit['admission_pending']},'mapping':a['mapping'],'AA_rule':a['AA_rule'],'evidence':evidence,'interpretation':'No observed cohort cost ratio is available in this snapshot. Software checks and finite source review do not constitute protocol or binary qualification.'}
(HERE/'report.json').write_text(json.dumps(report,indent=2)+'\n')
boxes=[('Prepared plans',['1,260 workloads','5 measured + 1 warmup']),('Records',['22,680 planned slots','0 observed in this input']),('Source + identity',['Finite source reviewed','Bindings pending']),('Pair pricing',['6,300 measured pairs','36 sensitivity cells']),('Cohort aggregate',['Exact ratio of sums','All ratios: null'])]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="350" viewBox="0 0 1200 350"><rect width="1200" height="350" fill="white"/><defs><marker id="arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M0,0 L8,4 L0,8" fill="#486276"/></marker></defs><g font-family="DejaVu Sans, sans-serif" fill="#183042">']
for i,(title,lines) in enumerate(boxes):
 x=10+i*240
 svg.append(f'<rect x="{x}" y="32" width="218" height="142" rx="8" fill="#eef4f7" stroke="#486276" stroke-width="2"/><text x="{x+109}" y="66" text-anchor="middle" font-size="21" font-weight="bold">{title.replace("+","+")}</text>')
 for j,line in enumerate(lines):svg.append(f'<text x="{x+109}" y="{106+j*31}" text-anchor="middle" font-size="17">{line}</text>')
 if i<4:svg.append(f'<path d="M{x+219},103 H{x+239}" fill="none" stroke="#486276" stroke-width="2" marker-end="url(#arrow)"/>')
svg+=['<path d="M600,177 V218" stroke="#486276" stroke-width="2" marker-end="url(#arrow)"/><rect x="180" y="225" width="840" height="76" rx="8" fill="#fff4df" stroke="#987135" stroke-width="2"/><text x="600" y="256" text-anchor="middle" font-size="21">Missing, censored or unqualified records retain null prices</text><text x="600" y="284" text-anchor="middle" font-size="17">No survivor-only aggregate; software fixtures are not cohort observations</text></g></svg>']
(HERE/'pipeline.svg').write_text(''.join(svg))
import cairosvg
cairosvg.svg2pdf(url=str(HERE/'pipeline.svg'),write_to=str(HERE/'pipeline.pdf'))
def esc(s):
 for a,b in [ ('\\',r'\textbackslash{}'),('_',r'\_'),('&',r'\&'),('%',r'\%'),('#',r'\#')]:s=s.replace(a,b)
 return s
tex=r'''\documentclass[11pt]{article}
\usepackage[margin=0.7in]{geometry}
\usepackage{graphicx,booktabs,xcolor,hyperref}
\hypersetup{colorlinks=true,urlcolor=blue}
\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}
\begin{document}
{\LARGE Pencil accounting: review snapshot}\\[4pt]
{\large Public synthetic known-answer mathematics}\\
29 October 2026, 19:10 UTC snapshot. Evidence-only report; no new cohort execution.

The existing plan-only aggregation input contains zero observed executions. It describes 1,260 workloads and retains null cost ratios in all 36 sensitivity cells. This count applies to the named input, not to historical experiments elsewhere. The observations below distinguish software validation, finite source review and future experimental admission.

\begin{center}\begin{tabular}{lr}\toprule
Plan-derived quantity [E1, E6] & Count\\\midrule
Workloads & 1,260\\
Measured candidate/reference pairs (5 streams per workload) & 6,300\\
Measured arm executions (candidate, rho, rho\_AA) & 18,900\\
Warmup arm executions (1 per arm and workload) & 3,780\\
Total expected execution slots & 22,680\\
Observed executions in this input & 0\\
Missing execution slots & 22,680\\\bottomrule\end{tabular}\end{center}
\includegraphics[width=\linewidth]{pipeline.pdf}

\textbf{Software validation.} The producer suite reports 49 passed, 0 failed and 1 ignored [E3]. The native analyzer reports 28 passed and 0 failed [E4]. The separately selected strict measurement test reports 1 passed and 49 filtered out [E5]. These are software tests, not sampled experimental targets. The full root release-library suite reports 5,277 passed, 0 failed and 123 ignored, with exit status 0. All 20 reviewed source files were unchanged after the run [E9].

\textbf{Aggregation rule.} The aggregate uses the ratio of exact sums over the prescribed cohort; missing or ineligible pairs prevent a qualified ratio. Each target's five measured rho\_AA/rho ratios must individually lie in the inclusive interval [0.90,1.10], separately for cold and online timing. This is an operational tolerance, not a confidence interval [E1].
\newpage
{\Large Source review and accounting boundary}

The finite source audit records \texttt{source\_review\_complete\_binding\_pending} [E2]. Its nine selected categories are field arithmetic; scalar arithmetic; outer and constructor validation; setup and transfer; collection, graph and elimination; hash and random generation; source verification; metadata cloning; and calibration. Each is recorded as covered under the declared finite model with no remaining listed uncovered site. This is a source-level review result, not a universal instruction census or an admitted experimental binary.

The finite algorithm model uses the inherited 36-cell sensitivity grid: square weight $S/M\in\{0,0.5,1\}$; inversion weight $I/M\in\{1,10,100,1000\}$; native-event weight $\{0,0.1,1\}$. Disjoint atomic scalar events, actual collection primitives, hash/random-generation events and copied metadata nodes follow the declared mapping. Diagnostic totals are not charged again [E1, E7].

Report formatting, serialization, allocation, RSS, wall and CPU time, parent verification, harness work, infrastructure and retries remain separately named resources. They are not assigned an invented conversion to field operations and are not assumed to cost zero. Calibration is retained separately and included once in the combined ratio [E1, E7]. No wall-time, memory or infrastructure measurements are presented here.

\textbf{Admission still pending at the source-audit snapshot.} An approved frozen protocol identifying policy and calibration treatment; exact instrument binary digest; reproducible build receipt binding the reviewed sources, Rust 1.99.1 and binary; an independent write-once coverage record satisfying the analyzer schema. The final root test and post-run source-hash checks have since passed [E9]. Remaining admission bindings are listed in the current audit [E2]. The audit's \texttt{reviewed\_complete\_emitted} and \texttt{cohort\_price\_ready} fields are both false. Prepared-plan integrity alone does not establish approval or authenticity.

\textbf{Evidence locations.} Paths below are relative to \texttt{research/pencil\_large\_prime\_20261029/}. Full SHA-256 digests and structured quantities are retained in \texttt{report.json}; generated-file hashes are in \texttt{SHA256SUMS}. The editable diagram is \texttt{pipeline.svg}, and this report's source is \texttt{report.tex}.
\begin{description}
'''
for e in evidence:tex+=r'\item['+e['id']+r'] {\footnotesize\path{'+e['path']+'}}\n'
tex+=r'\end{description}\end{document}'
(HERE/'report.tex').write_text(tex)
subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=HERE,stdout=(HERE/'build.log').open('w'),check=True)
print(HERE)
