# Exact bookkeeping on committed raw field-count records; no solver execution.
require 'json'
require 'csv'
require 'digest'
BASE=File.expand_path('..',__dir__)
def records(stage)
 rows=File.readlines("#{BASE}/#{stage}/CONTROL.txt").grep(/^COUNT /).map do |line|
  h=line.split(' vector=').first.split.drop(1).map{|s|s.split('=',2)}.to_h
  next unless h['rep']=='0'
  h['v']=JSON.parse(line.split('vector=').last);h
 end.compact
 out={}
 rows.each do |h|
  key=%w[g p source_multiple case].map{|k|Integer(h.fetch(k))}+[h.fetch('arm')]
  out[key]||={}; modulus=Integer(h.fetch('modulus'));raise 'duplicate vector' if out[key].key?(modulus)
  out[key][modulus]=h['v']
 end
 out
end
def condition(da,db)
 if db==0
  [da>=0 ? 'all_positive_prices' : 'no_positive_price','']
 elsif db>0
  da>=0 ? ['all_positive_prices',''] : ['price_at_least',Rational(-da,db).to_s]
 else
  da>0 ? ['price_at_most',Rational(da,-db).to_s] : ['no_positive_price','']
 end
end
candidate=records('native-affine-forest');raise unless candidate.size==22
stages=%w[native-public-2lp-cost native-prepared-solvers native-deferred-membership native-stream-target]
rows=[]
stages.each do |stage|
 records(stage).each do |key,old|
  g,p,sm,ci,arm=key;new=candidate.fetch([g,p,sm,ci,'affine_forest'])
  raise unless new.keys.sort==old.keys.sort&&new.size==2
  ell=(new.keys-[p]).fetch(0);a,b=old[p][8],old[ell][8];c,d=new[p][8],new[ell][8]
  ra,rb=Rational(a,c),Rational(b,d);kind,bound=condition(a-10*c,b-10*d)
  # Algebraic formula is independently checked at rational samples, not a proof by sampling.
  [Rational(1,100),Rational(1,1),Rational(100,1)].each do |r|
   expected=(a+r*b)>=10*(c+r*d)
   actual=case kind;when 'all_positive_prices';true;when 'no_positive_price';false;when 'price_at_least';r>=Rational(bound);when 'price_at_most';r<=Rational(bound);end
   raise unless expected==actual
  end
  rows<<[g,p,sm,ci,ell,stage,arm,a,b,c,d,ra.to_f,rb.to_f,[ra,rb].max.to_f,kind,bound]
 end
end
headers=%w[g p sm case ell control_stage control_arm control_base_redc control_subgroup_redc forest_base_redc forest_subgroup_redc base_ratio subgroup_ratio price_independent_upper tenfold_condition price_bound_exact]
CSV.open("#{__dir__}/ENVELOPES.tsv",'w',col_sep:"\t"){|csv|csv<<headers;rows.each{|r|csv<<r}}
rows.group_by{|r|[r[5],r[6]]}.each{|key,v|puts "#{key.join('/')} pairs=#{v.size} all_prices=#{v.count{|r|r[-2]=='all_positive_prices'}} no_price=#{v.count{|r|r[-2]=='no_positive_price'}} upper_range=#{v.map{|r|r[-3]}.min}..#{v.map{|r|r[-3]}.max}"}
paths=stages.map{|s|"#{BASE}/#{s}/CONTROL.txt"}+["#{BASE}/native-affine-forest/CONTROL.txt",__FILE__]
File.write("#{__dir__}/INPUT.sha256",paths.map{|p|"#{Digest::SHA256.file(p).hexdigest}  #{p}\n"}.join)
raise unless rows.size==264
