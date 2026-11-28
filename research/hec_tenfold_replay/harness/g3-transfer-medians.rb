def median(samples)
  a=samples.sort
  (a[(a.size-1)/2]+a[a.size/2])/2.0
end
datasets={}
ARGV.each do |file|
  stage,order=File.basename(file,'.log').split('-')
  h={}
  File.foreach(file) do |line|
    next unless line.start_with?('P ')
    p,c,k,r,a,t,ok=line.split.drop(1).map(&:to_i)
    key=[p,c,k,r,a]
    raise "invalid or duplicate timing" unless ok==1 && t>0 && !h.key?(key)
    h[key]=t
  end
  raise "incomplete timing stream" unless h.size==8192
  datasets[[stage,order]]=h
end
puts "p c case order pencil_original pencil_combined original_combined crossover_original_combined"
datasets.fetch(['panel','forward']).keys.map{|k|k.take(3)}.uniq.sort.each do |p,c,k|
  %w[forward reverse].each do |o|
    h=datasets.fetch(['panel',o]); pair=datasets.fetch(['crossover',o])
    ratios=[[0,1],[0,3],[1,3]].map do |a,b|
      median(32.times.map{|r|h.fetch([p,c,k,r,a]).fdiv(h.fetch([p,c,k,r,b]))})
    end
    cross=median(32.times.map do |r|
      t=4.times.map{|j|pair.fetch([p,c,k,r,j])}
      (t[0]+t[3]).fdiv(t[1]+t[2])
    end)
    puts [p,c,k,o,*ratios,cross].join(' ')
  end
end
