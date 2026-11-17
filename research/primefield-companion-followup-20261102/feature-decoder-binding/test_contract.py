"""Fixed contract checks; recorded oracles are not fresh source computations."""
from pathlib import Path
import json,unittest
import adapter,preserved_decoder
from reload_factor import load
D=Path(__file__).resolve().parent

class RecordedOracle:
    def __init__(self,point,signs):
        self.point=point
        self.signs=tuple(signs)
        self.calls=[]
    def __call__(self,offset):
        self.calls.append(offset)
        return self.signs[offset]

class Contracts(unittest.TestCase):
    def test_four_recorded_public_oracles(self):
        inputs=json.loads((D/'contract-input.json').read_text())
        def forbidden(*args,**kwargs):raise AssertionError('compiler must not run in decoder')
        old_compile,old_features=preserved_decoder.compile,preserved_decoder.Features
        preserved_decoder.compile=preserved_decoder.Features=forbidden
        try:
            bound=adapter.bind()
            view=adapter.legacy_view(bound)
            expected_products=sum(len(t['offsets']) for t in view['terms'])
            for case in inputs['cases']:
                with self.subTest(point=case['point']):
                    oracle=RecordedOracle(case['point'],case['signs'])
                    value,counts=adapter.decode(bound,oracle)
                    self.assertEqual(value,case['expected_scalar'])
                    self.assertEqual(oracle.calls,list(range(16)))
                    self.assertEqual(counts['signs'],case['signs'])
                    self.assertEqual(counts['decoder_products'],expected_products)
                    self.assertEqual(counts['decoder_additions'],len(view['terms']))
                    self.assertEqual(counts['required_prefix'],16)
                    self.assertFalse(hasattr(oracle,'expected_scalar'))
        finally:
            preserved_decoder.compile,preserved_decoder.Features=old_compile,old_features

    def test_binding_mutation_and_signed_value_contract(self):
        bound=adapter.bind();view=adapter.legacy_view(bound)
        expected=json.loads(json.dumps(view))
        view['terms'][0]['coefficient']=0;view['q']=1
        self.assertEqual(adapter.legacy_view(bound),expected)
        bad=bound.source_bytes+b' '
        with self.assertRaises(ValueError):adapter.bind(bad)
        calls=[]
        def oracle(a):calls.append(a);return 1
        with self.assertRaises(ValueError):adapter.decode(adapter.BoundProgram(bad),oracle)
        self.assertEqual(calls,[])
        for invalid in (True,478,1.0,None):
            with self.subTest(invalid=invalid),self.assertRaises(ValueError):
                adapter.decode(bound,lambda a:invalid)

    def test_saved_factor_reload_and_full_identity(self):
        factor,points,labels=load()
        coefficients=factor.solve(tuple((x-405*pow((x+15)%479,239,479))%479 for x in points))
        terms=[dict(offsets=list(S),coefficient=c) for S,c in zip(labels,coefficients) if c]
        terms.append(dict(offsets=[15],coefficient=405))
        program=dict(schema='public-character-polynomial/v1',r=479,terms=terms)
        self.assertEqual(program,json.loads((D/'program.json').read_text()))
        self.assertEqual(adapter.verify(program)['points_checked'],479)

if __name__=='__main__':unittest.main(verbosity=2)
