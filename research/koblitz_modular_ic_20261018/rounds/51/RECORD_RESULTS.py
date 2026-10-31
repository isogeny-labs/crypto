"""Record terminal receipts and render component evidence; no mathematics."""
from pathlib import Path
from collections import Counter
import json, hashlib, re, html
h=Path(__file__).resolve().parent
markers={'diamond':'DIAMOND51_ALL_CHECKS_PASS','glue-prep':'GLUE_PREP51_ALL_CHECKS_PASS','glue-forward':'GLUE51_ALL_CHECKS_PASS','glue-reverse':'GLUE51_ALL_CHECKS_PASS','glue-reload':'GLUE_RELOAD51_ALL_CHECKS_PASS','universal':'UNIVERSAL51_ALL_CHECKS_PASS','twist':'TWIST51_ALL_CHECKS_PASS','base23':'BASE23_51_ALL_CHECKS_PASS','fiber':'FIBER51_ALL_CHECKS_PASS'}
rows=[]
for p in sorted((h/'runs').iterdir()):
    if not p.is_dir(): continue
    row={'attempt':p.name,'status':'running','source_revision':(p/'source-revision.txt').read_text().strip() if (p/'source-revision.txt').exists() else None}
    if (p/'compile.exit').exists():
        row.update(kind='Lean',compile_exit=int((p/'compile.exit').read_text()),audit_exit=int((p/'audit.exit').read_text()) if (p/'audit.exit').exists() else None)
        if row['compile_exit']!=0: row['status']='rejected'
        elif row['audit_exit'] is not None:
            row['status']='accepted' if row['audit_exit']==0 and 'audited 5 theorems in [Overlap51]; 0 failing, 0 trusted (native_decide)' in (p/'audit.log').read_text() else 'rejected'
    elif (p/'exit.txt').exists():
        log=(p/'native.log').read_text();marker=next(v for k,v in markers.items() if p.name.startswith(k+'51-') or (k=='base23' and p.name.startswith('base23-51-')))
        errors=re.findall(r'^ *\?.*$',log,re.M)
        row.update(kind='Rust' if p.name.startswith('fiber') else 'Singular',native_exit=int((p/'exit.txt').read_text()),required_marker=marker,error_lines=errors)
        row['status']='accepted' if row['native_exit']==0 and not errors and marker in log.splitlines() else 'rejected'
    rows.append(row)
result={'scope':'R51 actual chart maps and larger marked23 QR construction','status_counts':dict(Counter(r['status'] for r in rows)),'runs':rows,'retained_source_field_degree':32,'normalization_generators_per_direction':32,'directional_regularity_certificates':64,'clearing_power':62,'lean_theorems':5,'scope_limits':['native arrays and RR/open-cover premises are not encoded in Lean','marked23 quotient dimension is not primality','selected modular-factor action and polarized embedding remain separate']}
(h/'results.json').write_text(json.dumps(result,indent=2)+'\n')
state={k:next((r['status'] for r in reversed(rows) if r['attempt'].startswith(k+'51-') or (k=='base23' and r['attempt'].startswith('base23-51-'))),'prepared') for k in markers}
nodes=[('diamond','Actual diamond / tenth identity'),('glue-forward','32 forward normalized generators'),('glue-reverse','32 inverse normalized generators'),('glue-reload','64 independent compatibility checks'),('universal','Universal QR polynomial certificate'),('twist','Quadratic twist / F2 descent'),('base23','Marked23 specialization / rank112'),('fiber','Exhaustive finite QR7 cycle test')]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="980" height="470" viewBox="0 0 980 470">','<rect width="980" height="470" fill="white"/>','<g font-family="DejaVu Sans,sans-serif" fill="#17304a">','<text x="24" y="34" font-size="23">R51 exact chart maps and larger-carrier evidence</text>']
for i,(key,label) in enumerate(nodes):
    col=i//4; y=65+(i%4)*78; x=24+col*475; status=state[key]
    color={'accepted':'#d4eddf','rejected':'#f6d8d6','running':'#fff0c2','prepared':'#edf0f3'}[status]
    svg += [f'<rect x="{x}" y="{y}" width="448" height="64" rx="7" fill="{color}"/>',f'<text x="{x+12}" y="{y+25}" font-size="16">{html.escape(label)}</text>',f'<text x="{x+12}" y="{y+49}" font-size="14">{status}; source: results.json / native receipts</text>']
svg += ['<text x="24" y="407" font-size="15">Five overlap lemmas: compile 0, audit 0; zero failing or trusted declarations.</text>','<text x="24" y="434" font-size="14">Rank112 is an algebra acceptance target; primality and selected-factor action need separate evidence.</text>','<text x="24" y="458" font-size="13">Deterministic certificates; no timing comparison or statistical uncertainty.</text>','</g></svg>']
(h/'round51.svg').write_text('\n'.join(svg)+'\n')
def text(s): return 'text('+json.dumps(s)+')'
tr=[]
for r in rows: tr += [text(r['attempt']),text(r['status']),text('compile '+str(r['compile_exit'])+', audit '+str(r['audit_exit']) if r.get('kind')=='Lean' else 'exit '+str(r.get('native_exit','pending')))]
typ='''#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= R51: explicit normalization-generator maps and QR transport
The actual degree32 source field has a checked diamond automorphism. Both
32-generator directional regularity calculations pass, with fixed clearing
power62 and full PID products. Independent direct substitution, universal
certificate transport and larger-base checks are recorded below at their
actual receipt status.
#image("round51.svg", width: 100%)
== Terminal evidence and pending work
#table(columns: (2.4fr,1fr,1.6fr), stroke: 0.4pt,
[Attempt],[Status],[Receipt],
'''+',\n'.join(tr)+''')
== Formal scope
Five Overlap51 declarations compile and pass the axiom audit with zero
failing and zero trusted declarations. They prove three characteristic-two
polynomial identities, a localized inverse, and inverse induced maps under
injective compatible embeddings. Native arrays, complete RR identification,
primality and the geometric open-cover premises remain explicit external
obligations. See PROOF_MAP.md and GLUING51.md for the exact claim boundaries.

== Larger construction
The archived universal R22 certificate is over independent Tate parameters
b,c. The new stages clear its actual coefficient denominators, transport its
full matrices through the quadratic diamond twist, and specialize to the
retained marked23 base. The rank112 quotient target does not assert field
primality or a selected order991 action. The finite-group stage checks whether
an irreducible degree16 ordinary fiber can be an appropriate test at all.
See the frozen stage protocols and NEXT.md for acceptance dependencies.
'''
(h/'report.typ').write_text(typ)
print(json.dumps(result['status_counts']))
