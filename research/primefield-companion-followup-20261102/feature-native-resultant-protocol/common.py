from pathlib import Path
import importlib.util,json,hashlib
D=Path(__file__).resolve().parent
PANEL=D.parent/'feature-native-resultant-panel'
def load(name,path):
 spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
def sha(path):return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def require(ok,why):
 if not ok:raise ValueError(why)
# Exact pinned upstream tool bytes; authority.json itself is bound by the published source.
authority=json.loads((D/'authority.json').read_text())
for entry in authority['files']:require(sha(D/'authority'/entry['path'])==entry['sha256'],'canonical authority changed')
canonical=load('canonical_protocol',D/'authority/tools/experiment_protocol.py')
analyzer=load('panel_analyzer',PANEL/'analyze.py')
def protocol(path,expected,frozen=True):
 p=analyzer.read_json(Path(path).read_text());errors=canonical.schema_errors(p,'experiment.protocol/v1')
 require(not errors,str(errors));errors=canonical.lint_protocol(p,Path(path).parent);require(not errors,str(errors))
 require(canonical.protocol_hash(p)==expected,'expected protocol hash mismatch')
 if frozen:require(p['freeze']['status']=='frozen' and p['freeze']['filed_before_affected_data']is True,'protocol not filed/frozen before data')
 return p
def bindings(p,base):
 for name,item in p['extensions']['bindings'].items():
  f=(base/item['path']).resolve();require(f.is_file()and sha(f)==item['sha256'],'binding changed: '+name)
 return {k:(base/v['path']).resolve()for k,v in p['extensions']['bindings'].items()}
