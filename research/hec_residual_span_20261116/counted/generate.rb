require 'fileutils'
root='research/hec_residual_span_20261116/counted'
FileUtils.mkdir_p("#{root}/native")
FileUtils.cp_r('crates/hec-native-ic/src',"#{root}/native/")
File.write("#{root}/native/Cargo.toml", "[package]\nname=\"hec-native-ic\"\nversion=\"0.1.0\"\nedition=\"2021\"\n[workspace]\n[features]\ncount-inv=[]\n")
p="#{root}/native/src/lib.rs";File.open(p,'a'){|f|f.puts '\npub mod counted;'.sub('\\n',"\n")}
count=<<~RS
use std::{cell::RefCell,collections::BTreeMap};
pub type Counts=BTreeMap<u64,[u64;16]>;
thread_local!{static COUNTS:RefCell<Counts>=RefCell::new(BTreeMap::new());}
pub fn hit(p:u64,j:usize,n:u64){COUNTS.with(|c|c.borrow_mut().entry(p).or_insert([0;16])[j]+=n)}
pub fn reset(){COUNTS.with(|c|c.borrow_mut().clear())}
pub fn snapshot()->Counts{COUNTS.with(|c|c.borrow().clone())}
RS
File.write("#{root}/native/src/counted.rs",count)
# Insert tagged standalone accounting lines only; reverse-check preserves all arithmetic.
inserts=[]
p="#{root}/native/src/fp.rs";s=File.read(p)
{'new'=>0,'add'=>1,'sub'=>2,'neg'=>3,'mul'=>4,'pre'=>5,'mul_pre'=>6,'reduce128'=>7,'redc'=>8,'inv'=>9}.each{|name,idx| pat=/(\b(?:pub )?fn #{name}\([^\n]*\) -> [^{]+\{\n)/; raise "missing #{name}" unless s.match?(pat);s=s.sub(pat){|m|m+"        crate::counted::hit(#{name=='new' ? 'p' : 'self.p'},#{idx},1); // COUNTED\n"}}
s=s.sub("        while r1 != 0 {\n","        while r1 != 0 {\n            crate::counted::hit(self.p,10,1); // COUNTED\n");File.write(p,s)
p="#{root}/native/src/poly.rs";s=File.read(p);needle='                acc += self.c[i] as u128 * o.c[k - i] as u128;';raise unless s.include?(needle);s=s.sub(needle,"                crate::counted::hit(fp.p,11,1); // COUNTED\n"+needle);s=s.sub(/(pub fn divrem\([^\n]*\) -> [^{]+\{\n)/){|m|m+"        crate::counted::hit(fp.p,12,1); // COUNTED\n"};File.write(p,s)
p="#{root}/native/src/jac.rs";s=File.read(p);{'add'=>13,'add_place'=>14,'mul'=>15}.each{|name,idx|pat=/(pub fn #{name}\([^\n]*\) -> [^{]+\{\n)/;raise "missing jac #{name}" unless s.match?(pat);s=s.sub(pat){|m|m+"        crate::counted::hit(self.fp.p,#{idx},1); // COUNTED\n"}};File.write(p,s)
Dir['crates/hec-native-ic/src/*.rs'].each{|orig|copy="#{root}/native/src/#{File.basename(orig)}";s=File.read(copy).lines.reject{|l|l.include?('// COUNTED')}.join;s=s.sub(/\npub mod counted;\n\z/,"") if File.basename(orig)=='lib.rs';raise "reversal mismatch #{orig}" unless s==File.read(orig)}
File.write("#{root}/COPY-VERIFIED.txt","Every original native src/*.rs is byte-identical after removing tagged COUNTED lines and the counted module declaration. Extra counted.rs contains counters only. Verified before execution.\n")
