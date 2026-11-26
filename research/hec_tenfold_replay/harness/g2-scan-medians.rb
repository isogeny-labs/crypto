def median(list)
  v=list.sort
  (v[(v.size-1)/2]+v[v.size/2])/2.0
end
crossover=ARGV.delete("--crossover")
ARGV.each do |file|
  timings={}
  File.foreach(file) do |line|
    next unless line.start_with?('P ')
    g,p,c,r,a,t=line.split.drop(1).take(6).map(&:to_i)
    key=[p,c,r,a]
    raise unless g==2 && t>0 && !timings.key?(key)
    timings[key]=t
  end
  fields=File.basename(File.dirname(file))=="character" ? [127,257] : [127,257,1021,4093]
  raise unless timings.size==(crossover ? 2048 : 1024)*fields.size
  fields.each do |p|
    32.times do |c|
      if crossover
        ratios=16.times.map do |r|
          times=4.times.map{|a|timings.fetch([p,c,r,a])}
          (times[0]+times[3]).fdiv(times[1]+times[2])
        end
        puts [File.basename(file,'.log'),p,c,median(ratios)].join(' ')
        next
      end
      ratios=[[0,1],[0,2],[0,3],[1,2],[1,3]].map do |a,b|
        median(8.times.map{|r|timings.fetch([p,c,r,a]).fdiv(timings.fetch([p,c,r,b]))})
      end
      puts [File.basename(file,'.log'),p,c,*ratios].join(' ')
    end
  end
end
