#!/usr/bin/env python3
"""Development-only unit and hostile-input tests; not an independent PCC review."""
import copy
import json
import random
import subprocess
import sys
import tempfile
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

    def test_direct_programmatic_replay_rejects_identity_and_offcurve_base(self):
        # The native adapter never permits infinity as a factor-base leaf.
        # The read-only Python interface must reject it even outside the CLI.
        base=self.base.copy()
        base[0]=None
        with self.assertRaisesRegex(ValueError,"identity is not an admissible"):
            checker.validate({},base,self.q,[])
        base[0]=(checker.P,1)
        with self.assertRaisesRegex(ValueError,"noncanonical"):
            checker.validate({},base,self.q,[])
        with self.assertRaisesRegex(ValueError,"off curve"):
            checker.validate({},self.base,(1,1),[])

    def test_producer_extracted_but_not_accepted_rows_are_not_successes(self):
        # Query 0G+0Q costs an attempted extraction; none of its claimed
        # candidate rows enters the authenticated relation matrix.
        manifest={"curve":"ordinary_prime31","p":checker.P,"n":checker.N,
                  "a":checker.A,"b":checker.B,"g":list(checker.G),
                  "base":[list(p) for p in self.base]}
        record={"target":list(self.q),"attempts":[
                  {"a":0,"b":0,"point":None,
                   "extraction":{"relations":[{}, {}, {}]},"receipts":[]}],
                "rank":0,"scalar":None,"final_replay_passed":False,
                "status":"query_limit"}
        with tempfile.TemporaryDirectory() as td:
            manifest_path=Path(td)/"base.json"
            record_path=Path(td)/"record.json"
            manifest_path.write_text(json.dumps(manifest))
            record_path.write_text(json.dumps(record))
            cmd=[sys.executable,str(HERE/"independent_rr5_replay.py"),
                 "--record",str(record_path),"--base",str(manifest_path)]
            done=subprocess.run(cmd,capture_output=True,text=True,check=False)
            self.assertEqual(done.returncode,0,done.stderr)
            report=json.loads(done.stdout)
            self.assertEqual(report["extracted_candidate_rows"],3)
            self.assertEqual(report["accepted_rows"],0)
            self.assertEqual(report["extracted_but_not_accepted_rows"],3)
            self.assertEqual(report["attempts_without_accepted_rows"],1)
            self.assertEqual(report["independent_rank"],0)
            self.assertIsNone(report["recovered"])
            record["status"]="solved"
            record_path.write_text(json.dumps(record))
            bad=subprocess.run(cmd,capture_output=True,text=True,check=False)
            self.assertNotEqual(bad.returncode,0)
            self.assertIn("terminal status/scalar contradiction",bad.stderr)

    def test_opt_in_dual_target_certificate_replays_original_rows(self):
        # Two authentic Prime31 relations imply L_0=1 and L_0-d=-4.
        # The separating row combination is row 1 minus row 2:
        # [C|-b|a] => [0,0,0,0,1,5]. No base logs enter this check.
        out=checker.validate({},self.base,self.q,self.fixture["relations"],
                             emit_dual_certificate=True)
        cert=out["dual_target_certificate"]
        self.assertEqual((cert["scalar"],cert["target_coefficient"],
                          cert["first_target_prefix"]),(5,1,2))
        self.assertEqual(cert["relation_weights"],[[1,1],[2,checker.N-1]])
        originals={1:[1,0,0,0,0,1],
                   2:[1,0,0,0,checker.N-1,checker.N-4]}
        self.assertTrue(checker.verify_dual_certificate(
            originals,cert["relation_weights"],5,4))
        for poison in ([[1,1],[2,1]], [[1,1],[1,checker.N-1]],
                       [[1,1],[3,checker.N-1]], [[1,0],[2,checker.N-1]]):
            with self.assertRaisesRegex(ValueError,"dual target certificate"):
                checker.verify_dual_certificate(originals,poison,5,4)
        with self.assertRaisesRegex(ValueError,"dual target certificate"):
            checker.verify_dual_certificate(
                originals,cert["relation_weights"],6,4)

        delayed=copy.deepcopy(self.fixture)
        delayed["relations"].insert(0,{"terms":[[1,1]],"a":2,"b":0})
        later=checker.validate({},self.base,self.q,delayed["relations"],
                               emit_dual_certificate=True)["dual_target_certificate"]
        self.assertEqual((later["relation_weights"],later["first_target_prefix"]),
                         ([[2,1],[3,checker.N-1]],3))
        self.assertIsNone(checker.validate(
            {},self.base,None,[],emit_dual_certificate=True)["dual_target_certificate"])

    def test_extracted_grevlex_b8_row_stays_underdetermined(self):
        # Exported from RR5 development09 b8-positive.stdout, not a planted
        # synthetic relation. Its *target* is a fixed positive fixture,
        # deliberately not an unknown-index full Prime31 target solve.
        path=HERE/"rr5_grevlex_b8_producer_row.json"
        data=json.loads(path.read_text())
        base=[checker.point(v,"grevlex B8 base") for v in data["base"]]
        q=checker.point(data["target_q"],"grevlex B8 Q")
        report=checker.validate({},base,q,data["relations"],emit_dual_certificate=True)
        self.assertIsNone(report["dual_target_certificate"])
        self.assertEqual((report["authenticated_rows"],report["independent_rank"]), (1,1))
        self.assertEqual(report["rank_without_target_column"],1)
        self.assertIsNone(report["first_target_prefix"])
        self.assertIsNone(report["recovered"])
        self.assertFalse(report["scalar_replay_passed"])
        # Both b=1 and the extracted signed leaves matter to the public
        # equality; the checker may not trust the producer's success label.
        bad=copy.deepcopy(data["relations"])
        bad[0]["terms"][1][1]=1
        with self.assertRaisesRegex(ValueError,"invalid public EC equality"):
            checker.validate({},base,q,bad)

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

    def test_producer_receipt_must_bind_to_extractor_candidate_prefix(self):
        # All accepted rows are mathematically true. A forged producer may
        # nevertheless attach them to unrelated extracted candidates. Their
        # group truth MUST NOT be admitted as evidence of extraction.
        manifest={"curve":"ordinary_prime31","p":checker.P,"n":checker.N,
                  "a":checker.A,"b":checker.B,"g":list(checker.G),
                  "base":[list(p) for p in self.base]}
        first={"canonical_terms":[[0,1]],"a":1,"b":0,
               "rank_before":0,"rank_after":1,
               "disposition":"Independent","recovered_scalar":None}
        second={"canonical_terms":[[0,1]],"a":checker.N-4,"b":1,
                "rank_before":1,"rank_after":2,
                "disposition":"Independent","recovered_scalar":5}
        rec={"target":list(self.q),
             "attempts":[
                 {"a":1,"b":0,"point":list(self.base[0]),
                  "extraction":{"relations":[{"terms":[[0,1]]}]},
                  "receipts":[first]},
                 {"a":checker.N-4,"b":1,"point":list(self.base[0]),
                  "extraction":{"relations":[{"terms":[[0,1]]}]},
                  "receipts":[second]}],
             "rank":2,"scalar":5,"status":"solved","final_replay_passed":True}
        with tempfile.TemporaryDirectory() as td:
            manifest_path=Path(td)/"base.json"
            record_path=Path(td)/"record.json"
            manifest_path.write_text(json.dumps(manifest))
            cmd=[sys.executable,str(HERE/"independent_rr5_replay.py"),
                 "--record",str(record_path),"--base",str(manifest_path)]
            def run(record):
                record_path.write_text(json.dumps(record))
                return subprocess.run(cmd,capture_output=True,text=True,check=False)
            good=run(rec)
            self.assertEqual(good.returncode,0,good.stderr)
            out=json.loads(good.stdout)
            self.assertEqual((out["authenticated_rows"],out["recovered"]),(2,5))
            self.assertEqual(out["extracted_candidate_rows"],2)

            bad=copy.deepcopy(rec)
            bad["attempts"][1]["extraction"]["relations"][0]["terms"]=[[1,1]]
            denied=run(bad)
            self.assertNotEqual(denied.returncode,0)
            self.assertIn("not canonical extraction prefix",denied.stderr)

            # The native adapter consumes candidates in order, not by
            # selecting a later candidate with a matching EC identity.
            bad=copy.deepcopy(rec)
            bad["attempts"][1]["extraction"]["relations"].insert(0,{"terms":[[1,1]]})
            denied=run(bad)
            self.assertNotEqual(denied.returncode,0)
            self.assertIn("not canonical extraction prefix",denied.stderr)

            # A and B may be algebraically changed to preserve the same
            # elliptic-curve equation; the original public-query labels
            # are still mandatory for extraction-provenance admission.
            bad=copy.deepcopy(rec)
            bad["attempts"][1]["receipts"][0]["a"]=1
            bad["attempts"][1]["receipts"][0]["b"]=0
            denied=run(bad)
            self.assertNotEqual(denied.returncode,0)
            self.assertIn("not canonical extraction prefix",denied.stderr)

    def test_producer_negative_first_leaf_normalizes_both_rhs_fields(self):
        # Exact original RelationAdapter convention: a negative leading
        # signed leaf negates *both* public RHS scalars, not just the terms.
        original_a,original_b=checker.N-1,2
        terms,a,b=checker.canonicalized_extraction(
            {"terms":[[0,-1]]},original_a,original_b,len(self.base),"sample")
        self.assertEqual((terms,a,b),([[0,1]],1,checker.N-2))
        self.assertEqual(checker.canonicalized_extraction(
            {"terms":[[2,1],[0,-1],[2,-1]]},checker.N-1,0,len(self.base),"sample"),
            ([[0,1]],1,0))
        for bad in ({"terms":[[0,5]]},{"terms":[[0,True]]},{"terms":[[0,1.0]]},
                    {"terms":[[len(self.base),1]]}):
            with self.assertRaises(ValueError):
                checker.canonicalized_extraction(bad,1,0,len(self.base),"poison")


    def test_splitmix_source_exact_query_schedule(self):
        # These four independent known-answer words come from the native u64
        # SplitMix64 constants and modulo-n reduction, not target logs.
        state = 42
        labels = []
        for _ in range(4):
            state, value = checker.splitmix_next(state)
            labels.append(value)
        self.assertEqual(labels, [49979948, 1033465976, 1897459893, 556465603])
        manifest = {"curve":"ordinary_prime31","p":checker.P,"n":checker.N,
                    "a":checker.A,"b":checker.B,"g":list(checker.G),
                    "base":[list(p) for p in self.base]}
        attempts = []
        for a, b in ((labels[0], labels[1]), (labels[2], labels[3])):
            qpt = checker.add(checker.scale(a, checker.G),
                              checker.scale(b, self.q))
            attempts.append({"a":a, "b":b, "point":list(qpt) if qpt else None,
                             "extraction":{"relations":[]}, "receipts":[]})
        record = {"target":list(self.q), "attempts":attempts, "rank":0,
                  "scalar":None, "status":"query_limit",
                  "final_replay_passed":False}
        with tempfile.TemporaryDirectory() as td:
            base_path = Path(td)/"base.json"
            record_path = Path(td)/"record.json"
            base_path.write_text(json.dumps(manifest))
            command = [sys.executable, str(HERE/"independent_rr5_replay.py"),
                       "--record", str(record_path), "--base", str(base_path),
                       "--seed", "42", "--max-queries", "2"]
            def run(data, options=command):
                record_path.write_text(json.dumps(data))
                return subprocess.run(options, capture_output=True, text=True, check=False)
            good = run(record)
            self.assertEqual(good.returncode, 0, good.stderr)
            out = json.loads(good.stdout)
            self.assertTrue(out["query_schedule_checked"])
            self.assertEqual(out["source_exact_scheduled_queries"], 2)
            self.assertEqual((out["authenticated_rows"],out["independent_rank"]), (0,0))

            # Point equality alone does not bind an attempt to the announced
            # reproducible query stream: all three mutations must fail closed.
            tampered = copy.deepcopy(record)
            tampered["attempts"][1]["a"] = (tampered["attempts"][1]["a"] + 1) % checker.N
            newpt = checker.add(checker.scale(tampered["attempts"][1]["a"], checker.G),
                                checker.scale(tampered["attempts"][1]["b"], self.q))
            tampered["attempts"][1]["point"] = list(newpt) if newpt else None
            denied = run(tampered)
            self.assertNotEqual(denied.returncode, 0)
            self.assertIn("SplitMix64 query schedule", denied.stderr)

            reordered = copy.deepcopy(record)
            reordered["attempts"].reverse()
            denied = run(reordered)
            self.assertNotEqual(denied.returncode, 0)
            self.assertIn("SplitMix64 query schedule", denied.stderr)

            omitted = copy.deepcopy(record)
            omitted["attempts"] = omitted["attempts"][1:]
            denied = run(omitted)
            self.assertNotEqual(denied.returncode, 0)
            self.assertIn("query_limit omits public attempts", denied.stderr)

            # A deadline can legitimately end a retained prefix early.
            prefix = copy.deepcopy(record)
            prefix["attempts"] = prefix["attempts"][:1]
            prefix["status"] = "deadline"
            accepted = run(prefix)
            self.assertEqual(accepted.returncode, 0, accepted.stderr)
            self.assertEqual(json.loads(accepted.stdout)["source_exact_scheduled_queries"], 1)

            denied = run(record, command[:-2]) # --seed without its paired cap
            self.assertNotEqual(denied.returncode, 0)


if __name__=="__main__":
    unittest.main()
