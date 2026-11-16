"""Synthetic receipt tests only: these values are not cryptographic measurements."""
import copy
import json
import unittest
import analyze as a

HASH = 'a' * 64

def panel():
    rows = [dict(schema='native-monic-run/v1', protocol_sha256=HASH, stages=a.STAGES,
                 components=a.COMPONENTS, operations=list(a.OPS))]
    for phase, arm, target in a.schedule():
        c = [[[0]*17 for _ in range(8)] for _ in range(5)]
        if phase == 'query':
            c[3][0][1] = 1
            c[3][5][7] = 477
            c[3][3][0] = 9 if arm == 'candidate' else 10
            c[4][6][0] = 2
        else:
            c[1 if phase == 'setup' else 2][0][0] = 3
        rows.append(dict(schema='native-monic-row/v1', arm=arm, target=target, phase=phase,
                         scalar=target, verified=phase == 'query', counts=c))
    return rows

def raw(rows):
    return ('\n'.join(json.dumps(r) for r in rows)+'\n').encode()

class ParserTests(unittest.TestCase):
    def test_supported_and_primary_includes_verification(self):
        result = a.analyze(raw(panel()), HASH, 0)
        self.assertEqual(result['verdict'], 'counted_pareto_improvement')
        self.assertEqual(result['field_products_sum_delta'], -8)
        self.assertEqual(result['pairs'][0]['baseline']['field_products'], 12)
        self.assertEqual(result['query_rows'], 24)

    def test_equal_or_single_loss_prevents_support(self):
        for products in (10, 11):
            rows = panel()
            rows[8]['counts'][3][3][0] = products
            self.assertEqual(a.analyze(raw(rows), HASH, 0)['verdict'], 'counted_tradeoff_or_mixed')

    def test_structural_mutations_rejected(self):
        variants = []
        p = panel(); variants.append(p[:-1])
        p = panel(); variants.append(p+[p[-1]])
        p = panel(); p[7], p[8] = p[8], p[7]; variants.append(p)
        p = panel(); p[8] = copy.deepcopy(p[7]); variants.append(p)
        p = panel(); p[0]['operations'][0:0] = ['wrong']; variants.append(p)
        for key, value in [('scalar', 99), ('scalar', False), ('verified', False), ('target', False), ('schema', 'wrong'), ('extra', 1)]:
            p = panel(); p[7][key] = value; variants.append(p)
        p = panel(); p[8]['counts'].pop(); variants.append(p)
        p = panel(); p[8]['counts'][3].pop(); variants.append(p)
        p = panel(); p[8]['counts'][3][0].pop(); variants.append(p)
        for index, p in enumerate(variants):
            with self.subTest(mutation=index), self.assertRaises(a.Invalid):
                a.analyze(raw(p), HASH, 0)

    def test_counter_mutations_rejected(self):
        changes = [(3,3,0,v) for v in (-1, 2**64, True, 1.0)]
        changes += [(0,0,0,1), (1,0,0,1), (2,0,0,1), (4,0,0,1), (3,6,0,1),
                    (3,7,0,1), (3,0,10,1), (3,3,16,1), (3,0,1,0), (3,5,7,476), (3,5,0,1)]
        for s,i,o,v in changes:
            p = panel(); p[8]['counts'][s][i][o] = v
            with self.subTest(change=(s,i,o,v)), self.assertRaises(a.Invalid):
                a.analyze(raw(p), HASH, 0)
        for index in (3,6,9):
            p = panel(); phase = 1 if index == 3 else 2 if index == 6 else 3
            p[index]['counts'][phase][0][0] += 1
            with self.subTest(aa=index), self.assertRaises(a.Invalid):
                a.analyze(raw(p), HASH, 0)

    def test_inversion_tradeoff_model_and_regression(self):
        rows = panel()
        for row in rows[1:]:
            if row['phase']=='query':
                row['counts'][3][3][0]=11 if row['arm']=='candidate' else 10
                row['counts'][3][4][1]=0 if row['arm']=='candidate' else 2
        result=a.analyze(raw(rows),HASH,0)
        self.assertEqual(result['verdict'],'counted_tradeoff_or_mixed')
        self.assertEqual(result['pairs'][0]['delta']['field_inversions'],-2)
        model=result['pairs'][0]['weighted_model']
        self.assertEqual((model['numerator'],model['denominator']),(1,2))
        self.assertEqual(model['candidate_better'],'w > numerator/denominator')
        for row in rows[1:]:
            if row['phase']=='query':
                row['counts'][3][4][1]=3 if row['arm']=='candidate' else 2
        self.assertEqual(a.analyze(raw(rows),HASH,0)['verdict'],'counted_pareto_regression')

    def test_transport_and_metadata_rejected(self):
        value = raw(panel())
        bad = [value+b'\n', value.replace(b'"scalar": 0', b'"scalar": 0, "scalar": 0',1),
               value.replace(b'"scalar": 0', b'"scalar": NaN',1), b'\xff', b'{}\n']
        for data in bad:
            with self.subTest(data=data[:40]), self.assertRaises((a.Invalid, ValueError, UnicodeError)):
                a.analyze(data,HASH,0)
        for digest, status in [('b'*64,0), ('bad',0), (HASH,1), (HASH,124), (HASH,False)]:
            with self.subTest(metadata=(digest,status)), self.assertRaises(a.Invalid):
                a.analyze(value,digest,status)

if __name__ == '__main__':
    unittest.main()
