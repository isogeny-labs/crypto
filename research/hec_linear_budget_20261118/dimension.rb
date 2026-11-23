require 'json'
require 'digest'
root=File.expand_path('../..',__dir__)
path=File.join(root,'research/hec_residual_span_20261116/native-pencil-3lp/INSTANCES.tsv')
admission=JSON.parse(File.read(File.join(root,'research/hec_residual_span_20261116/native-kummer-pair-screen/ADMISSION.json')))
rows=File.readlines(path).drop(1).map do |l|
 g,p,c,ell=l.split("\t").take(4).map(&:to_i)
 raise 'prime' unless ell>2&&(2..).take_while{|n|n*n<=ell}.all?{|n|ell%n!=0}
 model=admission.fetch('models').find{|m|m['genus']==g&&m['p']==p} or raise 'model'
 raise 'identity' unless model['c']==c&&model['ell']==ell
 r=1;r+=1 while r*r<p
 upper=(r+1)**(2*(g-1));raise 'notadmitted' unless ell>upper
 # Direct integer extension bounds for all h*d<g, independent of the binomial comparison.
 checks=[]
 (1...g).each do |h|
  (1...g).each do |d|
   next unless h*d<g
   q=p**d;s=1;s+=1 while s*s<q
   u=(s+1)**(2*h);raise 'extensionobstruction' unless ell>u
   checks<<{h:h,d:d,upper:u}
  end
 end
 {genus:g,p:p,c:c,ell:ell,lower_dimension_upper:upper,margin:ell-upper,base_field_simple_by_stated_criterion:true,extension_checks:checks}
end
raise '11models' unless rows.size==11
File.write(File.join(__dir__,'DIMENSION.json'),JSON.pretty_generate({schema:'hec.dimension-obstruction/v1',proof_status:'written derivation from standard theory; independent review open',order_premise:'reused native-kummer-pair-screen source order certification, not recomputed here',instances_sha256:Digest::SHA256.file(path).hexdigest,models:rows})+"\n")
rows.each{|r|puts "g=#{r[:genus]} p=#{r[:p]} ell=#{r[:ell]} lower_dim_upper=#{r[:lower_dimension_upper]} margin=#{r[:margin]} extension_checks=#{r[:extension_checks].size}"}
puts 'PASS 11integer criteria and every h*d<g integer extension bound; source-order premise reused from qualified gate'
