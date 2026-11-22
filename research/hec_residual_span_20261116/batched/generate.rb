s=File.read('research/hec_residual_span_20261116/normalized/pencil.rs').sub('#[path = "source.rs"]','#[path = "../normalized/source.rs"]')
s="#[path=\"inverse.rs\"]\npub mod inverse;\n"+s
start=s.index('        for (j, &(x, y)) in fb.iter().enumerate() {') or raise 'start'
finish=s.index('            let v = anchor.v.eval',start) or raise 'finish'
s[start...finish]=<<~RS.lines.map{|l|'        '+l}.join
let mut entries=vec![];let mut denominators=vec![];
for (j,&(x,y)) in fb.iter().enumerate(){let u=anchor.u.eval(x,&c.fp);if u!=0{entries.push((j,x,y));denominators.push(u)}}
let inverses=inverse::invert(&denominators,&c.fp);
for ((j,x,y),inv) in entries.into_iter().zip(inverses) {
RS
File.write('research/hec_residual_span_20261116/batched/core.rs',s)
