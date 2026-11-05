from pathlib import Path
import json,re,hashlib,subprocess
p=Path(__file__).parent
checks={x:json.loads((p/x/'gate-exit.json').read_text()) for x in ['normalized1','zerotest1','controls3','readback1']}
assert all(v['gate_exit_code']==0 and v['input_bindings_unchanged'] for v in checks.values())
rb=[]
for n,a in re.findall(r"'([^']+)' depends on axioms: \[([^\]]*)\]",(p/'readback1/stdout').read_text()):
 ax=[x.strip() for x in a.split(',')];assert set(ax)<= {'propext','Classical.choice','Quot.sound'};rb.append({'name':n,'axioms':ax})
for n in re.findall(r"'([^']+)' does not depend on any axioms",(p/'readback1/stdout').read_text()):rb.append({'name':n,'axioms':[]})
assert len(rb)==18
s={'schema':'research.proof-packet-status/v1','task':'T-1198','kind':'monic_remainder_exact_divisibility_test','main_declarations':10,'control_declarations':8,'checks':checks,'readback':rb,'failed_attempts':['controls1','controls2'],'assumptions':['prime 0<p<2^31','canonical divisor and input words','divisor last word is one'],'fuel_assumption':'fuel >= input coefficient-list length','criterion':'checked remainder = Some(empty) iff divisor polynomial divides input','terminal_result':'empty remainder or remainder shorter than divisor','preserved':['canonical words','polynomial divisibility equivalence'],'independent_source_fidelity_review':'pending','compiled_rust_equivalence':'not proved','quotient_and_instrumentation':'excluded','scientific_execution':False,'cost_ratio':None,'upstream_compiled_dependencies':'retained trust boundary'}
(p/'status.json').write_text(json.dumps(s,indent=2)+'\n')
body=(p/'REPORT.md').read_text()
(p/'REPORT.md').write_text(body)
dot='''digraph G { graph [rankdir=LR,bgcolor="white",label="T-1198: exact monic remainder criterion (producer-checked)",labelloc=t]; node [shape=box,style="rounded,filled",fillcolor="#edf5f7",fontname="Helvetica"]; a [label="Sufficient fuel\nnormalized output"]; b [label="Shorter remainder\ndegree below divisor"]; c [label="Divisible remainder\nmust be zero"]; d [label="Canonical and normalized\nzero iff empty"]; a -> b -> c -> d; }'''
(p/'flow.dot').write_text(dot)
labels=[['Sufficient fuel','terminal normalized list'],['Remainder degree','below divisor degree'],['Divisible remainder','must be zero'],['Canonical + normalized','zero iff empty']]
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
tex+=esc(paragraphs[0][2:])+ '\n\n'+esc(paragraphs[1])+r'\begin{center}\small\fbox{\shortstack{Sufficient fuel\\normalized output}} $\longrightarrow$ \fbox{\shortstack{Shorter remainder\\degree below divisor}}\\[8pt] $\Downarrow$\\[8pt] \fbox{\shortstack{Divisibility iff empty remainder\\canonical words required}}\end{center}'+'\n\n'+'\n\n'.join(esc(x) for x in paragraphs[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=(p/'render.log').open('w'),check=True)
(p/'manifest.json').write_text(json.dumps({str(q.relative_to(p)):hashlib.sha256(q.read_bytes()).hexdigest() for q in sorted(p.rglob('*')) if q.is_file() and q.name!='manifest.json' and q.suffix not in ['.olean','.png','.aux']},indent=2)+'\n')
