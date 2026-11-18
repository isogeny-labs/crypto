"""Exact reversible journal/receipt insertion; predecessor files remain immutable."""
from pathlib import Path
import json,hashlib
D=Path(__file__).resolve().parent;R=D.parent
sha=lambda b:hashlib.sha256(b).hexdigest()
records=[]
for dest,source in [('main.rs',R/'feature-native-resultant-panel/main.rs'),('capture.py',R/'feature-native-resultant-protocol/capture.py')]:
 original=source.read_text();s=original;edits=[]
 def change(old,new):
  global s
  assert s.count(old)==1,(dest,old)
  s=s.replace(old,new);edits.append({'old':old,'new':new})
 if dest=='main.rs':
  change('mod panel;','#[path="../feature-native-resultant-panel/panel.rs"]mod panel;\ninclude!("journal.rs");')
  change('panel::self_test();meter::self_test();','panel::self_test();meter::self_test();journal_self_test();')
  old='let mut out=std::fs::OpenOptions::new().write(true).create_new(true).open(output).unwrap();'
  change(old,old+'\n let mut events=std::fs::OpenOptions::new().write(true).create_new(true).open(format!("{}.events",output)).unwrap();')
  change('let arm=ARMS[(offset+c.id)%5];let(v,s)=evaluate', 'let arm=ARMS[(offset+c.id)%5];journal_event(&mut events,c.id*5+offset,"start",protocol,input_hash);let(v,s)=evaluate')
  change('out.flush().unwrap();','out.flush().unwrap();out.sync_all().unwrap();journal_event(&mut events,c.id*5+offset,"complete",protocol,input_hash);')
 else:
  change('from common import protocol,bindings,sha,require,canonical,analyzer','import sys\nsys.path.insert(0,str(Path(__file__).resolve().parent.parent/"feature-native-resultant-protocol"))\nfrom common import protocol,bindings,sha,require,canonical,analyzer')
  change("'attempts':1,'host':host_facts()", "'attempts':1,'events_sha256':sha(Path(str(rows)+'.events'))if Path(str(rows)+'.events').exists()else None,'host':host_facts()")
 reverse=s
 for e in reversed(edits):assert reverse.count(e['new'])==1;reverse=reverse.replace(e['new'],e['old'])
 assert reverse==original
 (D/dest).write_text(s);records.append(dict(output=dest,source=str(source.relative_to(R)),source_sha256=sha(original.encode()),output_sha256=sha(s.encode()),edits=edits,reverse_exact=True))
(D/'generation.json').write_text(json.dumps(records,indent=2)+'\n')
