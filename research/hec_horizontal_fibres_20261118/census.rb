require "csv"
fixtures = File.join(__dir__, "../hec_residual_span_20261116/native-pencil-3lp/INSTANCES.tsv")
puts %w[g p catalogue small complete eligible all_small].join("\t")
CSV.foreach(fixtures, headers: true, col_sep: "\t") do |row|
  g, p = row.values_at("g", "p").map(&:to_i)
  f = row["coefficients"].split(",").map(&:to_i)
  raise unless f.size == 2*g+2 && f.last == 1
  fibres = Hash.new { |h,k| h[k] = [] }
  (0...p).each do |x|
    square = f.reverse_each.reduce(0) { |v,c| (v*x+c) % p }
    fibres[square] << x
  end
  catalogue = (0..p/2).flat_map { |b| fibres[b*b % p] }.sort
  m = (1..catalogue.size).find { |n| n**g >= catalogue.size**(g-1) }
  cutoff = catalogue.fetch(m-1)
  counts = [0,0,0]
  (1..p/2).each do |b|
    roots = fibres[b*b % p]
    residual = f.dup
    residual[0] -= b*b
    roots.each do |x|
      loop do
        quotient = []
        carry = residual.last
        residual[0...-1].reverse_each do |coef|
          quotient << carry
          carry = (coef + carry*x) % p
        end
        break unless carry.zero?
        residual = quotient.reverse
      end
    end
    next unless residual == [1]
    large = roots.count { |x| x > cutoff }
    counts[0] += 1
    counts[1] += 1 if large <= 2
    counts[2] += 1 if large.zero?
  end
  puts [g,p,catalogue.size,m,*counts].join("\t")
end
