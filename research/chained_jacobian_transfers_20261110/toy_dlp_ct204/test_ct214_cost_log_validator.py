"""Synthetic positive and adversarial CT214 accounting tests, no execution attestation."""
import json
import unittest
from ct214_cost_log_validator import validate, InvalidCostLog

def fixture():
    rows, e_times, j_times = [], [], []
    for rnd in range(7):
        for offset in range(43):
            k=(offset+rnd*17)%43
            et,jt=100+k,300+k
            e_times.append(et)
            j_times.append(jt)
            rows.append({"round":rnd,"offset":offset,"target_index":k,
                         "arm_order":"EJ" if (k+rnd)%2==0 else "JE",
                         "direct_ns":et,"jacobian_ns":jt,
                         "direct_additions":13+k//7,"jacobian_additions":13+k//7,
                         "direct_baby_entries":7,"jacobian_baby_entries":7})
    total={"field":31,"order":43,"rounds":7,"pairs":301,
           "direct_total_ns":sum(e_times),"jacobian_total_ns":sum(j_times),
           "direct_median_ns":sorted(e_times)[150],"jacobian_median_ns":sorted(j_times)[150],
           "direct_bsgs_additions":4690,"jacobian_bsgs_additions":4690,
           "ratio_direct_over_jacobian":round(sum(e_times)/sum(j_times),9),
           "memory":"unmeasured","pcc":"unverified",
           "status":"toy_only_not_full_reference"}
    return rows,total

def logs(rows,total):
    return "\n".join(["CT214_PAIR_JSON "+json.dumps(r) for r in rows]
                     + ["CT213_TOY_COST_JSON "+json.dumps(total)])

class TestAudit(unittest.TestCase):
    def test_valid(self):
        a,t=fixture()
        self.assertEqual(validate(logs(a,t))["valid_pairs"],301)
    def test_missing(self):
        a,t=fixture()
        with self.assertRaises(InvalidCostLog): validate(logs(a[:-1],t))
    def test_duplicate(self):
        a,t=fixture();a[-1]=a[0]
        with self.assertRaises(InvalidCostLog): validate(logs(a,t))
    def test_tampered_time(self):
        a,t=fixture();a[0]["direct_ns"]+=1
        with self.assertRaises(InvalidCostLog): validate(logs(a,t))
    def test_tampered_add(self):
        a,t=fixture();a[0]["jacobian_additions"]+=1
        with self.assertRaises(InvalidCostLog): validate(logs(a,t))
    def test_tampered_order(self):
        a,t=fixture();a[0]["arm_order"]="JE"
        with self.assertRaises(InvalidCostLog): validate(logs(a,t))
    def test_false_pcc(self):
        a,t=fixture();t["pcc"]="verified"
        with self.assertRaises(InvalidCostLog): validate(logs(a,t))

if __name__ == "__main__":
    unittest.main()
