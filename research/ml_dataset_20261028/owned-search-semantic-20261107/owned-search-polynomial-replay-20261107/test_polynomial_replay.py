import unittest
from dataclasses import replace
from polynomial_replay import Limits, verify_row, verify_field_row

def row(p, proof): return {'polynomial': p, 'proof': proof}

class ReplayTests(unittest.TestCase):
    def status(self, expected, n, g, r, limits=Limits()):
        result = verify_row(n, g, r, limits)
        self.assertEqual(result['status'], expected, result)
        self.assertFalse(result['source_authenticated'])
        self.assertFalse(result['fall_degree_certified'])
        return result

    def test_original(self):
        self.status('verified', 2, [[[], [0,1]]], row([[], [0,1]], [[0, []]]))

    def test_boolean_idempotence_and_cancellation(self):
        self.status('verified', 2, [[[], [0]]], row([], [[0, [0]]]))
        self.status('verified', 2, [[[0,1]]], row([[0,1]], [[0, [0,1]]]))

    def test_distinct_proof_terms_cancel_under_boolean_quotient(self):
        self.status('verified', 2, [[[0]]], row([], [[0, []], [0, [0]]]))

    def test_multiple_generators(self):
        self.status('verified', 2, [[[],[0]], [[0],[1]]], row([[],[1]], [[0,[]],[1,[]]]))

    def test_constant_one_requires_valid_identity(self):
        self.status('mismatch', 1, [[[0]]], row([[]], [[0,[]]]))
        self.status('verified', 1, [[[],[0]], [[0]]], row([[]], [[0,[]],[1,[]]]))

    def test_mutations_fail(self):
        for r in [row([[1]], [[0,[]]]), row([[0]], []), row([[0]], [[0,[1]]])]:
            self.status('mismatch', 2, [[[0]]], r)

    def test_generator_binding_and_extra_field_generator_rejected(self):
        self.status('invalid', 2, [[[0]]], row([], [[1,[]]]))
        self.status('mismatch', 2, [[[1]]], row([[0]], [[0,[]]]))

    def test_noncanonical_monomials(self):
        for mono in [[0,0], [1,0], [-1], [2], [True], [1.0]]:
            self.status('invalid', 2, [[mono]], row([], []))

    def test_degreelex_not_lex_and_duplicate_terms(self):
        self.status('verified', 3, [[[2],[0,1]]], row([[2],[0,1]], [[0,[]]]))
        for p in [[[0,1],[2]], [[0],[0]]]:
            self.status('invalid', 3, [p], row([], []))

    def test_proof_order_is_lex_not_degreelex(self):
        self.status('verified', 3, [[]], row([], [[0,[0,1]],[0,[2]]]))
        self.status('invalid', 3, [[]], row([], [[0,[2]],[0,[0,1]]]))
        self.status('invalid', 1, [[]], row([], [[0,[]],[0,[]]]))

    def test_zero_variables_zero_polynomial(self):
        self.status('verified', 0, [], row([], []))
        self.status('verified', 0, [[[]]], row([[]], [[0,[]]]))

    def test_strict_types_and_shapes(self):
        for n in [True, -1, 1.0, 2**32]: self.status('invalid', n, [], row([], []))
        for r in [{}, {'polynomial': [], 'proof': [], 'extra': 1}, row([], [[True,[]]]), row((),[])]:
            self.status('invalid', 2, [[]], r)

    def test_each_resource_cap_is_inconclusive(self):
        n,g,r=2,[[[0],[1]]],row([[0],[1]],[[0,[]]])
        for name,value in [('variables',1),('generators',0),('terms',1),('degree',0),('proof',0),('work',0),('allocation_units',0)]:
            self.status('inconclusive',n,g,r,replace(Limits(),**{name:value}))

    def test_limits_invalid(self):
        self.status('invalid', 0, [], row([],[]), replace(Limits(),work=-1))
        self.status('invalid', 0, [], row([],[]), replace(Limits(),work=True))
        self.status('invalid', 0, [], row([],[]), replace(Limits(),degree=10**20))

    def test_field_task_multiplies_in_boolean_ring(self):
        g=[[[],[0]]];base=row([[],[0]],[[0,[]]]);derived=row([],[[0,[0]]])
        result=verify_field_row(1,g,base,0,derived)
        self.assertEqual(result['status'],'verified');self.assertTrue(result['field_task_checked'])
        # A valid generator derivation still need not be this Field task.
        self.assertEqual(verify_field_row(1,g,base,0,base)['status'],'mismatch')
        self.assertEqual(verify_field_row(1,g,base,1,derived)['status'],'invalid')
        self.assertEqual(verify_field_row(2,g,base,1,row([[1],[0,1]],[[0,[1]]]))['status'],'invalid')

    def test_independent_truth_table_oracle(self):
        # Exhaust all canonical polynomials and multipliers in 3 variables.
        monomials=sorted([tuple(i for i in range(3) if mask>>i&1) for mask in range(8)],key=lambda x:(len(x),x))
        for mask in range(256):
            g=[list(m) for i,m in enumerate(monomials) if mask>>i&1]
            for multiplier in monomials:
                acc=set()
                for t in g:
                    product=tuple(sorted(set(t)|set(multiplier)))
                    if product in acc:acc.remove(product)
                    else:acc.add(product)
                p=[list(t) for t in sorted(acc,key=lambda x:(len(x),x))]
                self.status('verified',3,[g],row(p,[[0,list(multiplier)]]))
                for assignment in range(8):
                    evaluate=lambda poly:sum(all(assignment>>v&1 for v in t) for t in poly)%2
                    self.assertEqual(evaluate(p),evaluate(g)*all(assignment>>v&1 for v in multiplier))

if __name__=='__main__': unittest.main()
