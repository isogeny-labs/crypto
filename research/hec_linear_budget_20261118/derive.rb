require 'csv'
require 'digest'
require 'json'
root=File.expand_path('../..',__dir__)
source=File.join(root,'research/hec_residual_span_20261116/native-lazy-finisher/COMPARISONS.tsv')
raw=File.join(root,'research/hec_residual_span_20261116/native-lazy-finisher/CONTROL.txt')
rows=CSV.read(source,headers:true,col_sep:"\t").select{|r|r['control']=='normalized'}
raise '44fieldrecords' unless rows.size==44
rawlines=File.readlines(raw)
out=[]
rows.group_by{|r|%w[g p sm case].map{|k|r[k]}}.each do |key,records|
 fields={}
 records.each do |r|
  raise 'duplicatefield' if fields.key?(r['field'])
  g,p,sm,kase=key
  pattern="COUNT g=#{g} p=#{p} sm=#{sm} case=#{kase} arm=normalized rep=0 modulus=#{r['modulus']} vector="
  line=rawlines.find{|l|l.start_with?(pattern)} or raise 'missing raw vector'
  v=JSON.parse(line.split('vector=',2)[1]);raise 'full16' unless v.size==16
  raise 'redcidentity' unless v[8]==2*v[7]+v[5]+v[6]
  value=Integer(r['control_redc']);raise 'rawreplay' unless value==v[8]
  fields[r['field']]=value
 end
 a=fields.fetch('base');b=fields.fetch('subgroup');raise 'positive' unless a>0&&b>0
 q=Rational(9*a,b)
 # Favorable zero-cost subgroup work, all unchanged base cost retained.
 [Rational(0),Rational(1),Rational(10),q,q+1].each do |price|
  ratio=Rational(a,1)/a+price*Rational(b,a)
  raise 'threshold identity' unless ((ratio>=10)==(price>=q))
 end
 out<<{g:Integer(key[0]),p:Integer(key[1]),source_multiple:Integer(key[2]),case:Integer(key[3]),base_redc:a,subgroup_redc:b,zero_subgroup_equal_price_ceiling:Rational(a+b,a).to_s,minimum_subgroup_price_for_tenfold:q.to_s,tenfold_possible_at_price_1:(a+b>=10*a),tenfold_possible_at_price_10:(a+10*b>=10*a)}
end
raise '22fixtures' unless out.size==22
File.write(File.join(__dir__,'BOUNDS.json'),JSON.pretty_generate({schema:'hec.linear-budget/v1',scope:'conditional fixed base work and zero subgroup cost; REDC price family only',source_sha256:Digest::SHA256.file(source).hexdigest,raw_sha256:Digest::SHA256.file(raw).hexdigest,rows:out})+"\n")
File.open(File.join(__dir__,'BOUNDS.tsv'),'w')do |f|
 f.puts "g\tp\tsource_multiple\tcase\tbase_redc\tsubgroup_redc\tzero_subgroup_equal_price_ceiling\tminimum_subgroup_price_for_tenfold"
 out.each{|r|f.puts r.values_at(:g,:p,:source_multiple,:case,:base_redc,:subgroup_redc,:zero_subgroup_equal_price_ceiling,:minimum_subgroup_price_for_tenfold).join("\t")}
end
puts "PASS raw_full16_checks=44 fixtures=#{out.size} equal_price_tenfold=#{out.count{|r|r[:tenfold_possible_at_price_1]}} price10_tenfold=#{out.count{|r|r[:tenfold_possible_at_price_10]}}"
out.group_by{|r|r[:g]}.each{|g,rr|puts "g=#{g} equal_price_ceiling=#{rr.map{|r|Rational(r[:zero_subgroup_equal_price_ceiling])}.minmax.map(&:to_f)} required_price=#{rr.map{|r|Rational(r[:minimum_subgroup_price_for_tenfold])}.minmax.map(&:to_f)}"}
