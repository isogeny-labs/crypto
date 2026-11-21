from pathlib import Path
import hashlib,json
p=Path(__file__).resolve().parent;old=p.parent/'feature-direct-phase-domain';rows=[]
for f in sorted(old.glob('*.rs')):
 if f.name=='driver.rs':continue
 (p/f.name).write_bytes(f.read_bytes());rows.append({'source':f.name,'sha256':hashlib.sha256(f.read_bytes()).hexdigest()})
s=(old/'direct-initial.rs').read_text().replace('DIRECT_INITIAL_POWER','SPLIT_INITIAL_POWER').replace('new_direct_phase','new_split_negative').replace('initial_direct_phase','initial_split_negative')
s=s.replace('initial_character:u64)->','initial_character:u64,trace_trials:usize)->').replace('Self::initial_split_negative(modes,b,f,i,slot_cap,initial_character)','Self::initial_split_negative(modes,b,f,i,slot_cap,initial_character,trace_trials)').replace('if m<3||','if trace_trials>64||m<3||')
s=s.replace('let phase_degree=if normalized==1{n/2+1}else{n};','let mut trace=None;\n  if normalized==r-1&&n%2==0{if m.checked_mul(2).and_then(|x|x.checked_mul(n/2)).ok_or(Error::Limit)?>slot_cap{return Err(Error::Limit)}trace=eighth_trace(r,trace_trials)?;}\n  let phase_degree=if trace.is_some(){n/2}else if normalized==1{n/2+1}else{n};')
s=s.replace('let domains=direct_phase_domains(r,n,i,normalized)?;','let domains=if let Some(s)=trace{split_negative_domains(r,n,i,s)?}else{direct_phase_domains(r,n,i,normalized)?};')
(p/'split-negative-initial.rs').write_text(s)
s=(old/'phase-domain-adapter.rs').read_text().replace('recover_direct_phase_image','recover_split_negative_image').replace('SeededImage::new_direct_phase','SeededImage::new_split_negative')
s=s.replace('power_slots:usize)->','power_slots:usize,trace_trials:usize)->').replace('if radii.len()>4','if trace_trials>64||radii.len()>4').replace('power_slots,initial_character)?','power_slots,initial_character,trace_trials)?').replace('max_calls,power_slots)}}','max_calls,power_slots,trace_trials)}}')
(p/'split-negative-adapter.rs').write_text(s)
s=(old/'driver.rs').read_text();s=s[:s.index('fn main(){')]+'include!("eighth-trace.rs");\ninclude!("split-negative-initial.rs");\ninclude!("split-negative-adapter.rs");\ninclude!("negative-split-checks.rs");\nfn main(){if std::env::args().any(|x|x=="--inherited"){inherited_checks()}else{negative_split_checks()}}\n'
(p/'driver.rs').write_text(s)
(p/'generation.json').write_text(json.dumps({'source_commit':'613a64d99950e3f01804c0503e7c1b5ac8a4c877','copied':rows},indent=2)+'\n')
