#!/usr/bin/env python3
"""Independent, source-separated BigInt verification of Prime31 transformed-slot S3 Rust output.
Fixture generation and post-solve verification only, NOT an ECDLP extractor.
"""
import json
import sys

P, A, B, N = 2147483647, 847514846, 2087444724, 2147446831
G = (625037715, 2100627251)
MASK = (1 << 64) - 1
SEED = 0x54524e5333463301
WINDOWS = [list(range(0,8)),list(range(8,16)),list(range(16,24)),list(range(23,31))]

def splitmix64(state):
    state = (state + 0x9e3779b97f4a7c15) & MASK
    z = state
    z = ((z ^ (z >> 30)) * 0xbf58476d1ce4e5b9) & MASK
    z = ((z ^ (z >> 27)) * 0x94d049bb133111eb) & MASK
    return state, z ^ (z >> 31)

def neg(q):
    return (q[0], (-q[1]) % P) if q is not None else None

def ec_add(pt, qt):
    if pt is None: return qt
    if qt is None: return pt
    x, y = pt
    u, v = qt
    if x == u and (y + v) % P == 0: return None
    if x == u: slope = ((3*x*x + A) * pow(2*y, -1, P)) % P
    else: slope = ((v-y) * pow((u-x) % P, -1, P)) % P
    rx = (slope*slope - x - u) % P
    return rx, (slope*(x-rx) - y) % P

def ec_mul(k, q):
    out = None
    while k:
        if k & 1: out = ec_add(out, q)
        q = ec_add(q, q)
        k >>= 1
    return out

def s3(x, y, z):
    return ((x-y)**2*z*z - 2*((x+y)*(x*y+A)+2*B)*z
            + (x*y-A)**2 - 4*B*(x+y)) % P

def product(items):
    result = 1
    for x in items: result = result*x % P
    return result


def s3_fixed_poly(t):
    t2=t*t%P
    return {e:c%P for e,c in [
        ((2,2),1), ((1,2),-2*t), ((0,2),t2),
        ((2,1),-2*t), ((1,1),-2*(t2+A)),
        ((0,1),-2*(A*t+2*B)), ((2,0),t2),
        ((1,0),-2*A*t-4*B), ((0,0),A*A-4*B*t)
    ] if c%P}

def multiply_bivariate(f,g):
    o={}
    for (i,j),x in f.items():
        for (k,l),y in g.items():
            e=(i+k,j+l)
            o[e]=(o.get(e,0)+x*y)%P
    return {e:x for e,x in o.items() if x}

def expanded_slot_terms(slot):
    result={(0,0):1}
    for t,_ in slot: result=multiply_bivariate(result,s3_fixed_poly(t))
    assert result[(16,16)]==1 and len(result)<=289
    return len(result)

def base31():
    points = []
    for x in range(10000):
        rhs = (x*x*x+A*x+B) % P
        y = pow(rhs, (P+1)//4, P)
        if y and y*y % P == rhs:
            points.append((x, min(y, P-y)))
            if len(points) == 31: break
    assert len(points) == 31
    assert [x for x,_ in points] == [0,1,2,3,5,6,8,9,10,12,14,16,18,19,22,26,27,
                                      33,35,37,38,41,42,46,47,50,53,56,59,60,66]
    return points

def run(cases_per_window):
    assert 1 <= cases_per_window <= 128
    assert ec_mul(N, G) is None
    base = base31()
    state = SEED
    first_cases = []
    windows = []
    counters = dict(ordinary=0,exceptional=0,h_zero=0,s3_zero=0,k_zero=0,
                    wrong_slot_rejected=0,mutated_public_target_rejected=0,
                    post_solve_sign_lift_verified=0)
    for wi,window in enumerate(WINDOWS):
        slot = [[ec_mul(1<<i, base[j]) for j in window] for i in range(8)]
        assert len({pt[0] for row in slot for pt in row}) == 64
        good = bad = 0
        for ci in range(cases_per_window):
            leaf = []
            choices = []
            signs = []
            for i in range(8):
                state, v = splitmix64(state)
                j = v % 8
                state, v = splitmix64(state)
                bit = v & 1
                choices.append(j)
                signs.append(bit)
                leaf.append(neg(slot[i][j]) if bit else slot[i][j])
            suffix = [None]*8
            suffix[7] = leaf[7]
            for i in range(6,-1,-1):
                suffix[i] = ec_add(leaf[i], suffix[i+1])
            if any(q is None for q in suffix):
                counters["exceptional"] += 1
                bad += 1
                continue
            t = [q[0] for q in leaf]
            z = [q[0] for q in suffix]
            target = suffix[0]
            assert z[7] == t[7] and z[0] == target[0]
            for i in range(8):
                assert product((t[i] - pt[0]) % P for pt in slot[i]) == 0
                counters["h_zero"] += 1
            for i in range(7):
                assert s3(t[i], z[i+1], z[i]) == 0
                assert product(s3(pt[0],z[i+1],z[i]) for pt in slot[i]) == 0
                counters["s3_zero"] += 1
                counters["k_zero"] += 1
            bad_t = next(x for x in range(100) if product((x - pt[0]) % P for pt in slot[0]))
            assert product((bad_t-pt[0]) % P for pt in slot[0])
            counters["wrong_slot_rejected"] += 1
            poisoned = product(s3(pt[0],z[1],(z[0]+1)%P) for pt in slot[0])
            counters["mutated_public_target_rejected"] += int(poisoned != 0)
            # Independent of planted signs: derive selectors solely from x coordinates.
            selected = []
            for i in range(8):
                selected.append(next(pt for pt in slot[i] if pt[0]==t[i]))
            rec = [None]*8
            rec[7] = selected[7]
            tail = rec[7]
            for i in range(6,-1,-1):
                found = None
                for pt in (selected[i],neg(selected[i])):
                    s = ec_add(pt,tail)
                    if s is not None and s[0] == z[i]:
                        found = (pt,s)
                        break
                assert found is not None
                rec[i], tail = found
            if tail != target:
                assert neg(tail) == target
                rec = [neg(pt) for pt in rec]
            assert sum([1 for i in range(8) if selected[i][0]==t[i]])==8
            out = None
            for pt in rec: out = ec_add(out,pt)
            assert out == target
            counters["post_solve_sign_lift_verified"] += 1
            counters["ordinary"] += 1
            good += 1
            if ci == 0:
                first_cases.append(dict(window=wi,choices=choices,planted_signs=signs,
                    transformed_roots=t,suffix_x=z,target_q=list(target),mutated_k0=poisoned))
        windows.append(dict(window=wi,distinct_slot_roots=64,ordinary=good,exceptional=bad,
                            k_polynomial_term_counts=[expanded_slot_terms(slot[i]) for i in range(7)]))
    return counters, windows, first_cases

def main():
    assert len(sys.argv)==2, "usage: verify_prime31_transformed_slot_f3.py output.json"
    rust=json.load(open(sys.argv[1],encoding="utf-8"))
    assert rust["schema"]=="prime31.transformed_slot_f3.v1"
    assert rust["seed"]=="0x54524e5333463301"
    assert rust["curve"]["p"]==P and rust["curve"]["n"]==N
    assert rust["public_only_unknown_index_extraction"] is False
    assert rust["complete_ecdlp_solved"] is False
    assert rust["planted_witness_controls_only"] is True
    assert rust["generic_sum_table_calls"]==0
    assert rust["direct_variables"]==14 and rust["direct_equations"]==15
    assert rust["eliminated_variables"]==7 and rust["eliminated_equations"]==8
    counters,windows,first=run(rust["cases_per_window"])
    assert rust["tested"]==4*rust["cases_per_window"]
    for k,value in counters.items(): assert rust[k]==value,(k,rust[k],value)
    assert rust["windows"]==windows
    assert rust["first_case_per_window"]==first
    print(json.dumps(dict(schema="prime31.transformed_slot_f3.independent.v1",
                          verified_cases=rust["tested"],
                          independent_checks="all cases, original affine EC, slot polynomials, S3 and sign lifting",
                          pcc_signed=False),sort_keys=True))

if __name__=="__main__":
    main()
