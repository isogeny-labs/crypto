from pathlib import Path
import subprocess,json,hashlib,sys
D=Path(__file__).parent
cases=[('legacy-rejected','legacy_check.py','Bad',0,1),('strict-rejected','strict_check.py','Bad',1,1),('strict-positive','strict_check.py','Good',0,0),('strict-input-mutation','strict_check.py','Mutation',2,0)]
results=[]
for tag,helper,module,want,native in cases:
 p=subprocess.run([sys.executable,str(D/helper),module,tag],capture_output=True,text=True)
 (D/(tag+'.wrapper.stdout')).write_text(p.stdout);(D/(tag+'.wrapper.stderr')).write_text(p.stderr)
 receipt=json.loads((D/tag/'receipt.json').read_text());binding=json.loads((D/tag/'binding-check.json').read_text())
 row={'case':tag,'wrapper_exit':p.returncode,'native_exit':receipt['exit_code'],'inputs_unchanged':binding['unchanged'],'expected_wrapper_exit':want,'expected_native_exit':native};results.append(row)
 (D/'results.json').write_text(json.dumps(results,indent=2)+'\n');print(json.dumps(row),flush=True)
 if p.returncode!=want or receipt['exit_code']!=native:sys.exit(1)
 if binding['unchanged'] != (tag!='strict-input-mutation'):sys.exit(1)
(D/'marker-after.txt').write_bytes((D/'Marker.lean').read_bytes())
d=D/'strict-rejected'
def snapshot():return {str(p.relative_to(d)):hashlib.sha256(p.read_bytes()).hexdigest() for p in d.rglob('*') if p.is_file()}
before=snapshot();p=subprocess.run([sys.executable,str(D/'strict_check.py'),'Bad','strict-rejected'],capture_output=True,text=True);after=snapshot()
(D/'stale-output.wrapper.stdout').write_text(p.stdout);(D/'stale-output.wrapper.stderr').write_text(p.stderr)
row={'case':'stale-output','wrapper_exit':p.returncode,'native_execution':'not reached; existing directory rejected','retained_files_unchanged':before==after};results.append(row)
(D/'results.json').write_text(json.dumps(results,indent=2)+'\n');print(json.dumps(row),flush=True)
assert p.returncode!=0 and before==after
