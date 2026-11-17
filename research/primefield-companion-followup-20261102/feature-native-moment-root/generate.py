from pathlib import Path
import json,hashlib
D=Path(__file__).resolve().parent;src=D.parent/'feature-native-aggregate-projector/lib.rs';text=src.read_text();prefix,tail=text.split('#[path="../feature-native-standalone-source/lib.rs"]',1)
prefix=prefix.replace('//!','//',1);(D/'projector.rs').write_text(prefix)
tail='#[path="../feature-native-standalone-source/lib.rs"]'+tail.split('#[cfg(test)]mod aggregate_tests{')[0]
tail=tail.replace('max_queries:usize}', 'max_queries:usize,root_limit:usize}').replace('max_queries:usize)->Result<Self,Error>', 'max_queries:usize,root_limit:usize)->Result<Self,Error>').replace('max_coefficients,max_queries})','max_coefficients,max_queries,root_limit})')
tail=tail.replace('Projector::new(self.r,self.max_coefficients)?','Hybrid::new(self.r,self.max_coefficients,self.root_limit)?')
tail=tail.replace('pub counts:Vec<u64>}','pub counts:Vec<u64>,pub root_updates:usize,pub conversion_degree:Option<usize>}').replace('Recovery{value,transcript,counts}','Recovery{value,transcript,counts,root_updates:state.root_updates,conversion_degree:state.conversion_degree}')
(D/'recovery.rs').write_text(tail)
(D/'generation.json').write_text(json.dumps({'source_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'outputs':{n:hashlib.sha256((D/n).read_bytes()).hexdigest()for n in ['projector.rs','recovery.rs']},'transformation':'preserve projector prefix; preserve recovery arithmetic, substitute Hybrid state and add explicit root threshold and diagnostics'},indent=2)+'\n')
