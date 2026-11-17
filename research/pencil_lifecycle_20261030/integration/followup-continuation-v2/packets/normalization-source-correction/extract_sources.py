from pathlib import Path
import re,json,hashlib
D=Path(__file__).parent
repo=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029')
sources={
 'canonical':repo/'research/five_leads_e2e_20261019/native/genus2_model_gate.rs',
 'related-gate':Path('/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/src/bin/projective_law_gate.rs'),
 'state-collector':Path('/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/src/bin/projective_state_collector.rs')}
def digest(b):return hashlib.sha256(b).hexdigest()
def extract(text,name):
 matches=list(re.finditer(r'(?m)^(?:pub(?:\([^\n]*\))?\s+)?fn '+re.escape(name)+r'\s*\(',text))
 if not matches:return None
 assert len(matches)==1
 start=matches[0].start();brace=text.index('{',start);depth=1;end=brace+1
 while depth:
  depth+=(text[end]=='{')-(text[end]=='}');end+=1
 body=text[start:end]
 # These exact functions have no quoted strings/comments containing braces.
 assert not re.search(r'//|/\*|"',body)
 return {'start_line':text[:start].count('\n')+1,'end_line':text[:end].count('\n')+1,'sha256':digest(body.encode()),'text':body}
rows={}
for name,path in sources.items():
 data=path.read_bytes();text=data.decode();(D/(name+'.source.rs.txt')).write_bytes(data)
 funcs={f:extract(text,f) for f in ['choose','transform','map_divisor']}
 rows[name]={'path':str(path),'full_sha256':digest(data),'functions':funcs}
 for f,v in funcs.items():
  if v:(D/(name+'.'+f+'.rs.txt')).write_text(v['text']+'\n')
comparisons=[]
for name in ['related-gate','state-collector']:
 for fn in ['choose','transform','map_divisor']:
  base=rows['canonical']['functions'][fn];other=rows[name]['functions'][fn]
  if other is None:
   comparisons.append({'source':name,'function':fn,'present':False});continue
  replacements={}
  if name=='related-gate' and fn=='choose':replacements={'out':'x'}
  if name=='related-gate' and fn=='transform':replacements={'scale_inv':'inv'}
  if name=='state-collector' and fn=='map_divisor':replacements={'out':'result'}
  normalized=other['text']
  for a,b in replacements.items():normalized=re.sub(r'\b'+a+r'\b',b,normalized)
  comparisons.append({'source':name,'function':fn,'present':True,'byte_identical':other['text']==base['text'],'identifier_replacements':replacements,'equal_after_recorded_replacements':normalized==base['text'],'interpretation':'text comparison only; not compiler or independent semantic verification'})
base=repo/'research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets'
affected=[]
for label in ['coefficient-array-storage','coefficient-traversal','traversal-polynomial-composition','scale-trim-storage']:
 path=base/label/'source-transform.rs.txt';data=path.read_bytes()
 affected.append({'packet':label,'excerpt_sha256':digest(data),'excerpt_matches_related_gate_first61':data.decode()=='\n'.join(sources['related-gate'].read_text().splitlines()[:61])+'\n','named_functions_present':{f:extract(data.decode(),f) is not None for f in ['choose','transform','map_divisor']},'correction':'Use named canonical function excerpts and source map in this correction; preserve prior packet unchanged.'})
orig=json.loads(Path('/var/tmp/pencil-affine-coefficient-loop-20261031-v1/source-bindings.json').read_text())
result={'extraction_method':'unique named fn header, balanced braces; asserts no strings or comments in selected bodies','sources':rows,'comparisons':comparisons,'affected_packets':affected,'original_binding':orig,'canonical_full_hash_matches_original':rows['canonical']['full_sha256']==orig['loop_source_full_sha256'],'independent_source_fidelity_review':'pending'}
(D/'source-map.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'canonical_matches_original':result['canonical_full_hash_matches_original'],'comparisons':comparisons,'affected_packets':affected},indent=2))
