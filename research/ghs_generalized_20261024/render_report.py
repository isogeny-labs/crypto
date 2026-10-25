"""Lay out immutable receipts into the research report; no attack arithmetic."""
from pathlib import Path
import hashlib
import json
import platform

root=Path(__file__).resolve().parent
results=root/'results'
specs=[(6,0,32,['direct','two_isogeny']),
       (8,0,32,['direct','two_isogeny']),
       (10,0,32,['direct','two_isogeny','three_isogeny']),
       (12,1,512,['direct','two_isogeny','five_isogeny'])]
lines=['# Generalized binary GHS: explicit map and attack controls',
       '', '24 October 2026. Source base `a53a7e2bfa20080a424b9a2484a16932b6e3c116`. '
       'This report cites raw JSON under `results/`, the frozen `PROTOCOL.md`, and `PRIOR_WORK.md`.',
       '', '## Scope and decision',
       '', 'All exact instances below are members of prior-known binary GHS/isogeny families '
       '([Galbraith–Hess–Smart 2002](https://www.iacr.org/archive/eurocrypt2002/23320028/smart.pdf); '
       '[Hess 2004](https://www.cambridge.org/core/journals/lms-journal-of-computation-and-mathematics/article/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem/315278965D8D277A85812D9498236A88)). '
       'An exact hexadecimal instance has not been matched to a publication, but that does not establish novel vulnerability. '
       'The measurements below do not support a calibrated GHS advantage over rho. '
       'The end-to-end executable work is the quadratic genus-two branch for the listed exact curves; '
       'other screen profiles have no complete attack map.',
       '', '## Search and mathematical map',
       '', 'For each proper `GF(2^s)` in `GF(2^n)` and each nonzero Hess parameter `c` in the finite field, '
       'the screen records a Frobenius-order profile, MMT gate, and genus. '
       'Degrees 6/8/10/12 use respectively 63/255/1023/4095 values: the full nonzero `c` range. '
       'The quadratic model is constructed over `GF(2^(n/2))` when `D = beta + beta^(2^(n/2)) != 0`, '
       'where `beta^2=b`. Given trace-one `lambda`, define `A=a+a^q`, '
       '`B=beta+lambda^2 D`, `mu=sqrt(B/D)`, `L=B/D+mu+lambda^2+lambda`, '
       '`H=u^2+u+A`, `C0=a+lambda^2 A`; then '
       '`C: W^2+H W = D H+(L u+(B/D)A+C0) H^2` (genus two). '
       'The curve-to-elliptic morphism is `X=beta H/D`, '
       '`Y=beta W/D+(mu+lambda)X u+beta`. '
       'The executed elliptic-to-Jacobian map is `trace(conorm(P-O))`; '
       'a source point maps to an explicitly reduced Mumford divisor. '
       'For a rational kernel `K` of order 2, 3, or 5, Vélu-style coordinates are '
       '`X=x(P)+sum_(T in K\\{O})(x(P+T)+x(T))`, '
       '`Y=y(P)+sum_(T in K\\{O})(y(P+T)+y(T))+shift`, '
       'with the destination coefficients fitted and independently checked on every rational source point. '
       'These formulas are source-linked to [Magma descent documentation](https://magma.maths.usyd.edu.au/magma/handbook/text/1561) '
       'and [the original isogeny extension](https://www.iacr.org/archive/eurocrypt2002/23320028/smart.pdf). '
       'The executable checks establish the properties for the enumerated finite groups and chosen subgroups; '
       'the Lean module proves the abstract subgroup implication, not the concrete coordinate identity.',
       '', '![Genus distributions from the four complete nonzero-c screens](genus-profiles.png)',
       '', 'Screen population and exact counts: '
       '`results/screen-n6-a0-b20.json`, `screen-n8-a0-b20.json`, '
       '`screen-n10-a0-b20.json`, `screen-n12-a1-b200.json`. '
       'Marker area grows with the number of `c` values yielding a genus at that subfield degree, capped at 400 for legibility; read the exact histogram counts in JSON. '
       'The editable renderer is `render_screen.py`; the vector copy is `genus-profiles.svg`.',
       '', 'For degree 12, `results/leads-n12-a1-b200-g2.json` contains 147 screened genus-at-most-two profiles, including 15 genus-one profiles over GF(2^4); the first is `c=0x1`. The independently replayed source subgroup has prime order 199. The integer Weil bound gives `#Jac(C)(GF(2^4)) <= (1+4)^2 = 25` for those 15 genus-one profiles, and `#Jac(C)(GF(2^2)) <= (1+2)^4 = 81` for the six genus-two profiles over GF(2^2). Every homomorphism from the order-199 subgroup to either target group is therefore zero, regardless of the as-yet-unconstructed coordinate map. `audit_subgroup_bounds.py` records the exact excluded `c` values and the same check across all four screens in `results/subgroup-weil-audit.json`: 21 low-genus profiles excluded, 232 retained by this necessary bound. The retained profiles are structural leads only; the bound does not establish a nonzero map or an attack.',
       '', '## Exact instances, subgroup maps, and measured work',
       '', 'All curves use `y²+xy=x³+a x²+b` in the indicated polynomial-basis field. '
       '`P`, `Q`, `phi(P)`, and `phi(Q)` are exact JSON coordinates; '
       'the GHS image is a pair of exact Mumford coefficient arrays. '
       'Each successful row enumerates all `r` scalars to check the full composite subgroup map, as well as `[r]phi(P)=O`, `phi([37]P)=[37]phi(P)`, '
       'nonzero Jacobian image, `[r]G=O`, source scalar replay, '
       'and a canonical-negation-folded rho on the same `(P,Q)`. Isogeny construction validates every rational source point '
       'against the codomain equation and 400 group pairs; the degree-six test checks every pair.',
       '', '| n | edge | #E / r | FB / rank | smooth / attempts | rho EC add / dbl / steps | result |',
       '|--:|:--|--:|--:|--:|--:|:--|']

def coords(v):
    return '('+','.join(str(x) for x in v[:2])+')' if v else 'missing'

def operations(cost):
    return ', '.join(f'{k}={v}' for k,v in sorted(cost['field_operations'].items()))+f'; wall_ns={cost["wall_ns"]} (L0)'

rows=[]
cost_rows=[]
for n,a,b,edges in specs:
    modulus={6:'0x43',8:'0x11b',10:'0x409',12:'0x1009'}[n]
    for edge in edges:
        path=results/f'exact-n{n}-a{a}-b{b}-{edge}.json'
        if not path.exists():
            lines.append(f'| {n} | {edge.replace("_isogeny","-isog")} | — | — | — | — | not executed |')
            continue
        j=json.loads(path.read_text()); src=j['source']; g=j.get('ghs'); iso=j.get('isogeny'); rho=j['rho']['result']
        stat='verified' if j['ghs_status']=='verified' else f"failed: {j['ghs_error']}"
        fb='—' if g is None else f"{g['factor_base_size']} / {g['matrix_rank']}"
        relations='—' if g is None else f"{g['relation_counts']['smooth']} / {g['relation_counts']['attempts']}"
        lines.append(f"| {n} | {edge.replace('_isogeny','-isog')} | {src['group_order']} / {src['subgroup_order']} | {fb} | {relations} | {rho['elliptic_additions']} / {rho['elliptic_doublings']} / {rho['steps']} | {stat} |")
        rows.append((path,j))
        if g:
            ns=lambda value: f'{value / 1_000_000:.3f}'
            setup=sum(cost['wall_ns'] for cost in j['setup_phases'].values() if isinstance(cost,dict) and 'wall_ns' in cost)
            if iso:
                setup+=iso['construction_and_validation_cost']['wall_ns']
            pre=sum(g['phases'][phase]['wall_ns'] for phase in ('factor_base','relations_and_linear_algebra'))
            online=sum(g['phases'][phase]['wall_ns'] for phase in ('map','target_descent_and_replay'))
            cost_rows.append(f"| {n} | {edge.replace('_isogeny','-isog')} | {ns(setup)} | {ns(pre)} | {ns(online)} | {ns(j['rho']['cost']['wall_ns'])} |")
lines+=['', '| n | edge | setup incl. audit | GHS precompute | GHS online | rho online |',
        '|--:|:--|--:|--:|--:|--:|']+cost_rows+['',
        'One-sample elapsed cost summary in milliseconds. Setup includes construction, source-order enumeration, exhaustive subgroup-map verification, and isogeny construction/validation where applicable. GHS precompute is factor-base construction plus relation collection and matrix work; online is the map plus target descent and replay. The rho column is its complete same-target run. These are unisolated L0 samples and do not establish a speedup.']
lines+=['', 'The matched reference is deterministic `rho.negation` with a memory collision table and up to 12 partition reseeds; its counted additions/doublings and field operations include restarts. `results/MANIFEST.json` binds the executable SHA-256, source base, host and each receipt hash. Operation counts are phase-specific and preserve the field degree. '
        'Inverse-call counters are informational and include multiplications and squarings already tallied, so do not add inverses to those counts. The genus-two field operations, source-field rho operations, matrix entry operations, '
        'and hash/table work use different units. `wall_ns` is one unisolated L0 observation, '
        'so no elapsed-time ratio or speedup is inferred; this rho has not been measured against the fastest external implementation. A calibrated GAE 0/0.1/1 band '
        'and an external implementation comparison remain outstanding.',
        '', '### Raw cost vectors (one sample per arm)', '']
for path,j in rows:
    src=j['source']; lines+= [f"**{path.name}**, sha256 `{hashlib.sha256(path.read_bytes()).hexdigest()}`. "
             f"E over GF(2^{src['absolute_degree']}), modulus {src['modulus']}, a={src['a']}, b={src['b']}; source #E={src['group_order']}, subgroup r={src['subgroup_order']}, P={coords(src['generator'])}, Q={coords(src['target'])}; rho scalar {j['rho']['result']['scalar']}.",
             '']
    for label,cost in j['setup_phases'].items():
        lines.append(f'- Setup {label}: {operations(cost)}.' if isinstance(cost,dict) else f'- Setup {label}: {cost}.')
    iso=j.get('isogeny')
    if iso:lines.append(f"- Degree-{iso['degree']} kernel {iso['kernel_points']}; codomain a={iso['codomain_a']}, b={iso['codomain_b']}; kernel and codomain construction: {operations(iso['construction_and_validation_cost'])}.")
    if j.get('ghs'):
        g=j['ghs']
        for label,cost in g['phases'].items():lines.append(f'- GHS {label}: {operations(cost)}.')
        lines.append(f"- Unpriced GHS counters: {json.dumps(g['unpriced_operations'],sort_keys=True)}.")
        lines.append(f"- Isogeny image P/Q: {iso['image_generator'] if iso else src['generator']} / {iso['image_target'] if iso else src['target']}; Jacobian image P/Q: {json.dumps(g['image_generator'])} / {json.dumps(g['image_target'])}.")
        lines.append(f"- Descended curve `h={json.dumps(g['map']['h'])}`, `f={json.dumps(g['map']['f'])}`, base basis `{json.dumps(g['map']['subfield_embedding_basis'])}`.")
    else:lines.append(f"- GHS failure: {j['ghs_error']}.")
    lines.append(f"- Rho: {operations(j['rho']['cost'])}; {json.dumps(j['rho']['result'],sort_keys=True)}.")
    lines.append('')
lines+=['## Formal scope, prior work and limits','',
        'The Lean file `lean/SubgroupTransfer.lean` proves additive-map scalar transfer, '
        'nonzero image plus prime subgroup injectivity, vanishing when destination r-torsion is trivial, and push-pull injectivity under '
        'a coprimality hypothesis. `lean --run ../../../tools/lean/AuditAxioms.lean SubgroupTransfer` '
        'audited ten declarations with zero failing and zero `native_decide` trusted; '
        '`propext` appears in generated constructor and injectivity declarations. '
        'The finite-field Weil bound and its concrete arithmetic are checked by the audit script, not proved in Lean. '
        'Lean does not prove that the coded trace-conorm and Vélu formulas satisfy those abstract hypotheses. '
        'That geometric proof and a characteristic-two identity certificate are open.',
        '', 'The 2002 isogeny extension and the 2004 generalization establish prior family coverage; '
        'see `PRIOR_WORK.md` for the per-instance searches. None of these rows is classified '
        'as potentially new vulnerable. The `c` screen, direct degree-two cover, and small rational-kernel '
        'isogenies are distinct capabilities: most screened profiles have no executable transfer '
        'and no index-calculus cost. Fields beyond degree 12, arbitrary isogeny chains, degree-7 '
        'end-to-end examples, higher-genus index calculus and odd characteristic are not covered. '
        'The provided target scalar is used to prepare and independently validate the instance; '
        'the attack and rho routines only receive source P and Q.',
        '', 'Reproduce with `cargo build --release --bins`, '
        '`bash research/ghs_generalized_20261024/run_exact.sh`, '
        '`cargo test --release --bin ghs_quadratic_attack`, '
        '`cargo test --release --lib ghs_generalized`, '
        '`cargo test --release --lib ghs_quadratic`, and '
        '`LEAN_PATH=. lean --run ../../../tools/lean/AuditAxioms.lean SubgroupTransfer` '
        'from the Lean directory. Host: '+platform.platform()+'.', '']
(root/'REPORT.md').write_text('\n'.join(lines))
print(f'wrote {len(rows)} completed rows')
