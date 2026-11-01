from pathlib import Path
import json
p=Path(__file__).parent
s=json.loads((p/'status.json').read_text());r=json.loads((p/'review-plan.json').read_text())
def esc(t):
 for a,b in [('\\',r'\textbackslash{}'),('_',r'\_'),('%',r'\%'),('&',r'\&'),('#',r'\#'),('^',r'\textasciicircum{}')]:t=t.replace(a,b)
 return t
parts=[r'\documentclass[10pt]{article}\usepackage[margin=.7in]{geometry}\usepackage[T1]{fontenc}\setlength{\parindent}{0pt}\setlength{\parskip}{7pt}\sloppy\begin{document}',r'\textbf{Normalization source-fidelity review contract}\\T-1187; 2026-10-31.',f"Producer-organized preparation; independent verdict pending. Bound {s['packets_bound']} packets, checked {s['included_files_hash_checked']} included custody hashes, and matched {s['selected_index_bound_inputs']} selected input files to Git index blobs at producer head "+r'\texttt{3388a75cf5}.', 'These checks bind bytes, not theorem truth or execution authenticity. No fresh proof replay or scientific execution occurred. Upstream compiled dependencies retain their trust boundary. Each role below is unassigned; independent reviewers must work in separate sessions and seal reports before seeing other findings.']
for i,x in enumerate(r['joints']):
 if i in [4,8]:parts.append(r'\newpage')
 parts += [r'\textbf{'+str(i+1)+'. '+esc(x['joint'])+'}',esc('Claim: '+x['claim']),esc('Attack: '+x['worked_attack']),esc('Blind re-derivation: '+x['blind_rederivation'])]
parts += ['Required outputs: exact named-module rebuilds and fresh axiom audits in an isolated immutable input tree; clause-by-clause source mapping; explicit domains or counterexamples; independently authored scoped findings.', 'Runtime reconciliation: the saved mount-diff amendment remains unreviewed and unintegrated. The prior qualification remains failed. The campaign deadline remains 2026-10-31 06:00 UTC; no renewal, launch or reinterpretation of old results is made here.',r'Sources: review-evidence/references/lean-review.md; corrected canonical source map; preceding Mathlib affine-substitution references. Full paths and SHA256 bindings are in bindings.json.\end{document}']
(p/'report.tex').write_text('\n\n'.join(parts))
labels=['Source identity','Arithmetic','Traversal/storage','Metadata','Identity branch','Validity','Inverse/sign','Multiplicity','Proof trust','Geometry']
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1100" height="470"><rect width="1100" height="470" fill="white"/><g font-family="sans-serif" fill="#17364a" text-anchor="middle"><text x="550" y="30" font-size="22">Independent review map · 10 checks · no verdicts yet</text>']
for i,label in enumerate(labels):
 x=30+(i%5)*215;y=70+(i//5)*140
 svg += [f'<rect x="{x}" y="{y}" width="195" height="95" fill="#edf5f7" stroke="#17364a"/><text x="{x+97}" y="{y+35}">{i+1}. {label}</text><text x="{x+97}" y="{y+65}">Reviewer unassigned</text>']
svg += ['<text x="550" y="365">13 packets · 902 included hashes checked · 107 selected index-bound inputs</text><text x="550" y="410">Each check: exact claim + worked attack + blind re-derivation</text><text x="550" y="445">Fresh independent replay and source-fidelity findings still required</text></g></svg>']
(p/'flow.svg').write_text(''.join(svg))
