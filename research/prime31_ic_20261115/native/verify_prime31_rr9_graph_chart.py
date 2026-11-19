#!/usr/bin/env python3
"""Independent Python BigInt replay of four-plane graph-chart Prime31 candidate.
The grader is separated from the algebraic Rust candidate and does not send it
witness labels. Uses the independently authored exterior-4 group oracle.
"""
import json,sys
import verify_prime31_rr9_exterior4 as oracle

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
