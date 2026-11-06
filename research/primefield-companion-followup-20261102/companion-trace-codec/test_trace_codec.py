"""Deterministic codec correctness tests, not a kernel experiment/benchmark."""
import hashlib
import unittest
from trace_codec import MAGIC, decode, encode, fingerprint


class CodecTests(unittest.TestCase):
    def test_round_trip_and_streamed_hash(self):
        values = [None, False, True, 0, -1, 127, 128, 255, 256,
                  (1 << 100000) + 17, -(1 << 100000), '', 'α\x00𝄞', [], {},
                  {'trace': [{'stage': 'column', 'P': (1 << 32000) + 1}], 'done': True}]
        for value in values:
            wire = encode(value)
            self.assertEqual(decode(wire), value)
            self.assertEqual(encode(decode(wire)), wire)
            self.assertEqual(fingerprint(value), dict(schema='companion-trace-fingerprint/v1',
                              sha256=hashlib.sha256(wire).hexdigest(), bytes=len(wire)))

    def test_golden_encodings(self):
        for value, body in [(None,b'n'),(False,b'f'),(True,b't'),(0,b'i\0\0'),
                            (256,b'i\0\2\1\0'),(-1,b'i\1\1\1'),('a',b's\1a'),
                            ([True,0],b'l\2ti\0\0')]:
            self.assertEqual(encode(value), MAGIC + body)

    def test_dictionary_order_is_canonical(self):
        self.assertEqual(encode({'b': 2, 'a': 1}), encode({'a': 1, 'b': 2}))

    def test_types_and_boundaries_are_distinct(self):
        values = [False,0,True,1,'1',None,[],{},[1,23],[12,3],{'a':1},{'a1':None}]
        self.assertEqual(len({encode(x) for x in values}), len(values))

    def test_every_truncation_is_rejected(self):
        wire = encode({'stage':'column','P':1 << 1024,'items':[True,None,-1]})
        for cut in range(len(wire)):
            with self.assertRaises((ValueError,UnicodeError)):
                decode(wire[:cut])
        with self.assertRaises(ValueError): decode(wire+b'n')

    def test_noncanonical_inputs_are_rejected(self):
        bodies=[b'i\1\0',b'i\2\1\1',b'i\0\1\0',b's\x80\0',b'x',
                b'd\2s\1bn s\1an'.replace(b' ',b''),
                b'd\2s\1ans\1an', b'd\1nn', b's\1\xff', b'l\x7f']
        for body in bodies:
            with self.assertRaises((ValueError,UnicodeError)): decode(MAGIC+body)

    def test_unsupported_values_are_rejected(self):
        for value in [(1,2),1.0,b'x',{1:'x'}]:
            with self.assertRaises(TypeError): fingerprint(value)

    def test_nesting_guard(self):
        value=0
        for _ in range(130): value=[value]
        with self.assertRaises(ValueError): encode(value)
        with self.assertRaises(ValueError): decode(MAGIC+b'l\1'*130+b'n')


if __name__ == '__main__': unittest.main()
