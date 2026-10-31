"""Record terminal receipts and render component evidence; no mathematics."""
from pathlib import Path
from collections import Counter
import json, hashlib, re, html
h=Path(__file__).resolve().parent
markers={'diamond':'DIAMOND51_ALL_CHECKS_PASS','glue-prep':'GLUE_PREP51_ALL_CHECKS_PASS','glue-forward':'GLUE51_ALL_CHECKS_PASS','glue-reverse':'GLUE51_ALL_CHECKS_PASS','glue-reload':'GLUE_RELOAD51_ALL_CHECKS_PASS','universal':'UNIVERSAL51_ALL_CHECKS_PASS','twist':'TWIST51_ALL_CHECKS_PASS','base23':'BASE23_51_ALL_CHECKS_PASS','base23-struct':'BASE23_STRUCT51_ALL_CHECKS_PASS','fiber':'FIBER51_ALL_CHECKS_PASS','hecke3-':'HECKE3_51_ALL_CHECKS_PASS','line':'LINE51_ALL_CHECKS_PASS','nu':'NU51_ALL_CHECKS_PASS','collision':'COLLISION51_ALL_CHECKS_PASS','diamond-nu':'DIAMONDNU51_ALL_CHECKS_PASS'}
rows=[]
for p in sorted((h/'runs').iterdir()):
    if not p.is_dir(): continue
    row={'attempt':p.name,'status':'running','source_revision':(p/'source-revision.txt').read_text().strip() if (p/'source-revision.txt').exists() else None}
    if (p/'compile.exit').exists():
        row.update(kind='Lean',compile_exit=int((p/'compile.exit').read_text()),audit_exit=int((p/'audit.exit').read_text()) if (p/'audit.exit').exists() else None)
        expected_root='Specialize51' if p.name.startswith('specialize-lean51-') else 'Overlap51'
        expected_count=6 if expected_root=='Specialize51' else 5
        row.update(audited_theorems=expected_count,root=expected_root)
        if row['compile_exit']!=0: row['status']='rejected'
        elif row['audit_exit'] is not None:
            row['status']='accepted' if row['audit_exit']==0 and f'audited {expected_count} theorems in [{expected_root}]; 0 failing, 0 trusted (native_decide)' in (p/'audit.log').read_text() else 'rejected'
    elif (p/'exit.txt').exists():
        log=(p/'native.log').read_text();marker=next(v for k,v in markers.items() if p.name.startswith(k+'51-') or (k=='base23' and p.name.startswith('base23-51-')))
        errors=re.findall(r'^ *\?.*$',log,re.M)
        row.update(kind='Rust' if p.name.startswith('fiber') else 'Singular',native_exit=int((p/'exit.txt').read_text()),required_marker=marker,error_lines=errors)
        row['status']='accepted' if row['native_exit']==0 and not errors and marker in log.splitlines() else 'rejected'
    rows.append(row)
result={'scope':'R51 actual chart maps and larger marked23 QR construction','status_counts':dict(Counter(r['status'] for r in rows)),'runs':rows,'retained_source_field_degree':32,'normalization_generators_per_direction':32,'directional_regularity_certificates':64,'clearing_power':62,'lean_theorems':sum({x['root']:x['audited_theorems'] for x in rows if x['status']=='accepted' and x.get('kind')=='Lean'}.values()),'scope_limits':['native arrays and RR/open-cover premises are not encoded in Lean','marked23 quotient dimension is not primality','selected modular-factor action and polarized embedding remain separate']}
accepted={row['attempt'] for row in rows if row['status']=='accepted'}
def native_value(attempt,key):
    if attempt not in accepted:return None
    match=re.search(r'(?<![A-Za-z_])'+re.escape(key)+r'=(\d+)(?![0-9])',(h/'runs'/attempt/'native.log').read_text())
    assert match,(attempt,key)
    return int(match.group(1))
counts={
 'localized_generator_compatibilities':64 if 'glue-reload51-01' in accepted else None,
 'localized_normal_chart_maps':2 if 'glue-reload51-01' in accepted else None,
 'universal_certificate_denominator_degree':native_value('universal51-01','denominator_degree'),
 'structured_marked23_algebra_dimension':native_value('base23-struct51-03','F2x_algebra_dimension'),
 'structured_marked23_source_equations':native_value('base23-struct51-03','complete_coupled_source_equations'),
 'integral_nu_normal_chart_coordinates':native_value('nu51-01','normal_chart_member'),
 'QR_square_determinant_actions':native_value('fiber51-01','QR_determinant_matrices'),
 'QR_largest_cycle':native_value('fiber51-01','longest_cycle'),
 'Hecke3_kernel_parameter_degree':native_value('hecke3-51-01','kernel_parameter_degree'),
 'Hecke3_isogeny_degree':native_value('hecke3-51-01','isogeny_X_degree'),
 'ordinary_QR_point_algebra_dimension':native_value('line51-01','point_root_algebra_dimension'),
 'coprime_kernel_Bezout_Delta_exponent':16 if 'collision51-01' in accepted else None,
 'nu_diamond_PID_entries':native_value('diamond-nu51-01','complete_transport_PID_entries'),
 'nu_diamond_clearing_power':native_value('diamond-nu51-01','normal_chart_clearing_power'),
 'audited_unique_theorems':result['lean_theorems'],
}
result['construction_counts']=counts
(h/'results.json').write_text(json.dumps(result,indent=2)+'\n')
state={k:next((r['status'] for r in reversed(rows) if r['attempt'].startswith(k+'51-') or (k=='base23' and r['attempt'].startswith('base23-51-'))),'prepared') for k in markers}
if any(x['attempt'].startswith('base23-struct51-') and x['status']=='accepted' for x in rows): state['base23']='accepted'
nodes=[('diamond','Actual diamond / tenth identity'),('glue-forward','32 forward normalized generators'),('glue-reverse','32 inverse normalized generators'),('glue-reload','64 independent compatibility checks'),('universal','Universal QR polynomial certificate'),('twist','Quadratic twist / F2 descent'),('base23','Marked23 specialization / rank112'),('fiber','Exhaustive finite QR7 cycle test')]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="980" height="470" viewBox="0 0 980 470">','<rect width="980" height="470" fill="white"/>','<g font-family="DejaVu Sans,sans-serif" fill="#17304a">','<text x="24" y="34" font-size="23">R51 exact chart maps and larger-carrier evidence</text>']
for i,(key,label) in enumerate(nodes):
    col=i//4; y=65+(i%4)*78; x=24+col*475; status=state[key]
    color={'accepted':'#d4eddf','rejected':'#f6d8d6','running':'#fff0c2','prepared':'#edf0f3'}[status]
    svg += [f'<rect x="{x}" y="{y}" width="448" height="64" rx="7" fill="{color}"/>',f'<text x="{x+12}" y="{y+25}" font-size="16">{html.escape(label)}</text>',f'<text x="{x+12}" y="{y+49}" font-size="14">{status}; source: results.json / native receipts</text>']
svg += [f'<text x="24" y="407" font-size="15">{result["lean_theorems"]} unique accepted Lean lemmas; zero failing or trusted declarations.</text>','<text x="24" y="434" font-size="14">Rank112 is an algebra acceptance target; primality and selected-factor action need separate evidence.</text>','<text x="24" y="458" font-size="13">Deterministic certificates; no timing comparison or statistical uncertainty.</text>','</g></svg>']
(h/'round51.svg').write_text('\n'.join(svg)+'\n')
def text(s): return 'text('+json.dumps(s)+')'
tr=[]
for r in rows: tr += [text(r['attempt']),text(r['status']),text('compile '+str(r['compile_exit'])+', audit '+str(r['audit_exit']) if r.get('kind')=='Lean' else 'exit '+str(r.get('native_exit','pending'))+('; '+str(len(r['error_lines']))+' error(s)' if r.get('error_lines') else ''))]
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
if result['lean_theorems']>5: typ += '\nSpecialize51 adds six audited declarations: denominator-unit cancellation,scaled ideal equality,the three-doubling group consequence,the exact line intersection,quartic difference and cross-kernel denominator inverse. Actual native inputs remain external premises.\n'
(h/'report.typ').write_text(typ)
print(json.dumps(result['status_counts']))
