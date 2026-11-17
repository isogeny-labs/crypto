from pathlib import Path
import json,hashlib,subprocess,datetime
p=Path(__file__).parent;r=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/preflight-review');m=Path('/var/tmp/pencil-flock-barrier-check-20261031-v1')
sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
refs=[]
for group,base,names in [('typed',r/'final-software-checks/typed-retained',['stdout','receipt.json','command.json']),('full',r/'final-software-checks/full-lib',['stdout','receipt.json','command.json']),('isolated',r/'gpu-capsule-failure-replay',['stdout','receipt.json','command.json']),('mechanism',m,['PLAN.md','check.py','preexecution.json','stdout','receipt.json','manifest.json'])]:
 for n in names:refs.append({'group':group,'path':str(base/n),'sha256':sha(base/n)})
pre=json.loads((m/'preexecution.json').read_text())
for n,h in pre.items():assert sha(m/n)==h
manifest=json.loads((m/'manifest.json').read_text())
for n,d in manifest['files'].items():assert sha(m/n)==d['sha256']
obs=json.loads((m/'stdout').read_text());receipt=json.loads((m/'receipt.json').read_text());assert receipt['exit_code']==0 and obs['passed']
assert obs['records'][1]['attempt']['errno']==11
assert obs['records'][0]['second']['acquired'] and obs['records'][2]['attempt']['acquired']
status={'schema':'software.validation_snapshot/v1','created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'scope':'software validation, separate from scientific results','review_kind':'producer self-audit; not independent','source_or_runtime_edits':False,'checks':[{'name':'corrected retained-data typed wrapper','result':'passed','passed':3,'failed':0,'ignored':0},{'name':'full library default-concurrency run','result':'failed','passed':5792,'failed':1,'ignored':128,'failure':'gpu.rs1270 nonblocking BenchLock reacquisition'},{'name':'isolated capsule replay','result':'passed','passed':1,'failed':0},{'name':'deterministic inherited-FD mechanism','result':'passed','attempts':1,'errno_while_child_holds':11,'original_failure_cause_established':False},{'name':'reduced-concurrency full run and additional checks','result':'pending at snapshot; owned by root'}],'references':refs,'mechanism_self_audit':{'all_manifest_hashes_match':True,'preexecution_hashes_match':True,'control_receipts_match_claims':True,'source_checked':True,'limitations':['producer self-audit','no independent replay','no original failing syscall trace','POSIX Python flock/fork diagnostic, not actual Rust BenchLock reproduction']}}
(p/'snapshot.json').write_text(json.dumps(status,indent=2)+'\n')
(p/'SELF-AUDIT.md').write_text('''# Producer self-audit

The reviewer here authored the diagnostic: this is not independent review. Source and existing receipts were checked without repeating execution. All manifest and preexecution source/plan hashes match. The child signals readiness only after fstat verifies the inherited descriptor; it waits on the second pipe before closing it. Parent closes its own descriptor before the positive acquisition, then signals child closure and waits for exit before the negative acquisition. The exact errno 11 is retained. Baseline reacquisition succeeds. The reported controls and exit zero match raw stdout/receipt.

The diagnostic uses Python POSIX fork/flock and a temporary file, not the Rust BenchLock API. It establishes the inherited-descriptor mechanism but does not establish the cause of the original Rust failure. FD_CLOEXEC is recorded; no exec is performed. An internal alarm bounds the process and an outer timeout bounds the invocation. A failed assertion inside the try block attempts child kill/reap; abrupt alarm termination is not claimed to provide that finally cleanup. This successful execution did reap the child with exit zero. No repeated tests or repairs were performed for this audit.

The PDF and SVG summarize a point-in-time software validation state. Full-library failure remains present. Reduced-concurrency and additional root checks are pending in this snapshot; later results require an additive update.
''')
items=[('Typed retained inputs','3 passed / 0 failed'),('Full library run','5792 passed / 1 failed / 128 ignored'),('Isolated capsule replay','1 passed / 0 failed'),('Inherited-FD diagnostic','holds: EAGAIN; closed/reaped: acquired'),('Root follow-up checks','PENDING at snapshot')]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="410" viewBox="0 0 1000 410"><rect width="1000" height="410" fill="white"/><g font-family="sans-serif" fill="#182433"><text x="24" y="35" font-size="23">Software validation snapshot</text>']
for i,(a,b) in enumerate(items):
 y=62+i*57;svg.append(f'<rect x="20" y="{y}" width="960" height="47" rx="5" fill="#eef2f6"/><text x="32" y="{y+29}" font-size="17">{a}</text><text x="335" y="{y+29}" font-size="17">{b}</text>')
svg.append('<text x="24" y="380" font-size="15">Mechanism observed; original failure cause unproved. Producer self-audit.</text></g></svg>')
(p/'snapshot.svg').write_text(''.join(svg))
subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'snapshot-figure.pdf'),str(p/'snapshot.svg')],check=True)
tex=r'''\documentclass[10pt]{article}
\usepackage[margin=0.65in]{geometry}\usepackage{graphicx}\usepackage{hyperref}
\begin{document}
\section*{Software validation snapshot}
\includegraphics[width=\linewidth]{snapshot-figure.pdf}
\noindent The full-library run retains one failure at GPU capsule lock reacquisition. The isolated replay does not replace that outcome. The deterministic pipe-barrier check observes errno 11 while an inherited descriptor remains open, followed by successful acquisition after child closure and reap. It does not identify the original failing interleaving.

\paragraph{Review boundary} Producer self-audit only. Source/receipt hashes and barrier ordering were checked without another execution. No scientific observations or ratios are reported here. Root-owned reduced-concurrency and additional checks remain pending at this snapshot.

\paragraph{Evidence} Exact absolute paths and full SHA256 hashes are retained in \texttt{snapshot.json}; raw evidence remains in the referenced directories. The diagnostic and its original diagnosis packet remain unchanged. This document is an additive point-in-time record.
\end{document}
'''
(p/'report.tex').write_text(tex)
x=subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,capture_output=True);(p/'render.stdout').write_bytes(x.stdout);(p/'render.stderr').write_bytes(x.stderr);assert x.returncode==0
subprocess.run(['pdftoppm','-f','1','-singlefile','-scale-to','1200','-png',str(p/'report.pdf'),str(p/'preview')],check=True)
(p/'manifest.json').write_text(json.dumps({'files':{q.name:{'sha256':sha(q),'bytes':q.stat().st_size} for q in sorted(p.iterdir()) if q.is_file() and q.name!='manifest.json'}},indent=2)+'\n')
print('manifest',sha(p/'manifest.json'))
