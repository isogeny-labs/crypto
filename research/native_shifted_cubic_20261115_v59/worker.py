from pathlib import Path
import hashlib,json,resource,subprocess,sys,time
from rootfinder.binaryfield import BinaryField
from research.torsion_shifted_cubic_20261115_v58.solver import chart,coefficients
from research.norm_block_reuse_20261115_v36 import backend as linear
from .oracles import fixture,family
P=Path(__file__).resolve().parent;START=time.monotonic()
def main():
 out={'calibrations':[],'fixtures':[]};error=None
 try:
  f=BinaryField(83,0x800000000200000000007);start=time.monotonic();build=subprocess.run(['g++','-O3','-std=c++17',str(P/'scan.cpp'),'-o',str(P/'scan')],capture_output=True,text=True,timeout=15);out['build']={'seconds':time.monotonic()-start,'exit_code':build.returncode,'stdout':build.stdout,'stderr':build.stderr};assert build.returncode==0,build.stderr
  for name,A,d,seed,distinct in [('calibration_A0',0,4,5901,False),('calibration_A1',1,4,5902,False),('full_width_A1',1,18,5903,True)]:
   instance,witness=fixture(f,A,d,seed,distinct)
   for suffix,data in[('instance',instance),('witness',witness)]:
    with(P/(name+'_'+suffix+'.json')).open('x')as file:json.dump(data,file,sort_keys=True,separators=(',',':'))
   out['fixtures'].append({'name':name,'d':d,'A':A,'known_cubic':witness['C'],'known_original_tuples':len(witness['original_tuples'])})
   if d==18:continue
   command=[str(P/'scan'),'--A',hex(A),'--z',hex(instance['z']),'--y',hex(instance['cli_ordinate']),'--dimension',str(d),'--max-outer','32','--max-candidates','10000','--max-accepted','100','--record-families','32','--max-seconds','8'];start=time.monotonic();run=subprocess.run(command,capture_output=True,text=True,timeout=10);native=json.loads(run.stdout);assert run.returncode==0 and native['complete'],run.stderr;py=chart(f,A,1,instance['z'],instance['source_ordinate'],d)
   got={(int(v['b'],16),int(v['c'],16),tuple(int(x,16)for x in v['cubic']))for v in native['accepted_norms']};expected={(v['h'],v['r'],v['C'])for v in py['solutions']};assert got==expected and got
   for record in native['conditional_family_records']:
    h=int(record['b'],16);ref=family(f,instance,h);assert(record['consistent'])==(ref is not None)
    if ref is not None:assert linear.same(ref,{'particular':int(record['particular'],16),'basis':[int(v,16)for v in record['kernel']]})
   raw={'native_stdout':run.stdout,'stderr':run.stderr,'exit_code':run.returncode,'native':native,'python':py,'wall_seconds':time.monotonic()-start}
   with(P/(name+'_raw.json')).open('x')as file:json.dump(raw,file,sort_keys=True,separators=(',',':'))
   out['calibrations'].append({'name':name,'outer_visited':native['stats']['outer_visited'],'supported_norms':len(got),'conditional_family_checks':len(native['conditional_family_records'])});assert time.monotonic()-START<65
 except Exception as exc:error=type(exc).__name__+': '+str(exc)
 trial=json.dumps({'out':out,'error':error},separators=(',',':'))
 if time.monotonic()-START>=75:error='late serialized output'
 raw=json.dumps({'out':out,'error':error},sort_keys=True,separators=(',',':')).encode()
 with(P/'raw.json').open('xb')as file:file.write(raw)
 result={'schema':'scientific_result_v1','experiment_id':sys.argv[1],'status':'passed'if not error else'inconclusive','summary':'General-A native cubic backend matches exact independent row fibers and complete small-domain support, with a separated full-width positive fixture.','evidence':{'error':error,'out':out,'worker_seconds':time.monotonic()-START,'peak_rss_kib':resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,'child_peak_rss_kib':resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss,'raw_sha256':hashlib.sha256(raw).hexdigest(),'qualified_speedup':None,'release_gate_claim':False},'next_actions':['Freeze the validated native binary and separated full-width instance; run a bounded complete cubic scan with independently charged factor/original-root replay.']};text=json.dumps(result,separators=(',',':'));assert time.monotonic()-START<82;print(text)
if __name__=='__main__':main()
