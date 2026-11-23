from pathlib import Path
import subprocess,time,json,hashlib
D=Path(__file__).parent;assert json.loads((D/'build-receipt.json').read_text())['exit_code']==0
commands=[('compile',['timeout','-k','2s','30s',str(D/'prefix/bin/musl-gcc'),'-static','-O2','-march=x86-64',str(D/'hello.c'),'-Wl,-Map='+str(D/'hello.map'),'-o',str(D/'hello')]),('elf_header',['readelf','-h',str(D/'hello')]),('elf_programs',['readelf','-l',str(D/'hello')]),('elf_dynamic',['readelf','-d',str(D/'hello')]),('run',['timeout','-k','1s','5s',str(D/'hello')])];receipts=[]
for name,cmd in commands:
 t=time.time();r=subprocess.run(cmd,capture_output=True,text=True,env={'PATH':'/usr/bin:/bin','LC_ALL':'C'});(D/(name+'.stdout')).write_text(r.stdout);(D/(name+'.stderr')).write_text(r.stderr);receipts.append({'name':name,'command':cmd,'exit_code':r.returncode,'start':t,'end':time.time()});(D/'smoke-receipts.json').write_text(json.dumps(receipts,indent=2));assert r.returncode==0,(name,r.stderr)
 if name=='elf_header':assert 'Advanced Micro Devices X86-64'in r.stdout
 if name=='elf_programs':assert 'INTERP'not in r.stdout
 if name=='elf_dynamic':assert 'NEEDED'not in r.stdout
 if name=='run':assert r.stdout=='pencil-musl-smoke-ok\n'
print('static hello passed')
