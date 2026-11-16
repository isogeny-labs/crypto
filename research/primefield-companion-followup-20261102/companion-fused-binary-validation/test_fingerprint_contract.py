"""Deterministic metadata tests; no operator/kernel execution."""
from pathlib import Path
import sys
import unittest
sys.path.insert(0, str(Path(__file__).resolve().parent.parent/'companion-trace-codec'))
from trace_codec import fingerprint
from fingerprint_contract import byte_bounds, validate


class ContractTests(unittest.TestCase):
    def fixture(self, upper):
        r,n,s,a,b=7,3,2,2,1
        bound=s*n*n*(r-1)**4*(a+1)*(b+1)
        d=max(1,(bound.bit_length()+7)//8)
        packed=lambda slots:(1 << (8*d*slots))-1 if upper else 0
        trace=[dict(stage='setup',bound=bound,digit_bytes=d,inverse=r-1 if upper else 1)]
        trace += [dict(stage='column',column=j,P=packed(2*n-1),T=packed(n),
                       Q=packed(2*n-1),Y=packed(n),acc=packed(n)) for j in range(s)]
        trace.append(dict(stage='final',packed=packed(n)))
        return (r,n,s,a,b),fingerprint(trace)

    def test_exact_envelope_endpoints(self):
        for upper in (False,True):
            args,record=self.fixture(upper)
            self.assertEqual(record['bytes'],byte_bounds(*args)[int(upper)])
            validate(record,*args)

    def test_mutated_metadata_is_rejected(self):
        args,record=self.fixture(False)
        for key,value in [('schema','legacy'),('sha256','A'*64),('sha256','0'*63),
                          ('bytes',True),('bytes',0),('bytes',record['bytes']-1),
                          ('bytes',byte_bounds(*args)[1]+1)]:
            changed=dict(record);changed[key]=value
            with self.assertRaises(ValueError):validate(changed,*args)
        for changed in [{},dict(record,extra=1)]:
            with self.assertRaises(ValueError):validate(changed,*args)

    def test_invalid_parameters_are_rejected(self):
        for args in [(7,0,1,2,1),(7,1,0,2,1),(7,1,1,1,1),(7,1,1,-1,1),
                     (7,1,1,7,1),(3,1,1,2,1),(7,True,1,2,1)]:
            with self.assertRaises(ValueError):byte_bounds(*args)


if __name__=='__main__':unittest.main()
