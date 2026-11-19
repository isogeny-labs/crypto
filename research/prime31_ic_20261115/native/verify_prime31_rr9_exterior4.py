#!/usr/bin/env python3
"""Independent Python BigInt replay of Prime31 exterior-4 relation candidate.
The Python group-sum grader is OUTSIDE the Rust algebraic candidate and
never supplies it any witnesses. Validates all emitted cases and work counts.
"""
import json
import sys

P = 2147483647
A, B = 847514846, 2087444724
N = 2147446831
G = (625037715, 2100627251)
W = (1, 3, 5, 7, 11, 13, 17, 19)

def add(u, v):
    if u is None:
        return v
    if v is None:
        return u
    x, y = u
    z, w = v
    if x == z and (y+w) % P == 0:
        return None
    if x == z:
        slope = (3*x*x + A) * pow(2*y, P-2, P) % P
    else:
        slope = (w-y) * pow(z-x, P-2, P) % P
    qx = (slope*slope - x-z) % P
    return (qx, (slope*(x-qx)-y) % P)

def neg(q):
    if q is None:
        return None
    return (q[0], (-q[1]) % P)

def scalar(k,q):
    r = None
    while k:
        if k&1:
            r=add(r,q)
        q=add(q,q)
        k >>= 1
    return r

def base():
    points=[]
    for x in range(100):
        rhs=(x*x*x + A*x + B)%P
        y=pow(rhs,(P+1)//4,P)
        if y and y*y%P == rhs:
            points.append((x,min(y,P-y)))
            if len(points)==4:
                break
    assert [x for x,y in points]==[0,1,2,3]
    return points

def supports():
    points=base()
    slots=[]
    xs=set()
    for w in W:
        pos=[scalar(w,q) for q in points]
        for q in pos:
            assert q is not None and q[0] not in xs
            xs.add(q[0])
        slots.append(pos+[neg(q) for q in pos])
    assert len(xs)==32
    return slots

def replay(slots,word):
    t=None
    for j,v in enumerate(word):
        t=add(t,slots[j][v])
    return t

def graded_truth(slots,target):
    # Independently executed with whole-group EC sums. Never used for
    # candidate relation discovery or any candidate timing.
    right=set()
    for a in range(8):
        for b in range(8):
            ab=add(slots[4][a],slots[5][b])
            for c in range(8):
                abc=add(ab,slots[6][c])
                for d in range(8):
                    right.add(add(abc,slots[7][d]))
    for a in range(8):
        for b in range(8):
            ab=add(slots[0][a],slots[1][b])
            for c in range(8):
                abc=add(ab,slots[2][c])
                for d in range(8):
                    left=add(abc,slots[3][d])
                    if add(target,neg(left)) in right:
                        return True
    return False

def verify(path):
    with open(path,encoding="utf8") as f:
        data=json.load(f)
    assert data["p"]==P and data["group_order"]==N
    assert data["weights"]==list(W) and data["base_x"]==[0,1,2,3]
    assert data["B4_full_pairs"]==8**8
    assert data["B8_full_pairs"]==16**8
    assert data["B8_full_coordinate_products"]==70*16**8
    assert data["B8_1024_full_coordinate_products"]==70*16**8*1024
    assert data["separate_truth_grader_excluded_from_candidate_time"] is True
    slots=supports()
    expected_planted=replay(slots,[0]*8)
    assert expected_planted != G
    cases=data["cases"]
    assert len(cases)==2
    seen=set()
    for case in cases:
        name=case["case"]
        assert name in {"planted_B4","G_unplanted_B4"} and name not in seen
        seen.add(name)
        target=tuple(case["target"])
        assert target == (expected_planted if name=="planted_B4" else G)
        assert 1 <= case["algebraic_pairs"] <= 8**8
        assert case["field_coordinate_product_muls"]==70*case["algebraic_pairs"]
        assert case["half_table_size"]==8**4
        assert case["candidate_public_only"] is True
        assert case["generic_group_table_in_candidate"] is False
        assert case["candidate_wall_ms_including_wedge_tables"] >= 0
        assert graded_truth(slots,target)==case["independent_truth_exists"]
        word=case["witness"]
        if word is None:
            assert case["algebraic_pairs"] == 8**8
            assert case["independent_truth_exists"] is False
        else:
            assert len(word)==8 and all(isinstance(i,int) and 0<=i<8 for i in word)
            assert replay(slots,word)==target
            assert case["independent_truth_exists"] is True
            q=[slots[j][i] for j,i in enumerate(word)]
            assert len(set(q))==8
            assert all(t!=neg(target) for t in q)
    assert seen=={"planted_B4","G_unplanted_B4"}
    print("INDEPENDENT_PYTHON_PASS: B4 supports, two exact truths, field-work and EC witnesses")

if __name__=="__main__":
    assert len(sys.argv)==2
    verify(sys.argv[1])
