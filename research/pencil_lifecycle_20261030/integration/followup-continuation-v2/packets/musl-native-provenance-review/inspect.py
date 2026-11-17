from pathlib import Path
import hashlib,json,re,shlex,shutil
D=Path(__file__).parent;P=Path('/var/tmp/pencil-musl-build-b985-20261031-v3');r=json.loads((P/'receipt.json').read_text());sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
files={n:{'sha256':sha(P/n),'bytes':(P/n).stat().st_size}for n in ['receipt.json','ecbench','ecbench.map','build.stdout','build.stderr','crt-libc-candidates.json','elf-header.stdout','elf-programs.stdout','elf-dynamic.stdout','head-before.stdout','head-after.stdout','tree.stdout']};assert files['ecbench']['sha256']==r['binary_sha256'];assert files['ecbench.map']['sha256']==r['link_map_sha256']
stdout=(P/'build.stdout').read_text();lines=[x for x in stdout.splitlines()if 'self-contained/rcrt1.o' in x];assert len(lines)==1;tokens=shlex.split(lines[0]);env={};i=0
while '=' in tokens[i] and not tokens[i].startswith('-'):
 k,v=tokens[i].split('=',1);env[k]=v;i+=1
args=tokens[i:];(D/'link-arguments.json').write_text(json.dumps({'environment_keys':sorted(env),'arguments':args},indent=2)+'\n')
loads=[];libc_members=set();native_members=set();private_refs=0
with (P/'ecbench.map').open()as f:
 for l in f:
  if '/var/tmp/pencil-musl-private-20261031/' in l:private_refs+=1
  if l.startswith('LOAD ') and ('self-contained' in l or 'libc.a' in l):loads.append(l.strip())
  libc_members.update(re.findall(r'([^\s()]+/libc\.a)\(([^)]+)\)',l))
  for pat in ['ring_core','sqlite3','blake3']:
   if pat in l and '.rlib(' in l:
    native_members.update(re.findall(r'([^\s()]+\.rlib)\(([^)]+)\)',l))
candidates=json.loads((P/'crt-libc-candidates.json').read_text());crt=[]
for f,h in candidates.items():
 q=Path(f);actual=sha(q);assert actual==h;crt.append({'path':f,'sha256':actual,'in_final_arguments':f in args,'in_map_loads':any(f in l for l in loads),'archive_members':len({m for a,m in libc_members if a==f})})
private={f:{'recorded_sha256':h,'current_sha256':sha(Path(f)),'match':h==sha(Path(f))}for f,h in r['compiler_inputs'].items()}
cc_lines=[l for l in stdout.splitlines()if 'CC_x86_64_unknown_linux_musl = Some(' in l]
driver=shutil.which(args[0],path=env.get('PATH'));info={'printed_driver':args[0],'resolved_now':driver,'realpath_now':str(Path(driver).resolve())if driver else None,'sha256_now':sha(Path(driver))if driver else None}
summary={'schema':'software.native-link-provenance/v1','files':files,'receipt_status':r['status'],'source_commit':r['source_commit_requested'],'source_tree':r['source_tree'],'source_manifest':r['source_manifest'],'build_exit_code':r['build_exit_code'],'actual_link_flags':[x for x in args if x in ['-static-pie','-nostartfiles','-nodefaultlibs']or x.startswith('-Wl,-Map=')],'driver':info,'crt_libc':crt,'map_loads':loads,'private_prefix_map_references':private_refs,'private_compiler_bindings':private,'c_dependency_compiler_declarations':cc_lines,'native_archive_member_examples':sorted(native_members)[:12],'elf':{'amd64':'Advanced Micro Devices X86-64'in(P/'elf-header.stdout').read_text(),'interpreter_present':'INTERP'in(P/'elf-programs.stdout').read_text(),'needed_present':'NEEDED'in(P/'elf-dynamic.stdout').read_text(),'dynamic_section_present':'Dynamic section'in(P/'elf-dynamic.stdout').read_text()},'no_executable_runs':True};(D/'evidence.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps({k:summary[k]for k in ['driver','crt_libc','map_loads','private_prefix_map_references','c_dependency_compiler_declarations','elf']},indent=2))
