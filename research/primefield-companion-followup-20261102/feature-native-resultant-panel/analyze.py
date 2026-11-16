"""Strict finite-panel analysis. No crypto evaluation. CLI requires external receipt pins."""
from pathlib import Path
import argparse,hashlib,json,csv,io
ARMS=['scalar','euclid','half','aa','positive']
def require(test,message):
 if not test:raise ValueError(message)
def digest(b):return hashlib.sha256(b).hexdigest()
def integer(x):return type(x)is int and 0<=x<2**64
def unique(pairs):
 d={}
 for k,v in pairs:
  require(k not in d,'duplicate JSON key');d[k]=v
 return d
def read_json(text):return json.loads(text,object_pairs_hook=unique,parse_constant=lambda x:(_ for _ in ()).throw(ValueError('nonfinite JSON')))
def fixtures(text):
 reader=csv.DictReader(io.StringIO(text),delimiter='\t');require(reader.fieldnames==['case_id','p','kind','n','m','roots','points','expected'],'input columns')
 rows=[]
 for r in reader:
  require(None not in r,'extra input columns')
  for k in ['case_id','p','n','m','expected']:r[k]=int(r[k]);require(integer(r[k]),'input integer')
  for k,size in [('roots',r['n']),('points',r['m'])]:
   values=[int(x)for x in r[k].split(',')];require(len(values)==size and len(set(values))==size and all(0<=x<r['p']for x in values),'input dimensions or canonical values');r[k]=values
  require(r['case_id']==len(rows),'input id order');require(r['expected']<r['p'],'oracle range');rows.append(r)
 expected=[(p,n,m,kind)for p in [509,1000003]for n in [16,32,64,128]for m in [n//2,n,2*n]for kind in ['disjoint','common_root']]
 require([(r['p'],r['n'],r['m'],r['kind'])for r in rows]==expected,'wrong panel dimensions')
 for r in rows:require(len(set(r['roots'])&set(r['points']))==(r['kind']=='common_root'),'common root classification')
 return rows
def validate_rows(rows,inputs,names,protocol_hash,input_hash):
 require(len(names)==len(set(names)) and 'Product'in names,'metric inventory')
 require(len(rows)==len(inputs)*5,'incomplete or oversized panel')
 columns={'case_id','arm','ordinal','p','kind','n','m','value','expected','protocol_sha256','input_sha256','metrics'}
 groups={}
 for ordinal,row in enumerate(rows):
  require(set(row)==columns,'row columns');case=inputs[ordinal//5];arm=ARMS[(ordinal%5+case['case_id'])%5]
  for k in ['case_id','p','n','m','value','expected','ordinal']:require(integer(row[k]),'row integer')
  require(row['ordinal']==ordinal and row['arm']==arm,'row order or arm mismatch')
  require(all(row[k]==case[k]for k in ['case_id','p','kind','n','m','expected']),'row/input mismatch')
  require(row['value']==case['expected'],'oracle mismatch')
  require(row['protocol_sha256']==protocol_hash and row['input_sha256']==input_hash,'row custody mismatch')
  m=row['metrics'];require(type(m)is dict and set(m)==set(names),'metric columns')
  require(all(integer(x)for x in m.values()),'metric integer/range')
  require(all(m[k]==0 for k in ['live_bytes','allocation_failures','accounting_invalid','counter_overflow']),'invalid accounting interval')
  require(m['Projection']==1,'projection boundary')
  groups.setdefault(case['case_id'],{})[arm]=m
 result=[]
 for c in inputs:
  g=groups[c['case_id']];base=g['scalar'];require(g['aa']==base,'A/A disagreement')
  require(all(g['positive'][k]==base[k]+(k=='Product')for k in names),'positive control disagreement')
  result.append({'case_id':c['case_id'],'kind':c['kind'],'metrics':g,'deltas_vs_scalar':{a:{k:g[a][k]-base[k]for k in names}for a in ['euclid','half']}})
 return {'scope':'finite pinned polynomial panel; separate dimensions, no weighted total or timing inference','cases':len(inputs),'rows':len(rows),'controls_passed':True,'pairs':result}
def main():
 p=argparse.ArgumentParser();
 for x in ['rows','inputs','metrics','receipt','expected-protocol','expected-input','expected-binary','output']:p.add_argument('--'+x,required=True)
 a=p.parse_args();raw=Path(a.rows).read_bytes();inp=Path(a.inputs).read_bytes();receipt=read_json(Path(a.receipt).read_text())
 require(type(receipt.get('exit_code'))is int and receipt['exit_code']==0,'producer did not finish successfully')
 require(receipt.get('protocol_sha256')==a.expected_protocol and receipt.get('input_sha256')==a.expected_input and receipt.get('binary_sha256')==a.expected_binary,'receipt pins mismatch')
 require(receipt.get('raw_sha256')==digest(raw) and digest(inp)==a.expected_input,'raw/input digest mismatch')
 metric_bytes=Path(a.metrics).read_bytes();require(receipt.get('metrics_sha256')==digest(metric_bytes),'metric inventory digest mismatch')
 for v in [a.expected_protocol,a.expected_input,a.expected_binary]:require(len(v)==64 and all(c in '0123456789abcdef'for c in v),'invalid expected digest')
 names=metric_bytes.decode().splitlines();rows=[read_json(line)for line in raw.decode().splitlines()]
 result=validate_rows(rows,fixtures(inp.decode()),names,a.expected_protocol,a.expected_input)
 result['custody']={'protocol_sha256':a.expected_protocol,'input_sha256':a.expected_input,'binary_sha256':a.expected_binary,'raw_sha256':digest(raw),'receipt_sha256':digest(Path(a.receipt).read_bytes())}
 with Path(a.output).open('x')as f:json.dump(result,f,indent=2);f.write('\n')
if __name__=='__main__':main()
