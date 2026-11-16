"""Independent exact generic public projection for this fixed Legacy fixture."""
import hashlib,json

def req(v,msg):
 if not v:raise RuntimeError(msg)
def integer(v):req(type(v) is int and v>=0,'public nonnegative integer');return v
def text(v):req(type(v) is str,'public string');return v
def boolean(v):req(type(v) is bool,'public boolean');return v
def decimal(v):req(type(v) is str and v and all('0'<=x<='9' for x in v),'decimal string');return str(int(v))
def point(v):return None if v is None else {'x':decimal(v['x']),'y':decimal(v['y'])}
def array(v):req(type(v) is list,'array');return v
def points(v):return [point(x) for x in array(v)]
def action(v):return [None if x is None else integer(x) for x in array(v)]
def projection(p,b):
 req(p['schema']=='ml-ecdlp-instance/v1' and b['schema'] in ['ml-ecdlp-factor-base/v1','ml-ecdlp-factor-base/v2'],'input schema')
 f=p['field'];field={'characteristic':decimal(f['characteristic']),'degree':integer(f['degree']),'basis':text(f['basis'])}
 if 'modulus' in f:field['modulus']=decimal(f['modulus'])
 if 'irreducible_low_terms' in f:field['irreducible_low_terms']=[integer(x) for x in array(f['irreducible_low_terms'])]
 c=p['curve'];curve={'model':text(c['model']),**{k:decimal(c[k]) for k in ['a','b','group_order','subgroup_order','cofactor']}}
 s=p['symmetry'];sym={k:integer(s[k]) for k in ['coefficient_field_degree','frobenius_power']};sym.update({k:boolean(s[k]) for k in ['frobenius_nontrivial_on_model','negation']})
 base={'rule':text(b['rule']),'points':points(b['points']),'negation_action':action(b['negation_action']),'frobenius_action':action(b['frobenius_action']),'frobenius_power':integer(b['frobenius_power']),'orbits':[{'points':points(o['points']),'closed':boolean(o['closed']),'stabilizer_size':integer(o['stabilizer_size'])} for o in array(b['orbits'])]}
 out={'pdp_domain':'cyclic-subgroup-factor-base','family':text(p['family']),'n':integer(p['n']),'m':integer(p['m']),'twist':boolean(p['twist']),'field':field,'curve':curve,'generator':point(p['generator']),'target':point(p['target']),'symmetry':sym,'factor_base':base}
 if b['schema']=='ml-ecdlp-factor-base/v2':
  req(b['policy']=={'kind':'legacy'},'fixed Legacy policy');base['policy']={'kind':'legacy'};out['source_factor_base_policy']={'kind':'legacy'}
 return out

def canonical(v):return json.dumps(v,sort_keys=True,separators=(',',':'),ensure_ascii=False)
def unique(pairs):
 out={}
 for key,value in pairs:
  req(key not in out,'duplicate public JSON key');out[key]=value
 return out

def check(root,load):
 export=root/'export';public=set()
 for p in export.rglob('*'):
  req(not p.is_symlink(),'export symlink')
  if p.is_file() and 'restricted' not in p.relative_to(export).parts:public.add(p.relative_to(export).as_posix())
 req(public=={'manifest.json','features.jsonl','prefix-features.jsonl','algebraic-features.jsonl'},'unexpected public file set')
 for n in ['prefix-features.jsonl','algebraic-features.jsonl']:req((export/n).stat().st_size==0,'unsupported consequence feature stream')
 expected={}
 component='field-size-n23';bucket=int(hashlib.sha256(('ml-dataset-whole-field-size-split-v1\n'+component).encode()).hexdigest()[:16],16)%100;split='train' if bucket<80 else 'validation' if bucket<90 else 'test'
 for cell in sorted((root/'run/cells').glob('*/manifest.json')):
  inputs=projection(load(cell.parent/'public.json'),load(cell.parent/'factor-base.json'))
  identity=hashlib.sha256(canonical(inputs).encode()).hexdigest();expected[identity]={'schema':'ml-dataset.features/v1','feature_id':identity,'split_component':component,'split':split,'inputs':inputs}
 actual={}
 for line in (export/'features.jsonl').read_text().splitlines():
  row=json.loads(line,object_pairs_hook=unique);req(row['feature_id'] not in actual,'duplicate public feature');actual[row['feature_id']]=row
 req(canonical(actual)==canonical(expected),'public projection differs from independent structural allowlist')
