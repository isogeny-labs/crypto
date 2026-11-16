"""Preparation-only Gold projection diagnostic; no root search or speed claim."""
from pathlib import Path
import gzip, hashlib, itertools, json, resource, sys, time
from rootfinder.binaryfield import BinaryField

P = Path(__file__).resolve().parent
ROOT = P.parents[1]
START = time.monotonic()
CHECKS = 0


def require(value, why):
    global CHECKS
    CHECKS += 1
    assert CHECKS <= 200000 and time.monotonic() - START < 80, "work cap"
    assert value, why


def multiply(a, b, degree, modulus):
    """Independent coefficient convolution then descending reduction."""
    out = 0
    for i in range(degree):
        if b >> i & 1:
            out ^= a << i
    while out.bit_length() > degree:
        out ^= modulus << (out.bit_length() - degree - 1)
    return out


def xor(values):
    out = 0
    for value in values:
        out ^= value
    return out


def pivots(values):
    table = {}
    for value in values:
        while value:
            bit = value.bit_length() - 1
            if bit not in table:
                table[bit] = value
                break
            value ^= table[bit]
    return table


def residue(value, table):
    for bit in sorted(table, reverse=True):
        if value >> bit & 1:
            value ^= table[bit]
    return value


def masks(d):
    return [1, 1 << (3*d-3),
            sum(1 << j for j in range(3, 3*d-3, 3)),
            sum(1 << j for j in range(1, 3*d-3) if j % 3)]


def projection(value, tests):
    return sum(((value & mask).bit_count() & 1) << j for j, mask in enumerate(tests))


def column_dp(columns, rhs):
    """At most16 output values; dependent columns give a triangular kernel."""
    reachable = {0: 0}
    kernel = []
    for i, col in enumerate(columns):
        if col in reachable:
            kernel.append((1 << i) ^ reachable[col])
        else:
            reachable.update({v ^ col: mask ^ (1 << i)
                              for v, mask in list(reachable.items())})
    rank = (len(reachable)).bit_length() - 1
    return dict(consistent=rhs in reachable, rank=rank,
                particular=reachable.get(rhs), basis=kernel,
                image=sorted(reachable))


def row_oracle(rows, variables):
    """Separate row Gaussian solver with explicit contradiction combinations."""
    table = {}
    contradiction = None
    for i, (mask, rhs) in enumerate(rows):
        witness = 1 << i
        while mask:
            bit = mask.bit_length()-1
            if bit not in table:
                table[bit] = mask, rhs, witness
                break
            m, r, w = table[bit]
            mask ^= m
            rhs ^= r
            witness ^= w
        if not mask and rhs and contradiction is None:
            contradiction = witness
    free = [i for i in range(variables) if i not in table]
    def back(value, homogeneous):
        for bit, (mask, rhs, _) in sorted(table.items()):
            if ((mask & value).bit_count() & 1) ^ (0 if homogeneous else rhs):
                value ^= 1 << bit
        return value
    return dict(consistent=contradiction is None, rank=len(table),
                particular=None if contradiction else back(0, False),
                basis=[back(1 << i, True) for i in free],
                contradiction=contradiction,
                pivots=[[bit, *data] for bit, data in sorted(table.items())])


def controls():
    formal = []
    for d in range(4, 29):
        generators = [(1 << (2*i+j)) ^ (1 << (i+2*j))
                      for i in range(d) for j in range(i+1, d)]
        rank = len(pivots(generators))
        require(rank == 3*d-6, "formal Gold rank")
        require(all(not projection(v, masks(d)) for v in generators), "four masks annihilate")
        require(len(pivots(masks(d))) == 4, "independent complement masks")
        formal.append([d, rank])
    total = 0
    for n, mod in [(2, 7), (3, 11), (4, 19)]:
        field = BinaryField(n, mod)
        mul = lambda a, b: multiply(a, b, n, mod)
        for a, b in itertools.product(range(1 << n), repeat=2):
            require(field.mul(a, b) == mul(a, b), "independent field arithmetic")
        gold = pivots([field.mul(field.square(1 << i), 1 << j) ^
                       field.mul(1 << i, field.square(1 << j))
                       for i in range(n) for j in range(i+1, n)])
        for bag in itertools.combinations_with_replacement(range(1 << n), 5):
            H = [1]
            for x in bag:
                out = [0] * (len(H)+1)
                for i, c in enumerate(H):
                    out[i] ^= field.mul(c, x)
                    out[i+1] ^= c
                H = out
            value = mul(H[4], H[3]) ^ H[2]
            independent = xor(mul(mul(a, a), b) ^ mul(a, mul(b, b))
                              for a, b in itertools.combinations(bag, 2))
            require(value == independent, "literal elementary/Newton identity")
            require(not residue(value, gold), "all bags in reduced-generator Gold space")
            total += 1
    require(total == 16352, "complete tiny multiset corpus")
    # Exhaustive independent truth oracle for all four-output, three-input maps.
    maps = 0
    for columns in itertools.product(range(4), repeat=3):
        for rhs in range(4):
            a = column_dp(columns, rhs)
            rows = [(sum(((v >> j) & 1) << i for i, v in enumerate(columns)),
                     (rhs >> j) & 1) for j in range(2)]
            b = row_oracle(rows, 3)
            wanted = {x for x in range(8)
                      if xor(v for i, v in enumerate(columns) if x >> i & 1) == rhs}
            got = set() if not a['consistent'] else {
                a['particular'] ^ xor(v for i, v in enumerate(a['basis']) if t >> i & 1)
                for t in range(1 << len(a['basis']))}
            require(got == wanted and a['consistent'] == b['consistent'] and
                    a['rank'] == b['rank'], "complete tiny projection oracle")
            maps += 1
    return dict(formal_graph_ranks=formal, tiny_source_multisets=total,
                complete_small_linear_systems=maps,
                wrapped_fields_use_reduced_generators=True)


def actual():
    data = json.loads((ROOT/'research/advance/instances/n83-d18-m5-T001-offset0.json').read_text())
    spec = data['extension_field']; n = spec['degree']; mod = int(spec['modulus_hex'], 16)
    require(n == 83 and mod == 0x800000000200000000007, "actual field")
    f = lambda a, b: multiply(a, b, n, mod)
    A = int(data['curve']['a_hex'], 16); B = int(data['curve']['b_hex'], 16)
    z = int(data['target']['x_hex'], 16); y = int(data['target']['y_hex'], 16) ^ z
    h = 0x4208336d0d441df79c400
    require([int(v, 16) for v in data['source_basis_hex']] == [1 << i for i in range(18)] and
            int(data.get('source_offset_hex', '0'), 16) == 0, "actual domain")
    require(f(y,y)^f(z,y) == f(f(z,z),z)^f(A,f(z,z))^B, "target curve")
    raw_path = ROOT/'research/full_quintic_block_20261115_v9/raw.json.gz'
    raw = raw_path.read_bytes()
    require(hashlib.sha256(raw).hexdigest() ==
            'e79604203f3ca5956d55b71d3dcb16d269b8514cd24e32cba8d75fcd914de7bb', "frozen baseline evidence")
    baseline = json.loads(gzip.decompress(raw))['out']['profile']['proofs']['baseline166']
    def pm(a, b):
        out = [0] * (len(a)+len(b)-1)
        for i, x in enumerate(a):
            for j, v in enumerate(b): out[i+j] ^= f(x, v)
        return out
    def norm(value):
        r = value & ((1 << n)-1); s = value >> n
        t = f(f(z,z),z)^f(r,f(z,z))^f(s,z)^f(f(h,z),y)
        F = [t,s,r,1]; G = [0,h]
        parts = [pm(F,F),pm([0,1],pm(G,F)),pm([B,0,A,1],pm(G,G))]
        N = [xor(v[i] for v in parts if i < len(v)) for i in range(7)]
        H = [0]*6; H[5] = N[6]
        for i in range(5,0,-1): H[i-1] = N[i]^f(z,H[i])
        require(N[0]^f(z,H[0]) == 0 and H[5] == 1, "literal target division")
        return H
    def pack(H):
        return sum(((H[5-j] >> d) << d) << ((j-1)*n)
                   for j,d in enumerate([18,35,52,69,83], 1))
    Hzero = norm(0)
    fullcols = [[a^b for a,b in zip(norm(1 << bit), Hzero)] for bit in range(166)]
    rhs = pack(Hzero); packed = [pack(v) for v in fullcols]
    rows = [[sum(((v >> j)&1) << i for i,v in enumerate(packed)), (rhs >> j)&1]
            for j in range(415)]
    require(rows == baseline['rows'] and len(pivots(m for m,r in rows)) == 93, "complete original norm map/rank")
    particular = baseline['particular']; basis = baseline['basis']
    require(len(basis) == len(pivots(basis)) == 73, "complete independent old kernel")
    require(all((m & particular).bit_count()%2 == r for m,r in rows) and
            all(all((m & v).bit_count()%2 == 0 for m,r in rows) for v in basis), "old coset certificates")
    H0 = norm(particular); b = H0[4]
    require(b < (1 << 18) and b == f(h,h)^h^z, "admitted fixed centroid")
    homogeneous = [[a^v for a,v in zip(norm(x), Hzero)] for x in basis]
    g0 = f(b,H0[3]) ^ H0[2]
    gs = [f(b,H[3]) ^ H[2] for H in homogeneous]
    columns = [projection(g, masks(18)) for g in gs]; target = projection(g0, masks(18))
    added_rows = [[sum(((v >> j)&1) << i for i,v in enumerate(columns)), (target >> j)&1]
                  for j in range(4)]
    candidate = column_dp(columns, target); oracle = row_oracle(added_rows, 73)
    require(candidate['rank'] == oracle['rank'] and candidate['consistent'] == oracle['consistent'], "actual independent solvers")
    require(len(candidate['basis']) == len(pivots(candidate['basis'])) == 73-candidate['rank'], "complete new kernel")
    require(all(xor(v for i,v in enumerate(columns) if mask >> i & 1) == 0
                for mask in candidate['basis']), "new kernel guards")
    if candidate['consistent']:
        require(xor(v for i,v in enumerate(columns) if candidate['particular'] >> i & 1) == target, "new particular guards")
        require(not residue(candidate['particular'] ^ oracle['particular'], pivots(candidate['basis'])) and
                all(not residue(v, pivots(candidate['basis'])) for v in oracle['basis']), "same complete new coset")
    else:
        witness = oracle['contradiction']
        require(xor(m for i,(m,r) in enumerate(added_rows) if witness >> i & 1) == 0 and
                xor(r for i,(m,r) in enumerate(added_rows) if witness >> i & 1) == 1, "whole-cell contradiction")
    return dict(old_variables=73, fixed_h=h, fixed_k=0, centroid=b, baseline_evidence_sha256=hashlib.sha256(raw).hexdigest(),
                gold_columns=columns, gold_target=target, gold_rows=added_rows,
                candidate=candidate, independent_row_certificate=oracle,
                new_parameter_bits=73-candidate['rank'] if candidate['consistent'] else None,
                exact_old_parameter_fraction=('1/'+str(1 << candidate['rank'])) if candidate['consistent'] else '0',
                original_root_count_exact=None, root_search_performed=False,
                baseline_literal_columns_reconstructed=166)


def main():
    result = {}; status = 'passed'; error = None
    try:
        result['controls'] = controls()
        result['actual'] = actual()
    except Exception as exc:
        status = 'inconclusive'; error = type(exc).__name__+': '+str(exc)
    payload = json.dumps(dict(result=result,error=error,checks=CHECKS), sort_keys=True, separators=(',',':')).encode()
    blob = gzip.compress(payload, mtime=0)
    if len(payload)>128*1024 or len(blob)>32*1024:
        status='inconclusive'; error='raw cap'; result={}
        payload=json.dumps(dict(error=error,complete=False)).encode();blob=gzip.compress(payload,mtime=0)
    with (P/'raw.json.gz').open('xb') as stream: stream.write(blob)
    actual_result=result.get('actual',{}); candidate=actual_result.get('candidate',{})
    evidence=dict(status_scope='sole-model preparation diagnostic; independent algorithms, no external reviewer admission',
                  controls=result.get('controls'), actual_summary={k:actual_result.get(k) for k in
                  ['old_variables','new_parameter_bits','exact_old_parameter_fraction','root_search_performed']},
                  gold_rank_increment=candidate.get('rank'), consistent=candidate.get('consistent'),
                  error=error,raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),
                  raw_payload_bytes=len(payload),peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
                  complete_projection_diagnostic=status=='passed',complete_global_root_search=False,
                  original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False,novelty_claim=False)
    out=dict(schema='scientific_result_v1',experiment_id=sys.argv[1],status=status,
             summary='Gold3 conditional projection rank with full linear certificates; no root search or speed claim.',
             evidence=evidence,next_actions=['If rank0consistent, retire Gold3-only tightening on this cell and test genuinely higher joint moment constraints; otherwise independently replay the exact tightened family before support work.'])
    encoded=json.dumps(out,separators=(',',':'))
    if time.monotonic()-START>=84:
        out['status']='inconclusive';evidence['complete_projection_diagnostic']=False
        evidence['gold_rank_increment']=None;evidence['actual_summary']={};evidence['error']='late report'
        encoded=json.dumps(out,separators=(',',':'))
    assert len(encoded.encode())<=8192 and time.monotonic()-START<88, 'final output cap'
    print(encoded)


if __name__=='__main__': main()
