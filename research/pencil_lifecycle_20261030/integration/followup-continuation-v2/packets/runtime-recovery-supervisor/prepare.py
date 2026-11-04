from pathlib import Path
import hashlib,json,difflib,ast
D=Path(__file__).parent;O=Path('/var/tmp/pencil-copy-only-assembly-supervisor-20261031-v1/assemble.py');s=O.read_text();old=s
s=s.replace("ARTIFACTS=P(","OLD=P('/var/tmp/pencil-copy-only-assembly-execution-20261031-v1')\nADOPT_ID='sha256:347667d6ecc9294ab7bfcc83411fb47f7548c8d1ca5f7d315cca23fc4b115476'\nOWNER='467e8502-46d8-4ab9-a13b-8d2375dd2ca2'\nARTIFACTS=P(",1)
s=s.replace("  binding_raw=P(a.binding).read_bytes();b=strict(binding_raw);", "  binding_raw=P(a.binding).read_bytes();b=strict(binding_raw);")
needle="  docker=P(b['docker_client']);pinned(docker,b['docker_client_sha256'])"
replacement="""  expected_paths=['receipt.json','binding.json','context-inventory.json','Dockerfile.ecbench','Dockerfile.runtime','005-build-base/receipt.json','006-uncertain-readback-base/inspection.json']
  require(set(b['recovery_sources'])==set(expected_paths),'recovery input inventory')
  retained={n:pinned(OLD/n,b['recovery_sources'][n]) for n in expected_paths}
  original=strict(retained['receipt.json']);original_binding=strict(retained['binding.json']);base_command=strict(retained['005-build-base/receipt.json'])
  require(original['owner']==OWNER and original['tags']=={phase:'pencil-local-'+phase+':'+OWNER for phase in ['base','runtime']},'original owner/tags')
  require(len(original['attempts'])==1 and original['attempts'][0]['phase']=='base' and original['attempts'][0]['attempt']==1,'sole original base attempt')
  require(original.get('client_unknown') is False and base_command['client_terminal_exit']==0 and base_command['client_unknown'] is False,'terminal base build evidence')
  require(original['attempts'][0]['iidfile']==str(OLD/'base.iid') and original['attempts'][0]['tag']==original['tags']['base'],'original attempt paths')
  for key,value in original_binding.items():
   if key not in ['reviewed_supervisor_sha256','prospective_review_sha256']:require(b.get(key)==value,'original binding changed '+key)
  docker=P(b['docker_client']);pinned(docker,b['docker_client_sha256'])"""
assert needle in s;s=s.replace(needle,replacement)
s=s.replace("  frozen=snapshot();", "  frozen=snapshot();require(frozen==strict(retained['context-inventory.json']),'full original context inventory mismatch');require(dockerfiles==[retained['Dockerfile.ecbench'],retained['Dockerfile.runtime']],'original recipes mismatch');")
s=s.replace("def run(label,argv,cap=15,inspect=False,readback=False):", "def run(label,argv,cap=15,inspect=False,readback=False,iid_read=False):")
s=s.replace("receipt={'argv':prefix+argv,'cap_seconds':cap}","command=(['/usr/bin/sudo','-n','/usr/bin/cat']+argv) if iid_read else prefix+argv;require(not iid_read or (len(argv)==1 and argv[0] in [str(OLD/'base.iid'),str(out/'runtime.iid')]),'scoped IID command');receipt={'argv':command,'cap_seconds':cap}")
s=s.replace("subprocess.Popen(prefix+argv,env=env", "subprocess.Popen(command,env=env")
s=s.replace("      if x.get('Config',{}).get('Env') is not None:x['Config']['Env']=[{'name':v.split('=',1)[0],'sha256':H(v.encode())} for v in x['Config']['Env']]", "      for section in ['Config','ContainerConfig']:\n       if x.get(section,{}).get('Env') is not None:x[section]['Env']=[{'name':v.split('=',1)[0],'sha256':H(v.encode())} for v in x[section]['Env']]")
start=s.index("  nonce=str(uuid.uuid4())");end=s.index("  for phase,ctx,df in",start)
s=s[:start]+'''  def read_iid(path,label):
   for c in [path,*path.parents]:require(not c.is_symlink(),'IID symlink path')
   st=path.lstat();require(stat.S_ISREG(st.st_mode) and st.st_nlink==1 and st.st_uid==0 and stat.S_IMODE(st.st_mode)==0o600 and st.st_size in [71,72],'IID type/owner/mode/size')
   rc,raw,err,_=run(label,[str(path)],iid_read=True,readback=True)
   require(rc==0 and re.fullmatch(rb'sha256:[0-9a-f]{64}\\n?',raw),'IID read/shape')
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
''' +s[end:]
s=s.replace("[('base',base,out/'Dockerfile.ecbench'),('runtime',overlay,out/'Dockerfile.runtime')]", "[('runtime',overlay,out/'Dockerfile.runtime')]")
s=s.replace("image_id=iid.read_text().strip();", "image_id=read_iid(iid,'runtime-iid');")
s=s.replace("   result['images'][phase]={'id':image_id", "   require(x['RootFS']['Layers'][:len(result['images']['base']['layers'])]==result['images']['base']['layers'],'runtime base layer prefix');require(len(x['RootFS']['Layers'])>len(result['images']['base']['layers']),'runtime layer missing')\n   result['images'][phase]={'id':image_id")
(D/'recover.py').write_text(s);ast.parse(s);(D/'correction.patch').write_text(''.join(difflib.unified_diff(old.splitlines(True),s.splitlines(True),fromfile=str(O),tofile='recover.py')))
b=json.loads((Path('/var/tmp/pencil-copy-only-assembly-execution-20261031-v1')/'binding.json').read_text());b['admitted']=False;b['reviewed_supervisor_sha256']=hashlib.sha256(s.encode()).hexdigest();b['prospective_review_sha256']=None;b['recovery_sources']={n:hashlib.sha256((Path('/var/tmp/pencil-copy-only-assembly-execution-20261031-v1')/n).read_bytes()).hexdigest() for n in ['receipt.json','binding.json','context-inventory.json','Dockerfile.ecbench','Dockerfile.runtime','005-build-base/receipt.json','006-uncertain-readback-base/inspection.json']};(D/'binding.template.json').write_text(json.dumps(b,indent=2)+'\n');print(b['reviewed_supervisor_sha256'])
