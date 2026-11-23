"""Metadata serialization correction for the repository's strict YAML subset."""
import hashlib
import json
from pathlib import Path
import yaml

class RegistryDumper(yaml.SafeDumper):
    def increase_indent(self, flow=False, indentless=False):
        return super().increase_indent(flow, False)

here=Path(__file__).resolve().parent
root=here.parents[1]
registry=root/'docs/curves/ic/curves.yaml'
original=(here/'crosswalk-before-registration.yaml').read_bytes()
previous=registry.read_bytes()
assert previous.startswith(original)
old=yaml.safe_load(original); current=yaml.safe_load(previous)
receipt=json.loads((here/'registration-verification.json').read_text())
names=[row['alias'] for row in receipt['records']]
assert len(names)==4
assert set(current['curves'])-set(old['curves'])==set(names)
assert all(current['curves'][name]==entry for name,entry in old['curves'].items())
new_entries={name:current['curves'][name] for name in names}
appendix='\n# Exact Fp4 factorial-constructor aliases; arithmetic and native checks are separate evidence.\n'
appendix+=''.join('  '+line+'\n' for line in yaml.dump(new_entries,Dumper=RegistryDumper,sort_keys=False,allow_unicode=True,width=1000000).splitlines())
final=original+appendix.encode()
assert yaml.safe_load(final)==current
registry.write_bytes(final)
correction={'schema':'campaign-fp4-yaml-correction/v1','reason':'Native strict YAML reader rejected indentless sequences and wrapped plain scalars before the model-identity skip; no trait digest was written.','old_entries_bytes_unchanged':True,'all_field_curve_hash_preimages_unchanged':True,'previous_registry_sha256':hashlib.sha256(previous).hexdigest(),'final_registry_sha256':hashlib.sha256(final).hexdigest(),'format':'Indented block sequences, width=1000000; only the four newly appended entries re-rendered.'}
with (here/'registration-yaml-correction.json').open('x') as out:
    json.dump(correction,out,indent=2);out.write('\n')
print(json.dumps(correction))
