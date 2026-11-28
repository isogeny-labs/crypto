require "zlib"

# Recorded rows use affine x coordinates as column identifiers. FORM stores
# the signed factor-base representation returned for each public divisor.
cases=[]
Zlib::GzipReader.open(ARGV.fetch(0)).each_line do |line|
  label,*a=line.split
  next unless %w[BEGIN ROW FORM].include?(label)
  a.map!(&:to_i)
  case label
  when "BEGIN" then cases << [a,[],[]]
  when "ROW" then p,seq,*terms=a;raise unless p==cases.last[0][0];cases.last[1]<<[seq,terms.each_slice(2).to_h]
  when "FORM" then cases.last[2]<<a.each_slice(2).to_h
  end
end
def reduce(row,basis,l)
  w=row.dup
  basis.keys.sort.reverse_each do |col|
    next unless (f=w[col])
    basis.fetch(col).each{|j,v|t=(w.fetch(j,0)-f*v)%l;t.zero? ? w.delete(j) : w[j]=t}
  end
  w
end
def alternate(p,c,u,v,l)
  return [] unless u.size==4
  places=(0..p/2).to_h{|y|[y*y%p,y]}
  reps=[]
  # For each pencil member divide f - (v + t*u)^2 by the Mumford u.
  # Successive linear divisions retain repeated roots of the residual quartic.
  p.times do |t|
    line=4.times.map{|i|((v[i]||0)+t*u[i])%p}
    f=[1,c,0,1,0,0,0,1]
    line.each_with_index{|a,i|line.each_with_index{|b,j|f[i+j]-=a*b}}
    q=[0]*5
    7.downto(3){|j|k=q[j-3]=f[j]%p;4.times{|i|f[i+j-3]-=k*u[i]}}
    raise unless f.all?{|a|a%p==0}
    form=Hash.new(0)
    p.times do |x|
      break if q.size==1
      while q.size>1 && q.reverse_each.reduce(0){|a,b|(a*x+b)%p}==0
        y=line.reverse_each.reduce(0){|a,b|(a*x+b)%p}
        raise unless places.fetch(y*y%p)==[y,p-y].min
        form[x]+=y<=p/2 ? -1 : 1 unless y==0
        div=[q.last];q[1...-1].reverse_each{|z|div<<((z+x*div.last)%p)}
        q=div.reverse
      end
    end
    reps<<form.transform_values{|v|v%l}.reject{|_,v|v==0} if q.size==1
  end
  reps.uniq
end
def insert(basis,row,l)
  w=reduce(row,basis,l)
  return if w.empty?
  j=w.keys.max;scale=w[j].pow(l-2,l)
  basis[j]=w.transform_values{|x|x*scale%l}
end
inputs=Hash.new{|h,k|h[k]=[]}
File.foreach(ARGV.fetch(1)){|s|next unless s.start_with?("INPUT ");p,c,f=s.split.drop(1).take(3).map(&:to_i);u,v=s.scan(/\[([^\]]*)\]/).flatten.map{|a|a.split(",").map(&:to_i)};inputs[[p,c]]<<[f,u,v]}
puts "p case arms full_rank_prefix base_rows base_poles first_rows first_poles extra_rows representations_D representations_T"
cases.each do |(p,c,l,truth),rows,forms|
  extra=[];counts=[];groups=[]
  raise "missing inputs" unless inputs.fetch([p,c]).size==2
  full={}
  rows.each{|_,row|insert(full,row,l)}
  inputs.fetch([p,c]).each_with_index do |(f,u,v),i|
    start=extra.size;reps=alternate(p,f,u,v,l);counts<<reps.size
    reps.each do |rep|
      diff=rep.dup
      forms[i].each{|j,x|diff[j]=(diff.fetch(j,0)-x)%l}
      diff=diff.reject{|_,x|x==0}
      raise unless reduce(diff,full,l).empty?
      extra<<diff unless diff.empty?
    end
    groups<<extra[start..]
  end
  [[], [0], [1], [0, 1]].each do |arms|
    added = arms.flat_map { |i| groups[i] }
    basis = {}
    added.each { |row| insert(basis, row, l) }
    first_certificate = nil
    full_prefix = nil
    (0..rows.size).each do |prefix|
      insert(basis, rows[prefix-1][1], l) if prefix.positive?
      if first_certificate.nil?
        generator, target = forms.map { |row| reduce(row, basis, l) }
        raise "generator became zero" if generator.empty?
        col, value = generator.first
        scalar = target.fetch(col, 0) * value.pow(l-2, l) % l
        scaled = generator.transform_values { |v| v*scalar % l }.reject { |_,v| v.zero? }
        if scaled == target
          raise "wrong scalar" unless scalar == truth
          first_certificate = prefix
        end
      end
      if basis.size == full.size
        full_prefix = prefix
        break
      end
    end
    raise "missing certificate" unless first_certificate && full_prefix
    poles = first_certificate.zero? ? 0 : rows[first_certificate-1][0]+1
    label = arms.empty? ? "none" : arms.join
    puts [p, c, label, full_prefix, rows.size, rows.last.first+1,
          first_certificate, poles, added.size, *counts].join(" ")
  end
end
