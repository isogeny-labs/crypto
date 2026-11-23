"""Static adaptation of frozen transport/consumer/decoder, no field execution."""
from pathlib import Path
import sys,json,hashlib
r=Path('research/primefield-companion-followup-20261102');d=r/'feature-native-monomial-recovery'
inputs=[r/'feature-native-monomial-transport/transport.rs',r/'feature-native-monomial-transport/lib.rs',r/'feature-native-batch-reciprocals/lib.rs']
def once(s,a,b):
 assert s.count(a)==1,(a,s.count(a))
 return s.replace(a,b)
def emit(name,s):
 if '--check' in sys.argv:assert(d/name).read_text()==s,name
 else:(d/name).write_text(s)
s=inputs[0].read_text()
s=once(s,"impl Prepared<'_>{", "impl<'a> Prepared<'a>{")
s=once(s,' pub fn window(&self,q:Point,length:usize)->Result<Window,Error>{'," pub fn window(&self,q:Point,length:usize)->Result<Window,Error>{self.begin(q,length,0).map(|(w,_)|w)}\n pub fn begin<'b>(&'b self,q:Point,length:usize,max_extra:usize)->Result<(Window,Cursor<'b,'a>),Error>{")
s=once(s,'if length>s.max_window','if length.checked_add(max_extra).ok_or(Error::Limit)?>s.max_window')
s=once(s,'let joint=self.geometry.joint_window(q,length)', 'let(joint,tail)=self.geometry.joint_window_tail(q,length)')
s=once(s,'  }Ok(out)','  }let previous=out.pairs[length-1].clone();let older=if length>=2{out.pairs[length-2].clone()}else{vec![]};let middle_x=x.pop().ok_or(Error::Domain)?;\n  Ok((out,Cursor{prepared:self,tail,count:length,previous,older,middle_x,remaining:max_extra}))')
s+='\ninclude!("transport-extra.rs");\n';emit('transport.rs',s)
s=inputs[1].read_text().replace('mod transport;','#[path="transport.rs"]mod transport;')
s=once(s,'Transport,Character}','Transport,Character,Limit}')
s=once(s,' fn ratio(&self,q:[u64;2],grid:&[[u64;2]],indices:&[u64],values:&[Pair])->Result<Pair,Error>{',' fn ratio(&self,q:[u64;2],grid:&[[u64;2]],indices:&[u64],values:&[Pair])->Result<Pair,Error>{\n  #[cfg(test)]SIGN_CALLS.with(|n|n.set(n.get()+1));')
s=s.replace('.map_err(|_|Error::Transport)', '.map_err(map_transport)')
s=s[:s.index("impl Prepared<'_>{")]+ '\ninclude!("source-extra.rs");\n'
emit('source.rs',s)
s=inputs[2].read_text().split('#[cfg(test)]mod tests{')[0]
s=s.replace('#[path="routed/source.rs"]mod source;','mod source;').replace("source::PreparedSource<'a>","source::Prepared<'a>")
s=once(s,'pub direct_phases:usize}','pub direct_phases:usize,pub initial_direct_initializations:usize,pub initial_conversions:usize}')
s=once(s,'r:u64,base:[u64;2])->Result<Self,Error>', 'r:u64,base:[u64;2],max_lanes:usize,max_window:usize)->Result<Self,Error>')
s=once(s,'source::Source::new(p,a,b,r,base)','source::Source::new(p,a,b,r,base,max_lanes,max_window)')
s=once(s,'steps,skipped_signs,conversions,direct_phases})','steps,skipped_signs,conversions,direct_phases,initial_direct_initializations:initial.direct_initializations,initial_conversions:initial.converted_points})')
s+='\n#[cfg(test)]mod tests;\n';emit('lib.rs',s)
emit('generation.json',json.dumps(dict(inputs={str(p.relative_to(r)):hashlib.sha256(p.read_bytes()).hexdigest()for p in inputs},outputs={name:hashlib.sha256((d/name).read_bytes()).hexdigest()for name in ['transport.rs','source.rs','lib.rs']},changes=['transport begin retains geometry tail, last two one-pair monomial vectors and middle x coordinates; budget covers initial plus extra rows','consumer gains begin, advance, skip_sign with shared normalization and test-only counters','quartic program construction/evaluation unchanged; source adapter gains explicit limits and initial-work fields']),indent=2)+'\n')
