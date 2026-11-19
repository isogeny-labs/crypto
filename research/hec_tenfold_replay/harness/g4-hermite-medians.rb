def median(v)
  v=v.sort
  (v[3]+v[4])/2.0
end
puts 'p c case order screen_hermite pencil_hermite'
ARGV.each do |file|
  runs={}
  File.foreach(file) do |line|
    next unless line.start_with?('P ')
    p,c,i,r,a,t,_,_,_,_,ok=line.split.drop(1).map(&:to_i)
    key=[p,c,i,r,a]
    raise if runs.key?(key) || t<=0 || (a>0 && ok!=1)
    runs[key]=[t,ok]
  end
  cases=runs.keys.map{|k|k.take(3)}.uniq.sort
  raise unless cases.size==84
  cases.each do |p,c,i|
    ratios=8.times.map do |r|
      t=runs.fetch([p,c,i,r,2])[0]
      base=runs[[p,c,i,r,0]]
      [runs.fetch([p,c,i,r,1])[0].fdiv(t),base && base[1]==1 ? base[0].fdiv(t) : (base ? 'unsolved' : 'ineligible')]
    end.transpose
    pencil=ratios[1]
    raise unless pencil.all?{|x|x.is_a?(Float)} || pencil.uniq.size==1
    puts [p,c,i,File.basename(file,'.log'),median(ratios[0]),pencil.first.is_a?(Float) ? median(pencil) : pencil.first].join(' ')
  end
end
