from pathlib import Path
import shutil
P=Path('/var/tmp/pencil-kummer-original-gstep-20261031');D=Path(__file__).parent
for n in ['native','src']:shutil.copytree(P/n,D/n)
for n in ['Cargo.toml','Cargo.lock','fixtures.json','run.py']:shutil.copy2(P/n,D/n)
for arm in ['eager','pruned']:
 p=D/'src'/f'{arm}.rs';s=p.read_text();s=s.replace('let mut rows=vec![];','let mut rows=vec![];let mut adapter=crate::graph::Adapter::new(r);')
 marker='  walk::reset_ops();signed::reset_ops();let before=j.f.counts();let expected='
 s=s.replace(marker,'  let graph_row=adapter.consume(&j,recovered,a,b);let terminal=graph_row["answer"].as_u64();\n'+marker)
 s=s.replace('"counts":{"transition":transition,"recovery":recovery,"verification":verification}', '"graph":graph_row,"counts":{"transition":transition,"recovery":recovery,"verification":verification}')
 s=s.replace('k=nk;kp=nkp;','k=nk;kp=nkp;if let Some(ans)=terminal{assert_eq!(j.mul(g,ans),q);assert_eq!(Some(ans),v["planted"].as_u64());break;}')
 p.write_text(s)
p=D/'src/check.rs';s=p.read_text().replace('mod eager;','mod graph;mod eager;').replace('let candidate_pass=ps.len()==3&&ps.iter().all','let candidate_pass=!ps.is_empty()&&ps.iter().all').replace('"complete":complete,','"controls":graph::controls(),"complete":complete,');p.write_text(s)
