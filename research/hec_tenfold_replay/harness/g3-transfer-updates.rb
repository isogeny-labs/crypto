require 'open3'
curves=ARGV.fetch(0)
code,status=Open3.capture2('ruby','harness/g3-transfer-kernels.rb',curves)
raise unless status.success?
from='auto t=entries[i];work[t.col]+=1ULL*v*t.value;'
raise unless code.scan(from).size==2
puts 'static unsigned long long coefficient_updates;'
code=code.gsub(from,'++coefficient_updates;'+from)
code=code.sub('int a=quartic(p,y,v,1,inv,rt,xs,&len);','coefficient_updates=0;int a=quartic(p,y,v,1,inv,rt,xs,&len);')
code=code.sub('    if(a>=0){','    printf("OPS %d %d %d %llu\\n",p,c,mode,coefficient_updates);if(a>=0){')
puts code
puts 'int main(){'
File.foreach(curves){|line|p,c,l,_=line.split.map(&:to_i);puts "assert(audit_catalogue(#{p},#{c},#{l})==7);"}
puts '}'
