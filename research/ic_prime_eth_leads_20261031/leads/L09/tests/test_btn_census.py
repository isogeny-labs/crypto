import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))
from btn_census import census
def test_hand1():
    R = census(os.path.join(os.path.dirname(__file__), "hand1.cnf"))
    assert (R["s"], R["V"], R["eq_verts"], R["free_verts"]) == (12, 9, 2, 1), R   # hand count, see REPORT
    assert R["degree2_check"]
    Rb = census(os.path.join(os.path.dirname(__file__), "hand1.cnf"), mode="bb")
    # bb: ports v1:3 v2:3 v3:2 v4:2 v5:1 -> s=3+3+1+1+1=9, V=3+1+1+free1=6... eq 2
    assert (Rb["s"], Rb["V"]) == (9, 3 + 2 + 1), Rb
    print("ok")
if __name__ == "__main__": test_hand1()
