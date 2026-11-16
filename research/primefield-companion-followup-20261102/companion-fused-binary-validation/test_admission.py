"""Publication/owner metadata checks with synthetic bytes, no network or kernels."""
from pathlib import Path
import copy
import hashlib
import json
import tempfile
import time
import unittest
import launch
from admission import published


class AdmissionTests(unittest.TestCase):
    def fixture(self, root):
        source=b'print(1)\n';plan=b'[]\n';protocol=b'{"frozen":"fixture"}'
        (root/'source.py').write_bytes(source);(root/'plan.json').write_bytes(plan)
        sources={n:hashlib.sha256((root/n).read_bytes()).hexdigest() for n in ['source.py','plan.json']}
        inputs={'input.jsonl':'0'*64}
        remote={'sources.json':json.dumps(sources).encode(),'inputs.json':json.dumps(inputs).encode(),
                'protocol.json':protocol,'source.py':source,
                '.artifacts.json':json.dumps({'schema':'artifacts.manifest/v1','files':{
                    'plan.json':{'sha256':sources['plan.json'],'bytes':len(plan)}}}).encode()}
        publication=dict(repository='isogeny-labs/crypto',commit='a'*40,
            source_manifest_path='sources.json',input_manifest_path='inputs.json',protocol_path='protocol.json',
            files={'source.py':{'path':'source.py'},'plan.json':{'manifest_path':'.artifacts.json','artifact':'plan.json'}})
        return publication,sources,inputs,protocol,remote

    def test_exact_publication(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d);p,s,i,b,remote=self.fixture(root)
            result=published(p,s,i,b,root,time.monotonic()+5,fetch=remote.__getitem__)
            self.assertEqual(result['verified_source_entries'],2)
            self.assertEqual(result['verified_input_manifest_entries'],1)

    def test_changed_source_protocol_and_manifests_rejected(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d);p,s,i,b,remote=self.fixture(root)
            for key in ['source.py','sources.json','inputs.json','protocol.json','.artifacts.json']:
                changed=dict(remote)
                changed[key]=b'{}' if key.endswith('.json') else b'changed'
                with self.assertRaises((ValueError,KeyError)):
                    published(p,s,i,b,root,time.monotonic()+5,fetch=changed.__getitem__)

    def test_immutable_commit_scope_and_script_publication(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d);p,s,i,b,remote=self.fixture(root)
            variants=[]
            q=copy.deepcopy(p);q['commit']='main';variants.append(q)
            q=copy.deepcopy(p);del q['files']['plan.json'];variants.append(q)
            q=copy.deepcopy(p);q['files']['source.py']={'manifest_path':'.artifacts.json','artifact':'source.py'};variants.append(q)
            q=copy.deepcopy(p);q['files']['source.py']={'path':'../source.py'};variants.append(q)
            for q in variants:
                with self.assertRaises(ValueError):published(q,s,i,b,root,time.monotonic()+5,fetch=remote.__getitem__)

    def test_control_receipt_is_exclusive_and_bounded(self):
        old=launch.D
        try:
            with tempfile.TemporaryDirectory() as d:
                launch.D=Path(d)
                launch.control('launch.json',{'owner':1})
                with self.assertRaises(FileExistsError):launch.control('launch.json',{'owner':2})
                with self.assertRaises(ValueError):launch.control('too-large.json',{'data':'x'*65536})
                self.assertFalse((launch.D/'too-large.json').exists())
                self.assertEqual(json.loads((launch.D/'launch.json').read_text()),{'owner':1})
        finally:launch.D=old


if __name__=='__main__':unittest.main()
