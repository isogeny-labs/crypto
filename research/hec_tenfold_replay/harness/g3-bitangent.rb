require 'open3'
rows=File.readlines(ARGV.fetch(0)).map{|r|r.split.map(&:to_i)}
s,status=Open3.capture2('ruby','harness/g3-transfer-kernels.rb',ARGV.first)
raise unless status.success?
s=s.sub('static int coefficient,active;'){File.read('harness/normalized-cubic.inc')+File.read('harness/g3-bitangent.inc')+"\nstatic int coefficient,active;"}
s=s.sub('fused::dual=i==2','fused::dual=true')
rows.each do |p,c,l,_|
  key="case #{p*512+c}: if(active){"
  raise unless s.include?(key)
  s=s.sub(key,"case #{p*512+c}: if(active)return bitangent<#{p},#{c},#{l}>(y,v,i,r,x,n,fused::validation,active==1); if(true){")
end
s=s.sub('for(int j=0;j<p;j++)assert(1LL*v[j]', 'for(int j=0;j<p;j++)if(y[j])assert(1LL*v[j]')
puts s
if ARGV.include?('audit')
  puts 'int main(){'+rows.map{|p,c,l,_|"printf(\"STATUS #{p} #{c} %d\\n\",audit_catalogue(#{p},#{c},#{l}));"}.join+'}'
end
