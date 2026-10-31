#!/usr/bin/env python3
"""Prospective exact COPY-only local image assembly; no implicit pull/retry."""
import argparse,hashlib,io,json,os,pathlib,re,signal,stat,subprocess,tarfile,tempfile,time,uuid
P=pathlib.Path;H=lambda b:hashlib.sha256(b).hexdigest()
ROOT=P('/home/ec2-user/crypto-pencil-signed-graph-20261029')
PARENT='public.ecr.aws/aws-cli/aws-cli@sha256:b94fc7612d131f1a3cb76145ba3385c813ad7561ef2c2b60c89fbd5dfee93d39'
PARENT_ID='sha256:f88bac9411086e0c87b0a1ff6a2d211897548182f86d5db16d02b9cfc38a6eaf'
OLD=P('/var/tmp/pencil-copy-only-assembly-execution-20261031-v1')
ADOPT_ID='sha256:347667d6ecc9294ab7bfcc83411fb47f7548c8d1ca5f7d315cca23fc4b115476'
OWNER='467e8502-46d8-4ab9-a13b-8d2375dd2ca2'
ARTIFACTS=P('/var/tmp/pencil-local-image-assembly-proposal-20261031-v1/bindings.json')
MEMBERS=P('/var/tmp/pencil-local-image-assembly-proposal-20261031-v1/runtime-members.json')
def require(x,msg):
 if not x:raise RuntimeError(msg)
def strict(b):
 def unique(ps):
  d={}
  for k,v in ps:require(k not in d,'duplicate JSON key');d[k]=v
  return d
 return json.loads(b,object_pairs_hook=unique)
def pinned(path,sha):
 p=P(path);require(p.is_absolute(),'absolute source path')
 for c in [p,*p.parents]:require(not c.is_symlink(),'symlink source path')
 st=p.stat();require(stat.S_ISREG(st.st_mode),'regular source');b=p.read_bytes();require(H(b)==sha,'source digest '+str(p));return b

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--binding',required=True);ap.add_argument('--receipts',required=True);a=ap.parse_args();out=P(a.receipts);require(out.is_absolute() and not out.exists(),'fresh receipt directory');out.mkdir(mode=0o700)
 start=time.monotonic();deadline=start+600;work_deadline=deadline-60;pending=None;seq=0;client_unknown=False;attempts=[];result={'status':'not_started','scientific_execution':False,'images':{},'attempts':attempts}
 def handler(s,f):
  nonlocal pending
  pending=s
 signal.signal(signal.SIGINT,handler);signal.signal(signal.SIGTERM,handler)
 try:
  binding_raw=P(a.binding).read_bytes();b=strict(binding_raw);require(b.get('admitted') is True,'not admitted');require(b['reviewed_supervisor_sha256']==H(P(__file__).read_bytes()),'supervisor review binding')
  for key in ['prospective_review_sha256','legacy_route_review_sha256','local_parent_resolution_review_sha256']:require(re.fullmatch('[0-9a-f]{64}',b.get(key,'')),'unfilled '+key)
  expected_paths=['receipt.json','binding.json','context-inventory.json','Dockerfile.ecbench','Dockerfile.runtime','005-build-base/receipt.json','006-uncertain-readback-base/inspection.json']
  require(set(b['recovery_sources'])==set(expected_paths),'recovery input inventory')
  retained={n:pinned(OLD/n,b['recovery_sources'][n]) for n in expected_paths}
  original=strict(retained['receipt.json']);original_binding=strict(retained['binding.json']);base_command=strict(retained['005-build-base/receipt.json'])
  require(original['owner']==OWNER and original['tags']=={phase:'pencil-local-'+phase+':'+OWNER for phase in ['base','runtime']},'original owner/tags')
  require(len(original['attempts'])==1 and original['attempts'][0]['phase']=='base' and original['attempts'][0]['attempt']==1,'sole original base attempt')
  require(original.get('client_unknown') is False and base_command['client_terminal_exit']==0 and base_command['client_unknown'] is False,'terminal base build evidence')
  require(original['attempts'][0]['iidfile']==str(OLD/'base.iid') and original['attempts'][0]['tag']==original['tags']['base'],'original attempt paths')
  for key,value in original_binding.items():
   if key not in ['reviewed_supervisor_sha256','prospective_review_sha256']:require(b.get(key)==value,'original binding changed '+key)
  docker=P(b['docker_client']);pinned(docker,b['docker_client_sha256'])
  artifacts=strict(pinned(ARTIFACTS,b['artifact_bindings_sha256']));members=strict(pinned(MEMBERS,b['runtime_members_sha256']))
  blobs={}
  for r in artifacts:blobs[r.get('name',r['path'])]=pinned(r['path'],r['sha256'])
  br=strict(pinned(b['final_build_receipt_path'],b['final_build_receipt_sha256']));revision=b['source_revision'];require(re.fullmatch('[0-9a-f]{40}',revision) and revision.startswith('3c592af'),'corrected source binding')
  require(br['status']=='built_static_candidate' and br['static_elf_checked'] is True and br['source_manifest']['commit']==revision,'accepted source/build receipt')
  binary=pinned(b['binary_path'],b['binary_sha256']);require(br['binary_sha256']==H(binary),'binary receipt join');require(br['source_manifest']['tree']==b['source_tree'],'source tree join')
  build=b['build_info'];require(build['source_revision']==revision and build['ecbench_sha256']==H(binary) and build['fresh_owner_sha256']==H(blobs['fresh_owner']),'build-info joins');require(build['source_repository']=='https://github.com/aburan28/crypto' and build['architecture']=='linux/amd64' and build['target']=='x86_64-unknown-linux-musl' and build['default_features'] is False,'build-info contract')
  for key in ['built_at','rustc','rustflags']:require(isinstance(build[key],str) and build[key],'build-info actual field')
  parent_receipt=strict(pinned(b['parent_receipt_path'],b['parent_receipt_sha256']));require(parent_receipt['status']=='acquired_verified' and parent_receipt['reference']==PARENT and parent_receipt['image']['image_id']==PARENT_ID,'acquired parent binding')
  df0=ROOT/'taskq/deploy/eks-karpenter/Dockerfile.ecbench';df1=ROOT/'taskq/deploy/eks-karpenter/resume/Dockerfile.runtime';dockerfiles=[blobs[str(df0)],blobs[str(df1)]]
  require(dockerfiles[0].decode().splitlines()[0]=='FROM '+PARENT,'base FROM pin')
  for text in dockerfiles:require(not re.search(rb'^\s*(RUN|ADD|ONBUILD)\b',text,re.M|re.I),'non-COPY recipe')
  context=out/'contexts';context.mkdir(mode=0o700);base=context/'base';overlay=context/'overlay';base.mkdir(mode=0o755);overlay.mkdir(mode=0o755);base.chmod(0o755);overlay.chmod(0o755)
  def put(path,data,mode):
   with path.open('xb') as f:f.write(data)
   path.chmod(mode);require(H(path.read_bytes())==H(data),'staged bytes changed')
  put(base/'ecbench',binary,0o755);put(base/'isolab-fresh-owner',blobs['fresh_owner'],0o755);put(base/'build-info.json',(json.dumps(build,sort_keys=True,indent=2)+'\n').encode(),0o644);put(base/'smoke-prime.json',blobs[str(ROOT/'taskq/deploy/eks-karpenter/l2-validation.json')],0o644)
  member_map={r['path']:r for r in members};require(len(member_map)==len(members),'duplicate member ledger');seen=set()
  with tarfile.open(fileobj=io.BytesIO(blobs['runtime_archive']),mode='r:gz') as tf:
   for m in tf:
    require(m.name in member_map and m.name not in seen and not P(m.name).is_absolute() and '..' not in P(m.name).parts,'archive allowlist');seen.add(m.name);r=member_map[m.name];dest=overlay/m.name
    require(m.isdir()==(r['kind']=='directory') and (m.isdir() or m.isfile()),'archive kind')
    require(m.mode==int(r['archive_mode'],8),'archive original mode')
    if m.isdir():dest.mkdir(mode=int(r['proposed_context_mode'],8));dest.chmod(int(r['proposed_context_mode'],8))
    else:
     data=tf.extractfile(m).read();require(len(data)==r['bytes'] and H(data)==r['sha256'],'archive file binding');put(dest,data,int(r['proposed_context_mode'],8))
  require(seen==set(member_map),'missing runtime member')
  # Verify checksums inside retained archive, in addition to independently bound member ledger.
  for line in (overlay/'runtime/SHA256SUMS').read_text().splitlines():
   sha,name=line.split(None,1);name=name.lstrip('*');require(not P(name).is_absolute() and '..' not in P(name).parts,'checksum path');require(H((overlay/'runtime'/name).read_bytes())==sha,'runtime SHA256SUMS')
  def snapshot():
   rows={}
   for directory in [base,overlay]:
    for p in directory.rglob('*'):
     require(not p.is_symlink(),'context symlink');st=p.stat();require(stat.S_ISREG(st.st_mode) or stat.S_ISDIR(st.st_mode),'context special');rows[str(p.relative_to(context))]={'kind':'directory' if p.is_dir() else 'file','mode':oct(stat.S_IMODE(st.st_mode)),'sha256':None if p.is_dir() else H(p.read_bytes())}
   return rows
  frozen=snapshot();require(frozen==strict(retained['context-inventory.json']),'full original context inventory mismatch');require(dockerfiles==[retained['Dockerfile.ecbench'],retained['Dockerfile.runtime']],'original recipes mismatch');(out/'context-inventory.json').write_text(json.dumps(frozen,indent=2)+'\n');(out/'binding.json').write_bytes(binding_raw)
  put(out/'Dockerfile.ecbench',dockerfiles[0],0o444);put(out/'Dockerfile.runtime',dockerfiles[1],0o444)
  config=out/'empty-docker-config';config.mkdir(mode=0o700);prefix=['/usr/bin/sudo','-n','/usr/bin/env','-i','PATH=/usr/bin:/bin','HOME='+str(config),'DOCKER_CONFIG='+str(config),'DOCKER_BUILDKIT=0',str(docker),'--host','unix:///var/run/docker.sock'];env={'PATH':'/usr/bin:/bin','HOME':str(config),'LANG':'C'}
  def run(label,argv,cap=15,inspect=False,readback=False,iid_read=False):
   nonlocal seq,client_unknown
   require(pending is None or readback,'deferred interruption');stop=deadline if readback else work_deadline;remaining=stop-time.monotonic();require(remaining>3,'deadline exhausted');cap=min(cap,remaining-2);seq+=1;d=out/f'{seq:03d}-{label}';d.mkdir();proc=None;t=time.monotonic();so=None;se=None;command=(['/usr/bin/sudo','-n','/usr/bin/cat']+argv) if iid_read else prefix+argv;require(not iid_read or (len(argv)==1 and argv[0] in [str(OLD/'base.iid'),str(out/'runtime.iid')]),'scoped IID command');receipt={'argv':command,'cap_seconds':cap}
   try:
    so=tempfile.TemporaryFile('w+b') if inspect else (d/'stdout').open('w+b');se=(d/'stderr').open('wb');client_unknown=True;proc=subprocess.Popen(command,env=env,stdout=so,stderr=se,start_new_session=True)
    while proc.poll() is None:
     require(pending is None or readback,'deferred interruption');require(time.monotonic()-t<cap,'client timeout');require(os.fstat(so.fileno()).st_size<16*1024**2 and os.fstat(se.fileno()).st_size<16*1024**2,'output cap');time.sleep(.04)
    so.seek(0);raw=so.read(16*1024**2+1);require(len(raw)<=16*1024**2,'output cap');obj=strict(raw) if inspect and proc.returncode==0 else None
    if obj is not None:
     redacted=json.loads(json.dumps(obj))
     for x in redacted:
      for section in ['Config','ContainerConfig']:
       if x.get(section,{}).get('Env') is not None:x[section]['Env']=[{'name':v.split('=',1)[0],'sha256':H(v.encode())} for v in x[section]['Env']]
     (d/'inspection.json').write_text(json.dumps(redacted,indent=2)+'\n')
    return proc.returncode,raw,(d/'stderr').read_bytes(),obj
   finally:
    try:
     if proc is not None and proc.poll() is None:
      try:os.killpg(proc.pid,signal.SIGTERM)
      except ProcessLookupError:pass
      try:proc.wait(timeout=1)
      except subprocess.TimeoutExpired:
       try:os.killpg(proc.pid,signal.SIGKILL)
       except ProcessLookupError:pass
       proc.wait(timeout=1)
     if proc is not None:client_unknown=proc.poll() is None;receipt['client_terminal_exit']=proc.poll()
    finally:
     if so is not None:so.close()
     if se is not None:se.close()
     receipt.update({'started_monotonic':t,'ended_monotonic':time.monotonic(),'client_unknown':client_unknown});(d/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
  rc,raw,err,_=run('daemon',['info','--format','{{.ID}}']);require(rc==0 and raw.decode().strip()==parent_receipt['daemon_id'],'same local daemon binding')
  rc,raw,err,parent=run('parent',['image','inspect',PARENT],inspect=True);require(rc==0 and len(parent)==1,'local parent absent');par=parent[0];require(par['Id']==PARENT_ID and PARENT in par.get('RepoDigests',[]) and par['Os']=='linux' and par['Architecture']=='amd64' and not par['Config'].get('OnBuild'),'parent resolution mismatch')
  def read_iid(path,label):
   for c in [path,*path.parents]:require(not c.is_symlink(),'IID symlink path')
   st=path.lstat();require(stat.S_ISREG(st.st_mode) and st.st_nlink==1 and st.st_uid==0 and stat.S_IMODE(st.st_mode)==0o600 and st.st_size in [71,72],'IID type/owner/mode/size')
   rc,raw,err,_=run(label,[str(path)],iid_read=True,readback=True)
   require(rc==0 and re.fullmatch(rb'sha256:[0-9a-f]{64}\n?',raw),'IID read/shape')
   after=path.lstat();require((st.st_dev,st.st_ino,st.st_size,st.st_mtime_ns)==(after.st_dev,after.st_ino,after.st_size,after.st_mtime_ns),'IID changed')
   return raw.decode().strip()
  nonce=OWNER;tags=original['tags'];result['owner']=nonce;result['tags']=tags
  expected_command=prefix[:];expected_command[5]='HOME='+str(OLD/'empty-docker-config');expected_command[6]='DOCKER_CONFIG='+str(OLD/'empty-docker-config')
  expected_command+=['build','--pull=false','--network=none','--platform=linux/amd64','--iidfile',str(OLD/'base.iid'),'--build-arg','SOURCE_REVISION='+revision,'--label','isolab.local-assembly.owner='+nonce,'--label','isolab.local-assembly.phase=base','--tag',tags['base'],'--file',str(OLD/'Dockerfile.ecbench'),str(OLD/'contexts/base')]
  require(base_command['argv']==expected_command,'original exact build command mismatch')
  require(read_iid(OLD/'base.iid','adopt-base-iid')==ADOPT_ID,'adopted IID mismatch')
  retained_base=strict(retained['006-uncertain-readback-base/inspection.json']);require(len(retained_base)==1,'retained base shape')
  for ref in [ADOPT_ID,tags['base']]:
   rc,raw,err,ims=run('adopt-base-readback',['image','inspect',ref],inspect=True)
   require(rc==0 and len(ims)==1,'adopt base readback');x=ims[0];cfg=x['Config'];lab=cfg.get('Labels',{})
   require(x['Id']==ADOPT_ID and x['Os']=='linux' and x['Architecture']=='amd64' and not cfg.get('OnBuild'),'adopt base identity')
   require(lab.get('isolab.local-assembly.owner')==OWNER and lab.get('isolab.local-assembly.phase')=='base' and lab.get('org.opencontainers.image.revision')==revision,'adopt labels')
   require(cfg['User']=='1000:1000' and not cfg.get('Entrypoint') and cfg.get('Cmd')==['/usr/local/bin/ecbench','--help'] and cfg.get('WorkingDir')=='/work','adopt config')
   require(x['RootFS']['Layers']==retained_base[0]['RootFS']['Layers'],'adopt layers')
  result['images']['base']={'id':ADOPT_ID,'tag':tags['base'],'layers':x['RootFS']['Layers'],'adopted':True,'original_command_exit':0}
  rc,raw,err,_=run('tag-absence-runtime',['image','inspect',tags['runtime']],inspect=True)
  require(rc!=0 and b'No such image' in err and tags['runtime'].encode() in err,'runtime tag unavailable')
  for phase,ctx,df in [('runtime',overlay,out/'Dockerfile.runtime')]:
   require(snapshot()==frozen,'context changed');require(time.monotonic()<work_deadline-3,'build budget exhausted');iid=out/(phase+'.iid');require(not iid.exists(),'IID already exists')
   parent_arg='SOURCE_REVISION='+revision if phase=='base' else 'BASE_IMAGE='+result['images']['base']['id']
   if phase=='runtime':
    rc,raw,err,local=run('base-parent-readback',['image','inspect',result['images']['base']['id']],inspect=True);require(rc==0 and local[0]['Id']==result['images']['base']['id'] and not local[0]['Config'].get('OnBuild'),'local base ID unavailable')
   attempts.append({'phase':phase,'tag':tags[phase],'iidfile':str(iid),'attempt':1,'status':'started'})
   rc,raw,err,_=run('build-'+phase,['build','--pull=false','--network=none','--platform=linux/amd64','--iidfile',str(iid),'--build-arg',parent_arg,'--label','isolab.local-assembly.owner='+nonce,'--label','isolab.local-assembly.phase='+phase,'--tag',tags[phase],'--file',str(df),str(ctx)],cap=240)
   require(rc==0,'build failed/uncertain');image_id=read_iid(iid,'runtime-iid');require(re.fullmatch('sha256:[0-9a-f]{64}',image_id),'IID format')
   rc,raw,err,ims=run('readback-'+phase,['image','inspect',tags[phase]],inspect=True,readback=True);require(rc==0 and len(ims)==1,'tag readback');x=ims[0]
   require(x['Id']==image_id and x['Os']=='linux' and x['Architecture']=='amd64' and not x['Config'].get('OnBuild'),'built image identity');labels=x['Config']['Labels'];require(labels.get('isolab.local-assembly.owner')==nonce and labels.get('isolab.local-assembly.phase')==phase and labels.get('org.opencontainers.image.revision')==revision,'built ownership/source labels')
   require(x['Config']['User']==('1000:1000' if phase=='base' else '0:0') and not x['Config'].get('Entrypoint'),'built recipe config');require(snapshot()==frozen,'context changed after build')
   require(x['RootFS']['Layers'][:len(result['images']['base']['layers'])]==result['images']['base']['layers'],'runtime base layer prefix');require(len(x['RootFS']['Layers'])>len(result['images']['base']['layers']),'runtime layer missing')
   result['images'][phase]={'id':image_id,'tag':tags[phase],'layers':x['RootFS']['Layers'],'user':x['Config']['User'],'entrypoint':x['Config'].get('Entrypoint'),'cmd':x['Config'].get('Cmd')};attempts[-1]['status']='built_readback_verified'
  result['status']='local_copy_only_images_built_pending_qualification'
 except BaseException as e:
  result.update({'status':'assembly_uncertain' if attempts or client_unknown else 'preparation_failed','error':type(e).__name__+': '+str(e),'client_unknown':client_unknown,'reconcile_exact_tags_and_iidfiles':True,'automatic_retry':False})
  if attempts and not client_unknown:
   result['reconciliation']=[]
   for attempt in attempts:
    if time.monotonic()>=deadline-3:break
    try:
     rc,raw,err,ims=run('uncertain-readback-'+attempt['phase'],['image','inspect',attempt['tag']],inspect=True,readback=True)
     observation={'phase':attempt['phase'],'tag':attempt['tag'],'inspection_exit':rc,'is_terminal_build_evidence':False}
     if rc==0 and len(ims)==1:
      labels=ims[0]['Config'].get('Labels',{});require(labels.get('isolab.local-assembly.owner')==result['owner'],'uncertain image ownership mismatch');observation['observed_image_id']=ims[0]['Id']
     result['reconciliation'].append(observation)
    except BaseException as reconcile_error:result['reconciliation'].append({'phase':attempt['phase'],'error':type(reconcile_error).__name__+': '+str(reconcile_error)});break
 finally:
  result.update({'elapsed_seconds':time.monotonic()-start,'cap_seconds':600,'remaining_image_qualification':True,'publication_performed':False,'implicit_pull_permitted':False});(out/'receipt.json').write_text(json.dumps(result,indent=2)+'\n')
 return 0 if result['status']=='local_copy_only_images_built_pending_qualification' else 1
if __name__=='__main__':raise SystemExit(main())
