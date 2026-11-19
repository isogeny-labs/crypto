def median(values)
  a=values.sort
  (a[(a.size-1)/2]+a[a.size/2])/2.0
end
puts 'seed p input order pencil_original pencil_screen original_screen'
ARGV.sort.each do |file|
  seed,order=File.basename(file,'.log').split('-')
  data={}
  File.foreach(file) do |line|
    next unless line.start_with?('P ')
    g,p,i,r,arm,t=line.split.drop(1).take(6).map(&:to_i)
    key=[p,i,r,arm]
    raise unless g==4 && t>0 && !data.key?(key)
    data[key]=t
  end
  pairs=seed=="2026111929" ? 32 : 8
  raise unless data.size==4*pairs*32*3
  [37,43,59,67].each do |p|
    pairs.times do |i|
      ratios=[[0,1],[0,2],[1,2]].map do |a,b|
        median(32.times.map{|r|data.fetch([p,i,r,a]).fdiv(data.fetch([p,i,r,b]))})
      end
      puts [seed,p,i,order,*ratios].join(' ')
    end
  end
end
