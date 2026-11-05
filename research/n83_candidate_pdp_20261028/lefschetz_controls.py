"""Exact small-field correctness controls; no n83 production entry point."""
from pathlib import Path
from itertools import combinations, combinations_with_replacement, product
from collections import Counter
from math import comb, prod
import hashlib, json, shutil, sys

def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()

def put(row, key, value, p):
    value = (row.get(key, 0) + value) % p
    if value:
        row[key] = value
    else:
        row.pop(key, None)

def sparse_rank(rows, p):
    basis = {}
    for raw in rows:
        row = dict(raw)
        while row:
            lead = min(row); c = row[lead]
            if lead in basis:
                for j, v in basis[lead].items(): put(row, j, -c * v, p)
            else:
                inv = pow(c, -1, p)
                basis[lead] = {j: v * inv % p for j, v in row.items()}
                break
    return len(basis)

def nullspace(matrix, n, p):
    """Independent dense RREF path for primitive vectors."""
    a = [list(row) for row in matrix]; pivots = []; r = 0
    for j in range(n):
        k = next((i for i in range(r, len(a)) if a[i][j]), None)
        if k is None: continue
        a[r], a[k] = a[k], a[r]
        inv = pow(a[r][j], -1, p); a[r] = [v * inv % p for v in a[r]]
        for i in range(len(a)):
            if i != r and a[i][j]:
                c = a[i][j]; a[i] = [(v-c*w) % p for v, w in zip(a[i], a[r])]
        pivots.append(j); r += 1
    result = []
    for j in range(n):
        if j in pivots: continue
        v = [0] * n; v[j] = 1
        for i, k in enumerate(pivots): v[k] = -a[i][j] % p
        result.append(v)
    return result

def masks(ell, degree):
    return [sum(1 << i for i in c) for c in combinations(range(ell), degree)]

def up(v, ell, p):
    out = {}
    for mask, c in v.items():
        for i in range(ell):
            if not (mask >> i & 1): put(out, mask | (1 << i), c, p)
    return out

def block_strings(ell, p):
    by_degree = [[] for _ in range(ell+1)]; strings = []; multiplicities = []
    endpoint_ok = True
    for k in range(ell//2+1):
        cols = masks(ell, k); lower = masks(ell, k-1) if k else []
        matrix = [[int((low & col) == low) for col in cols] for low in lower]
        primitive = nullspace(matrix, len(cols), p)
        expected = comb(ell, k)-(comb(ell, k-1) if k else 0)
        multiplicities.append({'k': k, 'actual': len(primitive), 'expected': expected})
        for v in primitive:
            vector = {mask: c for mask, c in zip(cols, v) if c}; chain = []
            for t in range(ell-2*k+1):
                chain.append([[mask, c] for mask, c in sorted(vector.items())])
                by_degree[k+t].append(vector); vector = up(vector, ell, p)
            endpoint_ok &= not vector
            strings.append({'primitive_degree': k, 'vectors': chain,
                            'endpoint': [[mask, c] for mask, c in sorted(vector.items())]})
    ranks = [{'degree': h, 'rank': sparse_rank(vectors, p), 'dimension': comb(ell, h),
              'vectors': len(vectors)} for h, vectors in enumerate(by_degree)]
    valid = (endpoint_ok and all(x['actual']==x['expected'] for x in multiplicities)
             and all(x['rank']==x['dimension']==x['vectors'] for x in ranks))
    return {'ell': ell, 'p': p, 'sufficient_characteristic': p>ell,
            'basis_valid': valid, 'endpoint_ok': endpoint_ok, 'multiplicities': multiplicities,
            'degree_ranks': ranks, 'strings': strings}

def monomials(n, degree, caps):
    return [q for q in combinations_with_replacement(range(n), degree)
            if all(c <= caps[i] for i, c in Counter(q).items() if i in caps)]

def quartics(groups, us, p, caps):
    """Direct six-term expansion; independent of the native factored builder."""
    m = len(groups); fs = []
    for k in range(m-1):
        x = groups[0] if k == 0 else [us[k-1]]; y = groups[k+1]
        if k == m-2:
            terms = [(x, x, y, y, 1)]
        else:
            z = [us[k]]
            terms = [(x,x,y,y,1),(x,x,z,z,1),(y,y,z,z,1),
                     (x,x,y,z,-2),(x,y,y,z,-2),(x,y,z,z,-2)]
        f = {}
        for a,b,c,d,coefficient in terms:
            for indices in product(a,b,c,d):
                mon = tuple(sorted(indices)); counts = Counter(mon)
                if any(counts[i] > cap for i, cap in caps.items()): continue
                put(f, mon, coefficient, p)
        fs.append(f)
    return fs

def multiplication_map(fs, n, q, caps, p):
    domain = monomials(n, q, caps); rows = {}
    for j, f in enumerate(fs):
        for i, h in enumerate(domain):
            column = j * len(domain) + i
            for mon, c in f.items():
                dest = tuple(sorted(h+mon)); counts = Counter(dest)
                if any(counts[t] > cap for t, cap in caps.items()): continue
                put(rows.setdefault(dest, {}), column, c, p)
    rows = {mon: row for mon, row in rows.items() if row}
    return {'columns': len(fs)*len(domain), 'multiplier_domain': domain,
            'rank': sparse_rank((rows[mon] for mon in sorted(rows)), p),
            'matrix_rows': [{'monomial': mon, 'entries': [[j,str(c)] for j,c in sorted(row.items())]}
                            for mon,row in sorted(rows.items())]}

def direct_map(m, ell, q, p):
    nb = m*ell; n = nb+m-2; caps = {i:1 for i in range(nb)}
    groups = [list(range(i*ell,(i+1)*ell)) for i in range(m)]
    return multiplication_map(quartics(groups,list(range(nb,n)),p,caps),n,q,caps,p)

def sector_map(m, ell, q, p):
    n = 2*m-2; fs = quartics([[i] for i in range(m)], list(range(m,n)),p,{})
    total_columns = total_rank = 0; sectors = []
    for ks in product(range(min(ell//2,q)+1), repeat=m):
        shift = sum(ks)
        if shift > q: continue
        multiplicity = prod(comb(ell,k)-(comb(ell,k-1) if k else 0) for k in ks)
        caps = {i:ell-2*k for i,k in enumerate(ks)}
        record = multiplication_map(fs,n,q-shift,caps,p)
        total_columns += multiplicity*record['columns']; total_rank += multiplicity*record['rank']
        sectors.append({'primitive_degrees':ks,'multiplicity':multiplicity,'caps':caps,**record})
    return {'columns':total_columns,'rank':total_rank,'sectors':sectors}

def leading_gate(m, weights):
    fs = quartics([[i] for i in range(m)],list(range(m,2*m-2)),101,{})
    records = []; supports = []
    for k,f in enumerate(fs):
        expected = tuple(sorted(([0,0] if k==0 else [m+k-1,m+k-1])+[k+1,k+1]))
        scores = {mon:sum(weights[i] for i in mon) for mon in f}
        maximal = [mon for mon,v in scores.items() if v==max(scores.values())]
        records.append({'generator':k,'expected':expected,'maximal':maximal,
                        'terms':[[mon,str(f[mon]),scores[mon]] for mon in sorted(f)],
                        'strict_expected':maximal==[expected]})
        supports.append(set(expected))
    disjoint = all(not (supports[i]&supports[j]) for i in range(len(fs)) for j in range(i))
    return {'m':m,'weights':weights,'positive':all(w>0 for w in weights),
            'pairwise_disjoint':disjoint,'records':records,
            'valid':all(w>0 for w in weights) and disjoint and all(x['strict_expected'] for x in records)}

def run(root, replay):
    def emit(name, value):
        value = json.loads(json.dumps(value))
        p = root/name
        if replay: assert json.loads(p.read_text())==value, name
        else:
            p.parent.mkdir(parents=True,exist_ok=True)
            with p.open('x') as f: json.dump(value,f,indent=2); f.write('\n')
        return value
    index = json.loads((root/'reference-index.json').read_text())
    for x in index['dense_controls']:
        p=root/'references'/Path(x['artifact_relative_to_support']).name
        assert sha(p)==x['sha256']
    emit('identity.json',{'schema':'n83.lefschetz-development/v1','development_only':True,
                          'source_sha256':sha(__file__),'reference_index_sha256':sha(root/'reference-index.json')})
    for ell in range(1,10):
        for p in [11,101]:
            d=emit(f'blocks/ell{ell}-p{p}.json',block_strings(ell,p));assert d['basis_valid']
        print(json.dumps({'block_ell_complete':ell,'replay':replay}),flush=True)
    for ell,p in [(3,2),(5,3)]:
        d=emit(f'negative/ell{ell}-p{p}.json',block_strings(ell,p))
        assert not d['sufficient_characteristic'] and not d['basis_valid']
    summary=[]
    for m,ells in [(3,[1,2,3]),(4,[1,2])]:
        for ell in ells:
            for q in range(4):
                for p in [11,101]:
                    direct=direct_map(m,ell,q,p); sectors=sector_map(m,ell,q,p)
                    key=f'm{m}-ell{ell}-q{q}-p{p}'
                    emit(f'maps/{key}.json',{'m':m,'ell':ell,'q':q,'p':p,'direct':direct,'sector':sectors})
                    assert direct['columns']==sectors['columns'] and direct['rank']==sectors['rank'],key
                    if m==3 and q==3:
                        native=json.loads((root/'references'/f'dense-m3-ell{ell}-p{p}.json').read_text())
                        assert native['columns']==direct['columns'] and native['independent_dense_rank']==direct['rank']
                        assert native['matrix_rows']==json.loads(json.dumps(direct['matrix_rows']))
                    summary.append({'key':key,'columns':direct['columns'],'rank':direct['rank'],
                                    'kernel_dimension':direct['columns']-direct['rank'],
                                    'sector_count':len(sectors['sectors']),'matches_native_reference':m==3 and q==3})
                print(json.dumps({'map_complete':key,'replay':replay}),flush=True)
    for m in range(2,10):
        weights=[m]*m+list(range(m-2,0,-1));d=emit(f'leading/m{m}.json',leading_gate(m,weights));assert d['valid']
    for m in range(3,10):
        d=emit(f'negative/leading-m{m}.json',leading_gate(m,[m]*(2*m-2)));assert not d['valid']
    degrees=[{'ell':27,'degree':d,'universal_slack':27-2*d+6,'gate':27-2*d+6>=0} for d in range(4,18)]
    boundary_fs=quartics([[0],[1],[2]],[3],101,{})
    boundary_map=multiplication_map(boundary_fs,4,0,{0:27,1:27,2:1},101)
    terminal_disappears=all(j!=1 for row in boundary_map['matrix_rows'] for j,c in row['entries'])
    emit('truncation.json',{'degrees':degrees,'boundary':{'degree':17,'primitive_last':13,'string_max_exponent':1,
         'terminal_required_exponent':2,'terminal_term_disappears':terminal_disappears,'matrix':boundary_map},'new_production_bound':None})
    assert terminal_disappears and boundary_map['rank']==1
    assert all(x['gate'] for x in degrees[:-1]) and not degrees[-1]['gate']
    emit('summary.json',{'development_only':True,'positive_blocks':18,'negative_blocks':2,
         'matched_maps':summary,'native_reference_matches':6,'leading_gates':8,'leading_negative_controls':7,
         'new_n83_bound':None,'current_production_interval':[7,17],
         'limits':'Finite controls do not establish the universal Koszul-transport proof or source fidelity.'})
    print(json.dumps({'complete':True,'matched_maps':len(summary),'replay':replay}),flush=True)

if __name__=='__main__':
    mode=sys.argv[1];root=Path(sys.argv[2]);assert mode in ['run','replay']
    if mode=='run':
        assert len(sys.argv)==5 and not root.exists();root.mkdir()
        refs=Path(sys.argv[3]);index=Path(sys.argv[4]);shutil.copy2(index,root/'reference-index.json')
        (root/'references').mkdir()
        for row in json.loads(index.read_text())['dense_controls']:
            name=Path(row['artifact_relative_to_support']).name;shutil.copy2(refs/name,root/'references'/name)
    else: assert len(sys.argv)==3 and root.is_dir()
    run(root,mode=='replay')
