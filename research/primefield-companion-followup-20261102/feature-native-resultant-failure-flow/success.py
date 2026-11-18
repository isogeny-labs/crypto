"""Journal-aware complete-success adapter, reusing the canonical predecessor."""
from pathlib import Path
import sys,argparse
import failure
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'feature-native-resultant-protocol'))
from common import load,canonical,protocol,bindings,sha,require,analyzer
previous=load('previous_success',Path(__file__).resolve().parent.parent/'feature-native-resultant-protocol/export.py')

def build(p,pp,raw,events,receipt,out):
 r=analyzer.read_json(receipt.read_text());require(events.exists()and r.get('events_sha256')==sha(events),'journal receipt mismatch')
 b=bindings(p,pp.parent);rows,rt=failure.prefix(raw.read_bytes());ev,et=failure.prefix(events.read_bytes());require(not rt and not et,'truncated successful attempt')
 flow=failure.classify(ev,rows,analyzer.fixtures(b['inputs'].read_text()),b['metrics'].read_text().splitlines(),p['freeze']['protocol_sha256'],sha(b['inputs']))
 require(len(flow['completed'])==240 and not flow['completion_unconfirmed'],'incomplete success journal')
 e,a=previous.build(p,pp,raw,receipt,out)
 import os
 e['files'].append(canonical.file_entry(out,os.path.relpath(events,out),'log'))
 require(not canonical.lint_export(e,p,out,True),'canonical export findings');return e,a

def main():
 a=argparse.ArgumentParser()
 for k in ['protocol','expected-protocol','raw','events','receipt','out']:a.add_argument('--'+k,required=True)
 args=a.parse_args();pp=Path(args.protocol).resolve();p=protocol(pp,args.expected_protocol);out=Path(args.out).resolve();out.mkdir(parents=True,exist_ok=False)
 e,analysis=build(p,pp,Path(args.raw).resolve(),Path(args.events).resolve(),Path(args.receipt).resolve(),out)
 canonical.write_once(out/'comparison.json',analysis);e['analysis']['outputs']=[canonical.file_entry(out,'comparison.json','analysis')];canonical.write_once(out/'export.json',e)
if __name__=='__main__':main()
