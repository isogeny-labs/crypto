#!/usr/bin/env python3
"""Development checks using synthetic metadata, not algorithm executions."""
import csv
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

ANALYZER = Path(os.environ["ANALYZER"]).resolve()

class AnalyzerChecks(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.rows = []
        for arm in "uvwxyz":
            for case in range(3):
                for round_ in range(10):
                    self.rows.append([arm,str(case),str(111*(case+1)),str(round_),
                        str(round_==0).lower(),"verified",str([42,45678,54321][case]),
                        "10","10"] + ["1.0"]*6 + ["0"]*7 + ["0.001","L0"])
                    (self.root/f"cert-{arm}-{case}-{round_}.txt").write_text(
                        f"SYNTHETIC VALIDATOR TEST ONLY case={case} round={round_}\n")
    def tearDown(self):
        self.temp.cleanup()
    def run_analyzer(self, rows=None):
        path=self.root/"rows.csv"
        with path.open("w") as f:
            writer=csv.writer(f,lineterminator="\n")
            writer.writerow(["synthetic column"]*24)
            writer.writerows(self.rows if rows is None else rows)
        return subprocess.run([str(ANALYZER),str(path),str(self.root)],capture_output=True,text=True)
    def test_complete_six_arm_panel(self):
        result=self.run_analyzer()
        self.assertEqual(result.returncode,0,result.stderr)
        self.assertIn("PASS 180 rows, 162 measured, 18 complete cells",result.stdout)
        with (self.root/"summary.csv").open() as f:
            summary=list(csv.DictReader(f))
        self.assertEqual(len(summary),108)
        self.assertEqual({r["arm"] for r in summary},set("uvwxyz"))
    def test_missing_or_duplicate_cell(self):
        for rows in [self.rows[:-1],self.rows+[self.rows[0]],
                     [r for r in self.rows if r[0]!="z"]]:
            self.assertNotEqual(self.run_analyzer(rows).returncode,0)
    def test_mismatched_full_certificate(self):
        (self.root/"cert-z-2-9.txt").write_text("DIFFERENT SYNTHETIC CERTIFICATE\n")
        self.assertNotEqual(self.run_analyzer().returncode,0)
    def test_empty_certificates(self):
        for arm in "uvwxyz":
            (self.root/f"cert-{arm}-0-0.txt").write_text(" \n")
        self.assertNotEqual(self.run_analyzer().returncode,0)
    def test_row_order_does_not_change_summary(self):
        self.assertEqual(self.run_analyzer().returncode,0)
        expected=(self.root/"summary.csv").read_bytes()
        self.assertEqual(self.run_analyzer(list(reversed(self.rows))).returncode,0)
        self.assertEqual((self.root/"summary.csv").read_bytes(),expected)
    def test_malformed_fields(self):
        for column,value in [(0,"bad"),(1,"3"),(2,"999"),(3,"10"),
                             (4,"false"),(5,"failed"),(6,"999"),(23,"L2")]:
            rows=[r.copy() for r in self.rows];rows[0][column]=value
            self.assertNotEqual(self.run_analyzer(rows).returncode,0,(column,value))
        for row in [0,1]:
            for column,value in [(9,"NaN"),(14,"-1"),(14,"inf"),(22,"NaN"),
                                 (7,"-1"),(15,"broken"),(21,"18446744073709551616")]:
                with self.subTest(row=row,column=column,value=value):
                    rows=[r.copy() for r in self.rows];rows[row][column]=value
                    self.assertNotEqual(self.run_analyzer(rows).returncode,0)

if __name__=="__main__": unittest.main()
