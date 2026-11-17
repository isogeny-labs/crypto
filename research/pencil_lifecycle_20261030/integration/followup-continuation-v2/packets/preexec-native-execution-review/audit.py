from pathlib import Path
import json,hashlib,struct,re
D=Path(__file__).parent;P=Path('/var/tmp/pencil-preexec-native-controls-20261031-v2');E=Path('/var/tmp/pencil-preexec-native-execution-20261031-v2');H=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();rows=[]
for rel,h in json.loads((P/'manifest.json').read_text())['files'].items():rows.append({'path':str(P/rel),'expected':h,'actual':H(P/rel),'match':H(P/rel)==h})
compiler=[]
for row in json.loads((P/'compiler-bindings.json').read_text()):compiler.append({**row,'current_sha256':H(Path(row['path'])),'match':H(Path(row['path']))==row['sha256']})
(D/'hash-check.json').write_text(json.dumps({'producer_files':rows,'compiler_inputs':compiler,'all_match':all(x['match'] for x in rows+compiler)},indent=2))
r=json.loads((E/'receipt.json').read_text());elf={}
for name in ['gate','controls']:
 raw=(E/name).read_bytes();assert raw[:6]==b'\x7fELF\x02\x01';et,em=struct.unpack_from('<HH',raw,16);off=struct.unpack_from('<Q',raw,32)[0];ent,n=struct.unpack_from('<HH',raw,54);types=[struct.unpack_from('<I',raw,off+i*ent)[0] for i in range(n)];assert em==62 and 3 not in types;elf[name]={'sha256':H(E/name),'receipt_match':H(E/name)==r[name+'_binary_sha256'],'elf_type':et,'machine':em,'program_header_types':types,'has_interp':3 in types,'has_dynamic':2 in types}
commands={p.stem.replace('.receipt',''):json.loads(p.read_text()) for p in E.glob('*.receipt.json')};dis=(E/'gate-disassembly.stdout').read_text();main=dis.split('<main>:',1)[1].split('\n\n',1)[0];calls=re.findall(r'\bcall\s+[^<]*<([^>]+)>',main);loads=[l for l in (E/'gate.map').read_text().splitlines() if l.startswith('LOAD ')];
(D/'native-readback.json').write_text(json.dumps({'elf':elf,'commands':commands,'main_calls':calls,'link_LOAD_entries':loads,'main_execve_calls':calls.count('execve'),'count_derivation':{'nonce':7,'mountinfo':9,'exact_values_including_open_close':11,'bounded_reader':12,'host_cases':6,'total':45},'all_expected_exits':all(v['exit_code']==(126 if k=='pristine-host-negative' else 0) and not v['timeout'] for k,v in commands.items()),'fixed_stdout':(E/'fixed-controls.stdout').read_text(),'host_negative_stderr':(E/'pristine-host-negative.stderr').read_text()},indent=2))
(D/'evidence.json').write_text(json.dumps({str(p):H(p) for p in list(E.iterdir()) if p.is_file()},indent=2));print('bindings',len(rows),len(compiler),'match',all(x['match'] for x in rows+compiler));print('main_calls',calls);print('elf',elf)
