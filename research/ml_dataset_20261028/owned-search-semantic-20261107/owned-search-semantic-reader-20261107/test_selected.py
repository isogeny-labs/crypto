import io,tempfile,unittest
from pathlib import Path
from dataclasses import replace
from semantic_reader import inspect_selected,verify_and_extract,RetentionLimits,structural_summary
from inspect_wire import Invalid
from test_inspect_wire import wire,fixture,MAGIC
from test_capture import make,B,S

def decode(root,**kwargs):return inspect_selected([io.BytesIO(MAGIC+wire(root))],**kwargs)
class SelectedTests(unittest.TestCase):
    def test_exact_selected_subtrees(self):
        r=fixture();r['expansion']={'variables':8,'partial':[[1]],'scratch':[]};r['continuation']={'partitions':[{'search':{'nodes':[{'equations':[[[2]]],'native':{'basis':[{'polynomial':[[2]],'proof':[[0,[]]]}]}}]}}]}
        d=decode(r)
        self.assertEqual(d['retained'],{k:r[k] for k in ('expansion','normalized','continuation')})
        self.assertNotIn('identity',d['retained']);self.assertFalse(d['original_algebra_replayed'])
    def test_null_and_empty_are_distinct(self):
        r=fixture();r['continuation']=None;r['normalized']=[]
        self.assertIsNone(decode(r)['retained']['continuation']);self.assertEqual(decode(r)['retained']['normalized'],[])
    def test_nested_raw_bytes(self):
        r=fixture();r['continuation']={'audit':{'raw':bytes(70000)}}
        self.assertEqual(decode(r)['retained']['continuation'],r['continuation'])
    def test_retained_nodes_refuse(self):
        with self.assertRaises(Invalid):decode(fixture(),retention=RetentionLimits(nodes=2))
    def test_retained_allocation_refuse(self):
        with self.assertRaises(Invalid):decode(fixture(),retention=RetentionLimits(allocation_units=500))
    def test_raw_declared_length_refuses_without_body(self):
        from test_inspect_wire import u
        r=fixture();data=MAGIC+wire(r);needle=wire('continuation')+b'\0';bad=data.replace(needle,wire('continuation')+b'\4'+u(2**63))
        with self.assertRaises(Invalid):inspect_selected([io.BytesIO(bad)])
    def test_retention_independent_chunking(self):
        r=fixture();data=MAGIC+wire(r);a=decode(r);b=inspect_selected([io.BytesIO(bytes([x])) for x in data]);self.assertEqual(a,b)
    def test_capture_binding_and_selected_data(self):
        with tempfile.TemporaryDirectory() as tmp:
            make(Path(tmp));r=verify_and_extract(tmp,B,S)
            self.assertEqual(len(r['snapshots']),1);self.assertTrue(r['capture']['committed_prefix_verified']);self.assertEqual(r['snapshots'][0]['retained']['normalized'],fixture()['normalized']);self.assertFalse(r['original_algebra_replayed'])
    def test_capture_retention_cap(self):
        with tempfile.TemporaryDirectory() as tmp:
            make(Path(tmp))
            with self.assertRaises(Invalid):verify_and_extract(tmp,B,S,retention=RetentionLimits(nodes=2))
    def test_invalid_retention_limits(self):
        for v in [RetentionLimits(nodes=True),RetentionLimits(nodes=0),RetentionLimits(nodes=1000001),RetentionLimits(allocation_units=2**30)]:
            with self.assertRaises(Invalid):decode(fixture(),retention=v)
    def test_cli_summary_does_not_export_full_objects(self):
        r=decode(fixture());s=structural_summary(r);self.assertNotIn('retained',s);self.assertEqual(s['selected_summary']['normalized_equations'],2)
if __name__=='__main__':unittest.main()
