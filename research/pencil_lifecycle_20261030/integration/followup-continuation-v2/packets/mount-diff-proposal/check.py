from pathlib import Path
import itertools,json,hashlib
from mount_diff import mount_diff
root=Path(__file__).parent
lines=['A /fixtures','A /qualification','A /qualification/exec-gate']
rows=[]
for i,order in enumerate(itertools.permutations(lines)):
 raw=('\n'.join(order)+'\n').encode();result=mount_diff(raw);assert len(result['changes'])==3 and result['writer_attribution'] is None and result['no_writes_claim'] is False
 rows.append({'case':'ordering-'+str(i),'accepted':True,'input_sha256':hashlib.sha256(raw).hexdigest()})
base=('\n'.join(lines)+'\n').encode()
bad={'empty':b'','missing':b'A /fixtures\n','duplicate':b'A /fixtures\nA /fixtures\nA /qualification\n','extra-child':base+b'A /qualification/other\n','prefix':base.replace(b'/fixtures',b'/fixtures2'),'suffix':base.replace(b'/fixtures',b'/fixtures/'),'dot':base.replace(b'/fixtures',b'/./fixtures'),'double-space':base.replace(b'A ',b'A  ',1),'tab':base.replace(b'A ',b'A\t',1),'crlf':base.replace(b'\n',b'\r\n'),'no-final-newline':base[:-1],'extra-final-newline':base+b'\n','non-ascii':base+b'\xff','oversized':base+b'x'*1025,'NUL':base.replace(b'/fixtures',b'/fixtures\0')}
for kind in [b'C',b'D']:
 for i in range(3):
  ls=[x.encode() for x in lines];ls[i]=kind+ls[i][1:];bad[kind.decode()+'-'+str(i)]=b'\n'.join(ls)+b'\n'
for name,raw in bad.items():
 try:mount_diff(raw)
 except ValueError:rows.append({'case':name,'accepted':False,'input_sha256':hashlib.sha256(raw).hexdigest()})
 else:raise AssertionError(name)
actual=Path('/var/tmp/pencil-runtime-qualification-execution-20261031-v5/030-diff-Q1/stdout').read_bytes();observed=mount_diff(actual)
(root/'checks.json').write_text(json.dumps({'schema':'software.fixed-parser-checks/v1','cases':rows,'actual_diff_sha256':hashlib.sha256(actual).hexdigest(),'actual_diff_parse':observed,'all_expected_outcomes':True,'runtime_execution':False},indent=2)+'\n');print(len(rows),'fixed cases passed')
