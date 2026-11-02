from pathlib import Path
import json,re,hashlib,subprocess
p=Path(__file__).parent
checks={x:json.loads((p/x/'gate-exit.json').read_text()) for x in ['descent2','fuel1','controls1','readback1']}
assert all(v['gate_exit_code']==0 and v['input_bindings_unchanged'] for v in checks.values())
rb=[]
for n,a in re.findall(r"'([^']+)' depends on axioms: \[([^\]]*)\]",(p/'readback1/stdout').read_text()):
 ax=[x.strip() for x in a.split(',')];assert set(ax)<= {'propext','Classical.choice','Quot.sound'};rb.append({'name':n,'axioms':ax})
assert len(rb)==17
s={'schema':'research.proof-packet-status/v1','task':'T-1196','kind':'monic_remainder_strict_descent_and_sufficient_fuel','main_declarations':9,'control_declarations':8,'checks':checks,'readback':rb,'failed_attempts':['descent1'],'failed_api_probe':['probe1'],'assumptions':['prime 0<p<2^31','canonical divisor and input words','divisor last word is one'],'derived_fuel_bound':'fuel >= input coefficient-list length','terminal_result':'empty remainder or remainder shorter than divisor','preserved':['canonical words','polynomial divisibility equivalence'],'independent_source_fidelity_review':'pending','compiled_rust_equivalence':'not proved','quotient_and_instrumentation':'excluded','scientific_execution':False,'cost_ratio':None,'upstream_compiled_dependencies':'retained trust boundary'}
(p/'status.json').write_text(json.dumps(s,indent=2)+'\n')
body='''# Monic remainder: strict descent and sufficient fuel

T-1196; 2026-10-31. Producer proof packet. Independent source-fidelity review pending.

Derived result. Under the existing prime-field and canonical-word assumptions, a divisor whose final stored word is one makes each active cancellation strictly shorten the trimmed remainder list. Fuel at least equal to the original input coefficient-list length suffices for the checked remainder model to return a result. That result is empty or shorter than the divisor, remains canonical, and preserves divisibility by the divisor polynomial. This extends T-1194's successful-return soundness with a sufficient-fuel theorem.

Construction. The generated cancellation has zero coefficient at the old final index. Canonical residues convert this field equality into an exact stored-word equality to zero. The word-array retains its allocated length during cancellation; trimming removes at least the final zero. Induction on fuel then proves termination of the bounded model. Initialization trimming cannot increase the input length. The bound is a mathematical upper bound on cancellation steps, not a measured runtime or a claim of a tight bound.

Checks. Nine main declarations and eight kernel-evaluated controls passed. Separate axiom readback covers all 17 declarations and uses only propext, Classical.choice and Quot.sound. Four final check receipts have native exit zero and unchanged input bindings. The controls show raw cancellation [16,0,1] to [16,1,0] modulo 17, strict trimming to [16,1], the next step to empty, exact division with input-length fuel, a nonzero terminal remainder, constant-one division and empty input. A nonmonic divisor [2] exhausts four steps of the unguarded loop; the monic hypothesis is essential to this proof.

Domain and interpretation. Assume prime 0<p<2^31, canonical input/divisor words and divisor last word equal to one. The native interpretation remains the monic branch of pdiv/prem over the odd prime field with diagnostic tables disabled. No quotient writes, instrumentation or compiled Rust execution are modeled. This proves termination of the explicit word/array/list model; a complete source-to-compiled-program equivalence and independent source-fidelity review remain separate tasks. Existing compiled dependencies retain the recorded trust boundary. No scientific trial or cost comparison was run.

Retained failures. The API probe referenced an unavailable list lemma. The first descent proof needed an explicit array-size witness and an explicit conversion to the local output-list expression; corrected proof passed. All failed receipts remain available. Nonfatal deprecation warnings are retained.

Sources. Native src/cryptanalysis/three_leads.rs pdiv/prem, the T-1194 monic remainder model, T-1193 polynomial cancellation, pinned Mathlib Algebra/Polynomial/Coeff.lean, and Mathlib's monic-division documentation (div_wf_lemma, inspected 2026-10-31): https://leanprover-community.github.io/mathlib4_docs/Mathlib/Algebra/Polynomial/Div.html. This is an implementation-specific connection of the standard leading-term cancellation argument, with no novelty claim.

Next construction. Connect the terminal remainder to the canonical polynomial remainder, and complete the independent source-fidelity review of native callers and guards. The full scientific cohort and its execution prerequisites remain unchanged.
'''
(p/'REPORT.md').write_text(body)
dot='''digraph G { graph [rankdir=LR,bgcolor="white",label="T-1196: monic remainder termination (producer-checked)",labelloc=t]; node [shape=box,style="rounded,filled",fillcolor="#edf5f7",fontname="Helvetica"]; a [label="Monic divisor\ncanonical words"]; b [label="Final coefficient = 0\nstored final word = 0"]; c [label="Trimmed length\nstrictly decreases"]; d [label="Fuel >= input length\nterminal result exists"]; a -> b -> c -> d; }'''
(p/'flow.dot').write_text(dot)
labels=[['Monic divisor','canonical words'],['Final coefficient = 0','stored final word = 0'],['Trimmed length','strictly decreases'],['Fuel >= input length','terminal result exists']]
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
tex+=esc(paragraphs[0][2:])+ '\n\n'+esc(paragraphs[1])+r'\begin{center}\small\fbox{\shortstack{Monic divisor\\canonical words}} $\longrightarrow$ \fbox{\shortstack{Final coefficient zero\\stored final word zero}}\\[8pt] $\Downarrow$\\[8pt] \fbox{\shortstack{Trimmed length decreases\\input-length fuel suffices}}\end{center}'+'\n\n'+'\n\n'.join(esc(x) for x in paragraphs[2:])+r'\end{document}'
(p/'report.tex').write_text(tex)
subprocess.run(['pdflatex','-interaction=nonstopmode','-halt-on-error','report.tex'],cwd=p,stdout=(p/'render.log').open('w'),check=True)
(p/'manifest.json').write_text(json.dumps({str(q.relative_to(p)):hashlib.sha256(q.read_bytes()).hexdigest() for q in sorted(p.rglob('*')) if q.is_file() and q.name!='manifest.json' and q.suffix not in ['.olean','.png','.aux']},indent=2)+'\n')
