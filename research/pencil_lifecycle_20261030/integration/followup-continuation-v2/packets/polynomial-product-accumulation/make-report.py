from pathlib import Path
import json,re,hashlib,subprocess
p=Path(__file__).parent
checks={x:json.loads((p/x/'gate-exit.json').read_text()) for x in ['word1','row1','product1','controls1','readback1']}
assert all(v['gate_exit_code']==0 and v['input_bindings_unchanged'] for v in checks.values())
rb=[]
for n,a in re.findall(r"'([^']+)' depends on axioms: \[([^\]]*)\]",(p/'readback1/stdout').read_text()):
 ax=[x.strip() for x in a.split(',')];assert set(ax)<= {'propext','Classical.choice','Quot.sound'};rb.append({'name':n,'axioms':ax})
for n in re.findall(r"'([^']+)' does not depend on any axioms",(p/'readback1/stdout').read_text()):rb.append({'name':n,'axioms':[]})
assert len(rb)==18
s={'schema':'research.proof-packet-status/v1','task':'T-1209','kind':'canonical_polynomial_product_accumulation','main_declarations':10,'control_declarations':8,'checks':checks,'readback':rb,'failed_attempts':[],'assumptions':['prime 0<p<2^31','canonical input words'],'criterion':'decoded product output equals product of decoded input polynomials','empty_branches':'empty input yields empty output','preserved':['canonical words','polynomial product decoding'],'independent_source_fidelity_review':'scoped model review completed; native full execution partial','independent_product_replay':{'main_declarations':10,'producer_controls':8,'reviewer_controls':6,'readbacks':24,'full_review_workflow_complete':False},'independent_monic_replay':{'main_declarations':19,'producer_controls':16,'reviewer_controls':4,'readbacks':39,'full_review_workflow_complete':False},'compiled_rust_equivalence':'not proved','quotient_and_instrumentation':'excluded','scientific_execution':False,'cost_ratio':None,'upstream_compiled_dependencies':'retained trust boundary'}
(p/'status.json').write_text(json.dumps(s,indent=2)+'\n')
body=(p/'REPORT.md').read_text()
(p/'REPORT.md').write_text(body)
labels=[['Canonical word update','r + x*y modulo p'],['One row at offset i','add x X^i B'],['All rows in order','accumulate A B'],['Zero and empty cases','trim output']]
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="140"><rect width="1000" height="140" fill="white"/>'
for i,lines in enumerate(labels):
 x=10+250*i
 svg+=f'<rect x="{x}" y="30" width="225" height="70" fill="#edf5f7" stroke="#17364a"/>'
 for j,t in enumerate(lines):svg+=f'<text x="{x+112}" y="{58+24*j}" text-anchor="middle" font-family="sans-serif" font-size="16">{t.replace(">","&gt;")}</text>'
 if i<3:svg+=f'<path d="M{x+225} 65h25l-6 -4m6 4l-6 4" stroke="#17364a" fill="none"/>'
svg+='</svg>'
(p/'flow.svg').write_text(svg)
def esc(s):
 for a,b in [('\\',r'\textbackslash{}'),('&',r'\&'),('%',r'\%'),('_',r'\_'),('#',r'\#'),('^',r'\textasciicircum{}'),('<',r'$<$'),('>',r'$>$')]:s=s.replace(a,b)
 return s
paragraphs=body.split('\n\n')
tex=r'\documentclass[10pt]{article}\usepackage[margin=.65in]{geometry}\usepackage[T1]{fontenc}\usepackage{graphicx}\usepackage{url}\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}\sloppy\begin{document}'
tex+=esc(paragraphs[0][2:])+ '\n\n'+esc(paragraphs[1])+r'\begin{center}\small\fbox{\shortstack{Canonical add-product\\bounded word arithmetic}} $\longrightarrow$ \fbox{\shortstack{Row-major updates\\bounded array indices}}\\[8pt] $\Downarrow$\\[8pt] \fbox{\shortstack{Decoded output = A times B\\canonical words preserved}}\end{center}'+'\n\n'+'\n\n'.join(esc(x) for x in paragraphs[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=(p/'render.log').open('w'),check=True)
(p/'manifest.json').write_text(json.dumps({str(q.relative_to(p)):hashlib.sha256(q.read_bytes()).hexdigest() for q in sorted(p.rglob('*')) if q.is_file() and q.name!='manifest.json' and q.suffix not in ['.olean','.png','.aux']},indent=2)+'\n')
