def median(v)
  v=v.sort
  (v[3]+v[4])/2.0
end
carry=ARGV.delete('--carry')
pivots=ARGV.delete('--pivots')
switch=ARGV.delete('--switch') || carry || pivots
quotient=ARGV.delete('--quotient') || switch
puts(carry ? 'p c case order regular_carry pencil_carry pencil_full' : pivots ? 'p c case order replay_carry pencil_carry pencil_replay' : switch ? 'p c case order full_half pencil_half pencil_full' : quotient ? 'p c case order hermite_quotient pencil_quotient pencil_hermite' : 'p c case order screen_hermite pencil_hermite')
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
  raise unless cases.size==(switch ? 394 : 84)
  cases.each do |p,c,i|
    ratios=8.times.map do |r|
      t=runs.fetch([p,c,i,r,2])[0]
      base=runs[[p,c,i,r,0]]
      [runs.fetch([p,c,i,r,1])[0].fdiv(t),base && base[1]==1 ? base[0].fdiv(t) : (base ? 'unsolved' : 'ineligible')]
    end.transpose
    pencil=ratios[1]
    raise unless pencil.all?{|x|x.is_a?(Float)} || pencil.uniq.size==1
    previous=if quotient && pencil.first.is_a?(Float)
      median(8.times.map{|r|runs.fetch([p,c,i,r,0])[0].fdiv(runs.fetch([p,c,i,r,1])[0])})
    else
      pencil.first
    end
    puts [p,c,i,File.basename(file,'.log'),median(ratios[0]),pencil.first.is_a?(Float) ? median(pencil) : pencil.first,*(quotient ? [previous] : [])].join(' ')
  end
end
