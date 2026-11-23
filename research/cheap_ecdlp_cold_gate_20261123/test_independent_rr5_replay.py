#!/usr/bin/env python3
"""Development-only unit and hostile-input tests; not an independent PCC review."""
import copy
import json
import random
import unittest
from pathlib import Path
import independent_rr5_replay as checker

HERE=Path(__file__).resolve().parent

class ReplayChecks(unittest.TestCase):
    def setUp(self):
        self.fixture=json.loads((HERE/"public_rr5_sample.json").read_text())
        self.base=[checker.point(v,"base") for v in self.fixture["base"]]
        self.q=checker.point(self.fixture["target_q"],"target")

    def check(self, f):
        return checker.validate(f,self.base,self.q,f["relations"])

    def test_synthetic_public_prime31(self):
        report=self.check(self.fixture)
        self.assertEqual((report["first_target_prefix"], report["recovered"]), (2,5))
        self.assertEqual((report["independent_rank"],report["rank_without_target_column"]),(2,1))

    def test_delayed_visibility(self):
        data=copy.deepcopy(self.fixture)
        data["relations"].insert(0,{"terms":[[1,1]],"a":2,"b":0})
        out=self.check(data)
        self.assertEqual((out["first_target_prefix"],out["recovered"]),(3,5))

    def test_dependent_inverse_and_zero_identity(self):
        data=copy.deepcopy(self.fixture)
        data["relations"].append({"terms":[[0,-1]],"a":checker.N-1,"b":0})
        data["relations"].append({"terms":[],"a":0,"b":0})
        out=self.check(data)
        self.assertEqual((out["independent_rank"],out["dependent_rows"]),(2,2))

    def test_random_valid_signed_rows_do_not_invent_target(self):
        rng=random.Random(0xCE31)
        base_logs=[1,2,3,4]
        for _ in range(120):
            rows=[]
            for i in range(rng.randint(0,15)):
                terms=[]
                for j in range(rng.randint(0,4)):
                    terms.append([rng.randrange(4),rng.choice([-1,1])])
                b=rng.randrange(7)
                left=sum(base_logs[k]*c for k,c in terms)
                a=(left-5*b) % checker.N
                rows.append({"terms":terms,"a":a,"b":b})
            checker.validate({},self.base,self.q,rows)

    def test_mutated_public_equation_rejected(self):
        data=copy.deepcopy(self.fixture)
        data["relations"][1]["b"]=2
        with self.assertRaisesRegex(ValueError,"invalid public EC equality"):
            self.check(data)

    def test_poisoned_duplicate_and_bad_coordinates_rejected(self):
        data=copy.deepcopy(self.fixture)
        data["relations"].append({"terms":[[0,4],[1,1]],"a":1,"b":0})
        with self.assertRaisesRegex(ValueError,"more than four"):
            self.check(data)
        with self.assertRaises(ValueError):
            checker.point([checker.P,1],"Q")
        with self.assertRaises(ValueError):
            checker.point([1,1],"Q")

    def test_infinity_is_known_at_prefix_zero(self):
        r=checker.validate({},self.base,None,[])
        self.assertEqual((r["first_target_prefix"],r["recovered"]),(0,0))

    def test_producer_receipt_checks(self):
        r1={"canonical_terms":[[0,1]],"a":1,"b":0,
            "rank_before":0,"rank_after":1,"disposition":"Independent","recovered_scalar":None}
        r2={"canonical_terms":[[0,1]],"a":checker.N-4,"b":1,
            "rank_before":1,"rank_after":2,"disposition":"Independent","recovered_scalar":5}
        recs=[r1,r2]
        out=checker.validate({},self.base,self.q,recs,producer=True)
        self.assertEqual(out["recovered"],5)
        bad=copy.deepcopy(recs)
        bad[1]["rank_before"]=0
        with self.assertRaisesRegex(ValueError,"producer rank disagreement"):
            checker.validate({},self.base,self.q,bad,producer=True)
        bad=copy.deepcopy(recs)
        bad[1]["recovered_scalar"]=6
        with self.assertRaisesRegex(ValueError,"producer recovered_scalar mismatch"):
            checker.validate({},self.base,self.q,bad,producer=True)

if __name__=="__main__":
    unittest.main()
