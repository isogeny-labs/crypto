"""Arithmetic/gate unit controls; no source-case certificate production."""
from pathlib import Path
import json,os,unittest
import prime_lefschetz_certificate as cert
import lefschetz_controls as core

ROOT=Path(os.environ['N83_LEFSCHETZ_TEST_ARTIFACTS'])
ROOT.mkdir(parents=True,exist_ok=False)
def save(name,value):
    with (ROOT/name).open('x') as f:json.dump(value,f,indent=2);f.write('\n')

class Arithmetic(unittest.TestCase):
    def test_structure_and_rejected_fields(self):
        d=cert.structural(27,cert.P);save('structure.json',d)
        self.assertEqual(sum(x['dimension'] for x in d['hilbert']),2**27)
        self.assertEqual(d['degrees'][-1]['minimum_slack'],1)
        self.assertEqual(d['first_uncovered_slack'],-1)
        for ell,p in [(26,cert.P),(27,101),(27,2)]:
            with self.assertRaises(AssertionError):cert.structural(ell,p)
    def test_upper_identity_and_corruption(self):
        d=cert.upper_witness(cert.P);save('upper.json',d)
        h={mask:int(c) for mask,c in d['normalized_terms']}
        mask=min(h);h[mask]=(h[mask]+1)%cert.P
        residual=core.up(core.up(h,27,cert.P),27,cert.P)
        save('corrupted-upper.json',{'changed_mask':mask,'changed_coefficient':str(h[mask]),
             'residual':[[m,str(c)] for m,c in sorted(residual.items())]})
        self.assertTrue(residual)

if __name__=='__main__':unittest.main()
