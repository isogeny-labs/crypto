MODULUS={73=>73277,127=>2214137,257=>5561089,383=>19226639}
def insert(row, basis, ell, minimum)
  loop do
    pivot=row.keys.max
    return false unless pivot && pivot>=minimum
    a=row.fetch(pivot)
    if (equation=basis[pivot])
      equation.each do |j,t|
        value=(row.fetch(j,0)-a*t)%ell
        value.zero? ? row.delete(j) : row[j]=value
      end
    else
      inv=a.pow(ell-2,ell)
      basis[pivot]=row.transform_values{|v|v*inv%ell}
      return true
    end
  end
end
data=Hash.new{|h,k|h[k]=[]}
values={}
STDIN.each_line do |line|
  label,*a=line.split
  p,*a=a.map(&:to_i)
  if label=='V'
    values[p]=a.each_slice(2).to_h
  else
    raise unless %w[F Q].include?(label)
    fibre,poles,degree,*terms=a
    work=Hash.new(0)
    terms.each_slice(2){|j,v|work[j]+=v}
    data[p]<<[fibre,poles,degree,work]
  end
end
raise unless values.keys.sort==MODULUS.keys.sort
puts 'prime max_quadratics baseline_fibres first_fibre poles singles doubles closed independent single_at_stop double_at_stop closed_at_stop independent_at_stop'
MODULUS.each do |p,ell|
  rows=data.fetch(p)
  logs=values.fetch(p)
  [0,1,2].each do |cap|
    places={};rational={}
    singles=doubles=closed=gained=0
    stop=nil
    rows.each do |fibre,poles,degree,coeff|
      next if degree>cap
      singles+=1 if degree==1
      doubles+=1 if degree==2
      w=coeff.transform_values{|v|v%ell}.reject{|_,v|v.zero?}
      next if degree>0 && insert(w,places,ell,p)
      closed+=1 if degree>0
      raise 'relation disagrees with catalogue' unless w.sum{|j,v|logs.fetch(j)*v}%ell==0
      if insert(w,rational,ell,0)
        gained+=1 if degree>0
        stop||=[fibre,poles,singles,doubles,closed,gained] if rational.size==logs.size-1
      end
    end
    raise 'rank deficit' unless stop
    puts [p,cap,rows.last.first,*stop.first(2),singles,doubles,closed,gained,*stop.drop(2)].join(' ')
  end
end
