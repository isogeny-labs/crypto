#!/usr/bin/env python3
"""Independent Python BigInt replay of four-plane graph-chart Prime31 candidate.
The grader is separated from the algebraic Rust candidate and does not send it
witness labels. Uses the independently authored exterior-4 group oracle.
"""
import json,sys
import verify_prime31_rr9_exterior4 as oracle


def rr_row(q,r):
    x,y=q
    rx,ry=r
    return [((x-rx)*pow(x,j,oracle.P))%oracle.P for j in range(4)]+[
        ((y+ry)*pow(x,j,oracle.P))%oracle.P for j in range(4)
    ]

def det_gauss(rows):
    n=len(rows)
    a=[list(row) for row in rows]
    z=1
    for k in range(n):
        pivot=next((i for i in range(k,n) if a[i][k]),None)
        if pivot is None:return 0
        if pivot!=k:
            a[pivot],a[k]=a[k],a[pivot]
            z=-z
        q=a[k][k]
        z=z*q%oracle.P
        iq=pow(q,oracle.P-2,oracle.P)
        for i in range(k+1,n):
            factor=a[i][k]*iq%oracle.P
            for j in range(k+1,n):
                a[i][j]=(a[i][j]-factor*a[k][j])%oracle.P
    return z%oracle.P

def graph_of_rows(rows):
    a=[list(row) for row in rows]
    z=1
    for k in range(4):
        pivot=next((i for i in range(k,4) if a[i][k]),None)
        assert pivot is not None, "inadmissible graph chart"
        if pivot!=k:
            a[pivot],a[k]=a[k],a[pivot]
            z=-z
        q=a[k][k]
        z=z*q%oracle.P
        iq=pow(q,oracle.P-2,oracle.P)
        for j in range(k,8):
            a[k][j]=a[k][j]*iq%oracle.P
        for i in range(4):
            if i==k:continue
            factor=a[i][k]
            for j in range(k+1,8):
                a[i][j]=(a[i][j]-factor*a[k][j])%oracle.P
            a[i][k]=0
    return [row[4:] for row in a],z

def det_four_2plus2(d):
    total=0
    for i,j in ((0,1),(0,2),(0,3),(1,2),(1,3),(2,3)):
        k,h=[c for c in range(4) if c not in (i,j)]
        top=(d[0][i]*d[1][j]-d[0][j]*d[1][i])%oracle.P
        bot=(d[2][k]*d[3][h]-d[2][h]*d[3][k])%oracle.P
        term=top*bot%oracle.P
        total=(total + (term if (i+j)%2 else -term))%oracle.P
    return total

def verify_graph_identity(slots,planted):
    # Independent Fp implementation; the Rust producer cannot provide these
    # polynomial identities or the hidden word choices to the candidate.
    for R in (planted,oracle.G):
        words=[[0]*8]+[
            [(case*97+i*23)%8 for i in range(8)] for case in range(30)
        ]
        for word in words:
            rows=[rr_row(slots[i][word[i]],R) for i in range(8)]
            lhs=det_gauss(rows)
            lm,ld=graph_of_rows(rows[:4])
            rm,rd=graph_of_rows(rows[4:])
            delta=[[(rm[i][j]-lm[i][j])%oracle.P for j in range(4)] for i in range(4)]
            rhs=ld*rd*det_four_2plus2(delta)%oracle.P
            assert lhs==rhs
            if R==planted and word==[0]*8:
                assert lhs==0
    print("INDEPENDENT_PYTHON_GRAPH_MATH_PASS: 62 original-curve residues")

def verify(path):
    with open(path,encoding="utf8") as f:
        data=json.load(f)
    assert data["p"]==oracle.P and data["group_order"]==oracle.N
    assert data["weights"]==list(oracle.W) and data["base_x"]==[0,1,2,3]
    assert data["B4_full_pairs"]==8**8
    assert data["B8_full_pairs"]==16**8
    assert data["unit_contraction_products_per_pair"]==30
    assert data["B8_full_chart_products"]==30*16**8
    assert data["B8_1024_full_chart_products"]==30*16**8*1024
    assert data["B4_half_graph_storage_bytes"]==2*4096*16*8
    assert data["B4_half_exterior_storage_bytes"]==2*4096*70*8
    assert data["B8_half_graph_storage_bytes"]==2*65536*16*8
    assert data["B8_half_exterior_storage_bytes"]==2*65536*70*8
    assert data["separate_truth_grader_excluded_from_candidate_time"] is True
    slots=oracle.supports()
    planted=oracle.replay(slots,[0]*8)
    verify_graph_identity(slots,planted)
    assert planted != oracle.G
    cases=data["cases"]
    assert len(cases)==2
    seen=set()
    for case in cases:
        name=case["case"]
        assert name in {"planted_B4","G_unplanted_B4"} and name not in seen
        seen.add(name)
        target=tuple(case["target"])
        assert target==(planted if name=="planted_B4" else oracle.G)
        assert 1<=case["algebraic_pairs"]<=8**8
        assert case["field_coordinate_product_muls"]==30*case["algebraic_pairs"]
        assert case["half_table_size"]==8**4
        assert case["candidate_public_only"] is True
        assert case["generic_group_table_in_candidate"] is False
        assert case["candidate_wall_ms_including_chart_tables"]>=0
        assert oracle.graded_truth(slots,target)==case["independent_truth_exists"]
        word=case["witness"]
        if word is None:
            assert case["algebraic_pairs"]==8**8
            assert case["independent_truth_exists"] is False
        else:
            assert len(word)==8 and all(type(i) is int and 0<=i<8 for i in word)
            assert oracle.replay(slots,word)==target
            assert case["independent_truth_exists"] is True
            points=[slots[j][i] for j,i in enumerate(word)]
            assert len(set(points))==8
            assert all(p!=oracle.neg(target) for p in points)
    assert seen=={"planted_B4","G_unplanted_B4"}
    print("INDEPENDENT_PYTHON_PASS: RR9 graph chart / B4 both labels, full negative and 30-mul costs")

if __name__=="__main__":
    assert len(sys.argv)==2
    verify(sys.argv[1])
