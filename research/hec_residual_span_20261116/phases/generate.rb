root='research/hec_residual_span_20261116/phases'
lib=File.read('research/hec_residual_span_20261116/counted/native/src/lib.rs').gsub(/^pub mod (\w+);$/){|m|name=$1; name=='counted' ? m : "#[path=\"../../../counted/native/src/#{name}.rs\"]\n#{m}"}
File.write("#{root}/native/src/lib.rs",lib)
File.write("#{root}/native/Cargo.toml","[package]\nname=\"hec-native-ic\"\nversion=\"0.1.0\"\nedition=\"2021\"\n[workspace]\n[features]\ncount-inv=[]\n")
File.write("#{root}/native/src/counted.rs",<<~RS)
use std::{cell::RefCell,collections::BTreeMap};
pub type Counts=BTreeMap<u64,[u64;16]>;
type Phases=BTreeMap<(&'static str,u64),[u64;16]>;
thread_local!{static TOTAL:RefCell<Counts>=RefCell::new(BTreeMap::new());static PHASES:RefCell<Phases>=RefCell::new(BTreeMap::new());static STAGE:RefCell<&'static str>=const{RefCell::new("entry")};}
pub fn stage(s:&'static str){STAGE.with(|c|*c.borrow_mut()=s)}
pub fn hit(p:u64,j:usize,n:u64){TOTAL.with(|c|c.borrow_mut().entry(p).or_insert([0;16])[j]+=n);let s=STAGE.with(|c|*c.borrow());PHASES.with(|c|c.borrow_mut().entry((s,p)).or_insert([0;16])[j]+=n)}
pub fn reset(){TOTAL.with(|c|c.borrow_mut().clear());PHASES.with(|c|c.borrow_mut().clear());stage("entry")}
pub fn snapshot()->Counts{TOTAL.with(|c|c.borrow().clone())}
pub fn phases()->Phases{PHASES.with(|c|c.borrow().clone())}
RS
sources={'pencil'=>'pencil-runtime/core.rs','walk'=>'production/core.rs'}
sources.each do |name,orig|
 s=File.read("research/hec_residual_span_20261116/#{orig}")
 markers= name=='pencil' ? {
 '    let fb = factor_base(c);'=>'factor_base',
 '    let (map, reps) = partition(c, &fb, ell);'=>'action',
 '    for a in 1..=16u64 {'=>'anchor',
 '        anchor = c.add(&anchor, source);'=>'anchor',
 '        let mut buckets ='=>'sieve',
 '            let vt ='=>'rows',
 '            let k = solve_for_last'=>'la',
 '                assert_eq!(c.mul(source, k), *target);'=>'verify'
 } : {
 '    let fb = factor_base(c);'=>'factor_base',
 '    let (map, reps) = if'=>'action',
 '    let folded ='=>'walk_prep',
 '        rep.trials = trial;'=>'walk',
 '        let Some(roots) ='=>'oracle',
 '        let mut v = vec![0; cols];'=>'rows',
 '        let k = solve_for_last'=>'la',
 '            assert_eq!(c.mul(source, k), *target);'=>'verify'
 }
 markers.each do |needle,label|
  raise "missing #{name}:#{needle}" unless s.include?(needle)
  s=s.sub(needle,"#{needle[/^ */]}hec_native_ic::counted::stage(\"#{label}\"); // PHASE\n#{needle}")
 end
 File.write("#{root}/#{name}.rs",s)
 raise 'reverse mismatch' unless s.lines.reject{|l|l.include?('// PHASE')}.join==File.read("research/hec_residual_span_20261116/#{orig}")
end
s=File.read('research/hec_residual_span_20261116/counted/main.rs').sub('#[path = "../pencil-runtime/core.rs"]','#[path = "pencil.rs"]').sub('#[path = "../production/core.rs"]','#[path = "walk.rs"]').gsub('research/hec_residual_span_20261116/counted/FIXTURES.tsv',"#{root}/FIXTURES.tsv")
# Keep original total rows and append phase rows to a separate receipt.
s=s.sub('    let mut receipt =',"    let prior=std::fs::read_to_string(\"research/hec_residual_span_20261116/counted/CONTROL.tsv\").unwrap();\n    let mut phase_out=std::fs::File::create_new(\"#{root}/PHASES.tsv\").unwrap();\n    writeln!(phase_out,\"g\\tp\\tsm\\tcase\\tarm\\trep\\tphase\\tmodulus\\tcounts\").unwrap();\n    let mut receipt =")
needle='                        let counts = counted::snapshot();'
raise 'counts marker' unless s.include?(needle)
s=s.sub(needle,needle+<<~RS)
                        let phases=counted::phases();
                        let mut sums=counted::Counts::new();
                        for ((phase,modulus),v) in &phases{for j in 0..16{sums.entry(*modulus).or_insert([0;16])[j]+=v[j]}write!(phase_out,"{g}\\t{p}\\t{sm}\\t{case}\\t{}\\t{rep}\\t{phase}\\t{modulus}",["pencil","folded","expanded"][arm]).unwrap();for n in v{write!(phase_out,"\\t{n}").unwrap()}writeln!(phase_out).unwrap()}
                        assert_eq!(sums,counts);
RS
needle='                            assert!(modulus == p || modulus == x.ell);'
raise 'mod marker' unless s.include?(needle)
s=s.sub(needle,needle+<<~RS)
                            let refrow=prior.lines().skip(1).map(|l|l.split('\\t').collect::<Vec<_>>()).find(|r|r[0]==g.to_string()&&r[1]==p.to_string()&&r[2]==sm.to_string()&&r[3]==case.to_string()&&r[4]==name&&r[5]==rep.to_string()&&r[6]==modulus.to_string()).unwrap();
                            for j in 0..16{assert_eq!(v[j],refrow[8+j].parse::<u64>().unwrap())}
RS
File.write("#{root}/main.rs",s)
