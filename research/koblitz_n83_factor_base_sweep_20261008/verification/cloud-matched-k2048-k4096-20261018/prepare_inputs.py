from pathlib import Path
import json,shutil,subprocess,hashlib
root=Path(__file__).resolve().parent;repo=Path('/home/ec2-user/crypto-n83-resume');study=repo/'research/koblitz_n83_factor_base_sweep_20261008';dest=root/'inputs';dest.mkdir()
fixtures=dest/'fixtures';fixtures.mkdir()
original=study/'verification/k2048-natural-highcap-20261018/evidence/frozen-input'
for name in ['probe-corpus.json','probe-validation.json']:shutil.copyfile(original/'fixtures'/name,fixtures/name)
rows=[]
for k,template in [(2048,original/'panel'),(4096,study/'verification/k4096-primary-20261018/evidence/construction-01/object')]:
 panel=dest/('k'+str(k));panel.mkdir();(panel/'objects').mkdir()
 for name in ['manifest.json','replay.json','upload-receipt.json']:shutil.copyfile(template/name,panel/name)
 shutil.copyfile(fixtures/'probe-corpus.json',panel/'probe-corpus.json')
 row=json.loads((panel/'manifest.json').read_text())['bases'][0]
 key='factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/v2-size-frontier/a0/'+row['object']
 outputs={}
 for op in ['head-object','get-object']:
  args=['aws','s3api',op,'--bucket','setec-astronomy-artifacts','--key',key,'--region','us-east-2','--output','json']
  if op=='get-object':args+=[str(panel/row['object'])]
  p=subprocess.run(args,check=True,capture_output=True,text=True,timeout=180);outputs[op]=json.loads(p.stdout)
  (dest/('k'+str(k)+'-'+op+'.json')).write_text(p.stdout)
 obj=panel/row['object'];sha=hashlib.sha256(obj.read_bytes()).hexdigest();b3=subprocess.check_output([str(root/'b3file'),str(obj)],text=True).strip()
 expected={2048:'aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a',4096:'ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad'}[k]
 assert sha==expected and b3==row['compressed_blake3'] and obj.stat().st_size==row['bytes']==outputs['head-object']['ContentLength']==outputs['get-object']['ContentLength']
 assert outputs['head-object']['ETag']==outputs['get-object']['ETag'] and outputs['head-object']['ServerSideEncryption']=='AES256'
 rows.append({'columns':k,'current_s3_uri':'s3://setec-astronomy-artifacts/'+key,'historical_manifest_s3_uri':row['s3_uri'],'bytes':obj.stat().st_size,'sha256':sha,'blake3':b3,'head_etag':outputs['head-object']['ETag'],'get_etag':outputs['get-object']['ETag'],'panel_sha256':{name:hashlib.sha256((panel/name).read_bytes()).hexdigest() for name in ['manifest.json','replay.json','upload-receipt.json','probe-corpus.json']}})
r={'schema':'n83.cloud-matched-fresh-input/v1','status':'PASS_FRESH_SETEC_INPUTS','provenance':'historical original-bucket sidecars preserved unchanged; actual current GETs use setec','bases':rows,'fixture_sha256':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in fixtures.iterdir()},'selected_best_total_runtime':None}
(dest/'input.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r))
