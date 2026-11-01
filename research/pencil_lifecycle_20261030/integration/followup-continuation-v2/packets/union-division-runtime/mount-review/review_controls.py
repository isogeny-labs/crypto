"""Independent pure-parser software controls; never imports the supervisor."""
import hashlib, importlib.util, itertools, json
from pathlib import Path
root=Path(__file__).parent
spec=importlib.util.spec_from_file_location('reviewed_mount_diff', root/'snapshot/mount_diff.py')
module=importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
parse=module.mount_diff
lines=(b'A /qualification/exec-gate',b'A /fixtures',b'A /qualification')
base=b'\n'.join(lines)+b'\n'
expected={'schema':'software.exact-mount-diff/v1','changes':[{'kind':'added','path':p} for p in ['/fixtures','/qualification','/qualification/exec-gate']],'writer_attribution':None,'no_writes_claim':False}
cases=[]
for i,order in enumerate(itertools.permutations(lines)):
 cases.append((f'permutation-{i}',b'\n'.join(order)+b'\n',True))
cases.append(('retained-v5-diff',(root/'snapshot/native-diff.txt').read_bytes(),True))
class BytesSubclass(bytes):pass
neg={
 'str-type':base.decode(), 'bytearray-type':bytearray(base), 'memoryview-type':memoryview(base),
 'bytes-subclass':BytesSubclass(base),'null-type':None,'integer-type':3,
 'empty':b'', 'partial':b'\n'.join(lines[:2])+b'\n', 'duplicate':b'\n'.join((lines[0],lines[1],lines[1]))+b'\n',
 'missing-newline':base[:-1],'double-newline':base+b'\n','leading-newline':b'\n'+base,
 'CRLF':base.replace(b'\n',b'\r\n'),'CR-only':base.replace(b'\n',b'\r'),
 'unicode-lookalike':base.replace(b'/fixtures','/fixturеs'.encode()),'invalid-utf8':base.replace(b'/fixtures',b'/fixtures\xff'),
 'NUL':base.replace(b'/fixtures',b'/fixtures\0'),'tab-separator':base.replace(b'A ',b'A\t',1),
 'double-space':base.replace(b'A ',b'A  ',1),'trailing-space':base.replace(b'/fixtures',b'/fixtures '),
 'leading-space':b' '+base,'lowercase-operation':base.replace(b'A ',b'a ',1),
 'dot-component':base.replace(b'/fixtures',b'/./fixtures'),'parent-component':base.replace(b'/fixtures',b'/tmp/../fixtures'),
 'double-slash':base.replace(b'/fixtures',b'//fixtures'),'trailing-slash':base.replace(b'/fixtures',b'/fixtures/'),
 'prefix-lookalike':base.replace(b'/fixtures',b'/fixtures2'),'extra-child':base+b'A /qualification/exec-gate/child\n',
 'oversized-1025':b'x'*1025,'oversized-1024-malformed':b'x'*1024,
}
for kind in (b'C',b'D'):
 for i in range(3):
  changed=list(lines);changed[i]=kind+changed[i][1:]
  neg[f'{kind.decode()}-line-{i}']=b'\n'.join(changed)+b'\n'
cases.extend((n,v,False) for n,v in neg.items())
rows=[]
for name,value,wanted in cases:
 try:
  result=parse(value);accepted=True
 except ValueError as error:
  result={'exception':'ValueError','message':str(error)};accepted=False
 if accepted!=wanted:raise RuntimeError(f'unexpected acceptance: {name}')
 if accepted and result!=expected:raise RuntimeError(f'output contract mismatch: {name}')
 if type(value) is bytes:raw=value
 else:raw=repr(value).encode()
 rows.append({'case':name,'input_type':type(value).__name__,'expected_acceptance':wanted,'accepted':accepted,'input_sha256':hashlib.sha256(raw).hexdigest(),'result':result})
(root/'controls.json').write_text(json.dumps({'schema':'independent.pure-parser-controls/v1','cases':rows,'count':len(rows),'accepted':sum(r['accepted'] for r in rows),'rejected':sum(not r['accepted'] for r in rows),'all_expected':True,'docker_invoked':False,'runtime_execution':False},indent=2)+'\n')
print(len(rows),'independent fixed cases matched')
