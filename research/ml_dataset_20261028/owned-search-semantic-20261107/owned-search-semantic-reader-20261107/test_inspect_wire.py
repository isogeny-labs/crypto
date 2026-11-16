import io
import struct
import unittest
from dataclasses import replace
from inspect_wire import inspect, Limits, Invalid, MAGIC, ROOT, IDENTITY, SCHEMA

def u(n): return struct.pack('<Q', n)
def wire(x):
    if x is None: return b'\0'
    if isinstance(x, bool): return b'\1' + bytes([x])
    if isinstance(x, int): return b'\2' + u(x)
    if isinstance(x, str):
        b = x.encode(); return b'\3' + u(len(b)) + b
    if isinstance(x, bytes): return b'\4' + u(len(x)) + x
    if isinstance(x, list): return b'\5' + u(len(x)) + b''.join(map(wire,x))
    return b'\6' + u(len(x)) + b''.join(wire(k)+wire(v) for k,v in x.items())
def fixture():
    identity = {k:bytes(range(32)) if k.endswith('_sha256') else None for k in IDENTITY}
    identity.update(map_bytes=b'\x00\xff'*100, publication_attempted=True, map_published=True)
    root = dict.fromkeys(ROOT)
    root.update(schema=SCHEMA, snapshot_index=3, identity=identity,
                normalized=[[[0,1],[]],[]], status='Incomplete', encoding_complete=True)
    return root

def parse(data, limits=Limits()): return inspect([io.BytesIO(data)], limits)
class WireTests(unittest.TestCase):
    def test_roundtrip_identity_counts_and_no_semantic_claim(self):
        data=MAGIC+wire(fixture());r=parse(data)
        self.assertEqual(r['bytes'],len(data));self.assertEqual(r['snapshot_index'],3)
        self.assertEqual(r['source_hashes']['public_sha256'],bytes(range(32)).hex())
        self.assertFalse(r['mathematical_replay']);self.assertFalse(r['capture_chain_verified'])
    def test_every_single_byte_chunk_boundary(self):
        data=MAGIC+wire(fixture());a=parse(data)
        b=inspect([io.BytesIO(bytes([x])) for x in data]);self.assertEqual(a,b)
    def test_every_truncated_prefix_rejected(self):
        data=MAGIC+wire(fixture())
        for end in range(len(data)):
            with self.assertRaises(Invalid):parse(data[:end])
    def test_trailing_byte_and_exact_cap(self):
        data=MAGIC+wire(fixture());parse(data,replace(Limits(),bytes=len(data)))
        with self.assertRaises(Invalid):parse(data+b'\0',replace(Limits(),bytes=len(data)))
        with self.assertRaises(Invalid):parse(data,replace(Limits(),bytes=len(data)-1))
    def test_reordered_root_and_wrong_schema_rejected(self):
        r=fixture();r['schema']='other/v1'
        with self.assertRaises(Invalid):parse(MAGIC+wire(r))
        r=fixture();r={k:r[k] for k in reversed(ROOT)}
        with self.assertRaises(Invalid):parse(MAGIC+wire(r))
    def test_false_sentinel_and_hash_size_type(self):
        for val in [False,None,1,'true']:
            r=fixture();r['encoding_complete']=val
            with self.assertRaises(Invalid):parse(MAGIC+wire(r))
        for val in [b'a'*31,'a'*32,None]:
            r=fixture();r['identity']['public_sha256']=val
            with self.assertRaises(Invalid):parse(MAGIC+wire(r))
    def test_nested_duplicate_key_and_unknown_tag(self):
        data=MAGIC+wire(fixture());old=wire('continuation')+b'\0'
        bad=wire('continuation')+b'\6'+u(2)+wire('x')+b'\0'+wire('x')+b'\0'
        with self.assertRaises(Invalid):parse(data.replace(old,bad))
        with self.assertRaises(Invalid):parse(data.replace(old,wire('continuation')+b'\xff'))
    def test_declared_lengths_refuse_before_body_read(self):
        for tag in [4,5,6]:
            data=MAGIC+b'\6'+u(len(ROOT))+wire('schema')+wire(SCHEMA)+wire('snapshot_index')+wire(3)+wire('identity')+bytes([tag])+u(2**64-1)
            with self.assertRaises(Invalid):parse(data)
    def test_depth_value_string_and_invalid_utf8(self):
        r=fixture();nested=None
        for _ in range(15):nested=[nested]
        r['continuation']=nested;data=MAGIC+wire(r)
        with self.assertRaises(Invalid):parse(data,replace(Limits(),depth=8))
        with self.assertRaises(Invalid):parse(data,replace(Limits(),values=20))
        with self.assertRaises(Invalid):parse(data,replace(Limits(),string_bytes=5))
        with self.assertRaises(Invalid):parse(data.replace(wire('Incomplete'),b'\3'+u(1)+b'\xff'))
    def test_wrong_magic_root_and_boolean(self):
        for data in [b'bad'+wire(fixture()),MAGIC+b'\0',MAGIC+b'\6'+u(len(ROOT))+wire('schema')+b'\1\2']:
            with self.assertRaises(Invalid):parse(data)
    def test_invalid_limit_configuration(self):
        for limits in [replace(Limits(),depth=129),replace(Limits(),values=0),replace(Limits(),string_bytes=65537)]:
            with self.assertRaises(Invalid):parse(MAGIC+wire(fixture()),limits)
if __name__=='__main__':unittest.main()
