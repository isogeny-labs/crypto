"""Repository-portable algebra/receipt replay; native rerun is optional and explicit."""
from pathlib import Path
import argparse,hashlib,json,subprocess,time
from research.k0_zero_cubic_20261115_v43.worker import controls
P=Path(__file__).resolve().parent;R=P.parents[1]
def main():
 parser=argparse.ArgumentParser();parser.add_argument('--native',type=Path);args=parser.parse_args();begin=time.monotonic();folder=R/'research/k0_zero_cubic_20261115_v43';blob=(folder/'raw-controls.json').read_bytes();assert hashlib.sha256(blob).hexdigest()=='a33a11d6cc722427bca9673e657c7f7b6a602cd9bf050732d532393f672e8a29';assert controls()==json.loads(blob)
 hist=json.loads((folder/'HISTORICAL_CUBIC_SCIENCE.json').read_text());assert hist['complete']and hist['supported_norm_count_exact']==0 and hist['stats']['outer_visited']==262144 and hist['stats']['candidates_visited']==262164
 for name,digest in[('binary_triple_native_v3/triple_scan.cpp','00e32f9a07a320e6053d3f2512a5e7f1ca8dcc2d815ed5e223fad74b668299ae'),('lab_variant_02/native_gf83.h','9fd30bf6acd6714d2b0f4947891e36b97edb02e8cea8fce78ebb98da1dbc626a')]:assert hashlib.sha256((folder/name).read_bytes()).hexdigest()==digest
 rerun=False
 if args.native:
  r=subprocess.run([str(args.native.resolve()),'--z',hist['target'][0],'--y',hist['target'][1],'--dimension','18','--max-outer','262144','--max-candidates','16777216','--max-nullity','12','--max-accepted','1024','--max-seconds','55'],capture_output=True,text=True,timeout=60);assert r.returncode==0;n=json.loads(r.stdout);assert n['complete']and n['supported_norm_count_exact']==0;rerun=True
 print(json.dumps(dict(bridge_parameter_states=2688,historical_cubic_candidates=262164,historical_cubic_norms=0,native_rerun=rerun,seconds=time.monotonic()-begin,scope='Algebra bridge plus audited historical normcertificate; no new originalroot result')))
if __name__=='__main__':main()
