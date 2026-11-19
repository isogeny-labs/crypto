def median(values)
  values=values.sort
  (values[15]+values[16])/2.0
end
puts 'p c case order pencil_original pencil_packed pencil_four'
pairs=ARGV.delete("--expanded") ? 32 : 16
ARGV.each do |file|
  times={}
  masks={}
  File.foreach(file) do |line|
    if line.start_with?('CAT ')
      p,c,mask=line.split.drop(1).map(&:to_i)
      masks[[p,c]]=mask
    elsif line.start_with?('P ')
      p,c,i,r,a,t=line.split.drop(1).take(6).map(&:to_i)
      key=[p,c,i,r,a]
      raise if t<=0 || times.key?(key)
      times[key]=t
    end
  end
  raise unless masks.size==4
  masks.each do |(p,c),mask|
    pairs.times do |i|
      ratios=3.times.map do |a|
        next 'rank-deficient' if mask[a]==0
        median(32.times.map{|r|times.fetch([p,c,i,r,0]).fdiv(times.fetch([p,c,i,r,a+1]))})
      end
      puts [p,c,i,File.basename(file,'.log'),*ratios].join(' ')
    end
  end
  raise unless times.size==masks.values.sum{|m|1+m.digits(2).sum}*pairs*32
end
