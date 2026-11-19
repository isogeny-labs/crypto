require 'zlib'

def read_rows(lines)
  matrices = {}
  active = nil
  lines.each_line do |line|
    tag, *numbers = line.split
    case tag
    when 'BEGIN'
      prime, case_id = numbers.map(&:to_i)
      active = case_id.to_i.zero? ? (matrices[prime] ||= []) : nil
    when 'ROW'
      active << numbers.drop(2).map(&:to_i).each_slice(2).to_h if active
    end
  end
  matrices
end

# Remove a pivot equation and its variable. Removing the
# equation reduces rank by exactly one. Store it for reconstruction.
def filter(matrix, columns, ell, max_degree)
  rows = matrix.map(&:dup)
  cols = columns.dup
  history = []
  updates = 0
  loop do
    incidence = Hash.new { |h,k| h[k] = [] }
    rows.each_with_index { |row,i| row.each_key { |c| incidence[c] << i } }
    if max_degree == :bounded_fill
      candidates = []
      cols.each do |c|
        ids = incidence[c]
        next if ids.empty?
        ids.each do |i|
          # Exact structural upper bound on added coefficients, ignoring
          # cancellations. Removing the pivot row and column pays for overlap.
          eq = rows[i]
          budget = eq.size + ids.size - 1
          ids.each do |h|
            next if h == i
            eq.each_key { |j| budget -= 1 unless rows[h].key?(j) }
            break if budget < 0
          end
          next if budget < 0
          candidates << [(eq.size - 1)*(ids.size - 1), -budget, c, i]
        end
      end
      break if candidates.empty?
      _, _, col, pivot = candidates.min
      holders = incidence[col]
    else
      options = cols.select { |c| (1..max_degree).cover?(incidence[c].size) }
      break if options.empty?
      col = options.min_by { |c| [incidence[c].size, incidence[c].map { |i| rows[i].size }.min, c] }
      holders = incidence[col]
      pivot = holders.min_by { |i| [rows[i].size, i] }
    end
    equation = rows[pivot]
    inverse = equation.fetch(col).pow(ell-2,ell)
    (holders - [pivot]).each do |i|
      scale = rows[i].delete(col) * inverse % ell
      equation.each do |j,v|
        next if j == col
        t = (rows[i].fetch(j,0) - scale*v) % ell
        t.zero? ? rows[i].delete(j) : rows[i][j] = t
        updates += 1
      end
    end
    history << [col,equation,inverse,holders.size]
    rows.delete_at(pivot)
    cols.delete(col)
  end
  [rows.reject(&:empty?), cols, history, updates]
end

def solve(rows, cols, ell)
  pivots = {}
  rows.each do |row|
    w = row.dup
    until w.empty?
      c = w.keys.max
      a = w.fetch(c)
      if !pivots.key?(c)
        inv = a.pow(ell-2,ell)
        pivots[c] = w.transform_values { |v| v*inv % ell }
        break
      end
      pivots.fetch(c).each do |j,v|
        t = (w.fetch(j,0) - a*v) % ell
        t.zero? ? w.delete(j) : w[j] = t
      end
    end
  end
  free = cols - pivots.keys
  raise 'expected one-dimensional kernel' unless free.size == 1
  solution = {free.first => 1}
  pivots.keys.sort.each do |c|
    sum = pivots.fetch(c).sum { |j,v| j == c ? 0 : v*solution.fetch(j) }
    solution[c] = -sum % ell
  end
  solution
end

moduli = {73=>73277,127=>2214137,257=>5561089,383=>19226639,37=>520447,43=>1020619,59=>1011001,67=>2804327}
three = Zlib::GzipReader.open(ARGV.fetch(0)) { |io| read_rows(io) }
four = File.open(ARGV.fetch(1)) { |io| read_rows(io) }
raise 'missing matrices' unless three.size==4 && four.size==4
puts 'genus prime degree rows cols nnz core_rows core_cols core_nnz singletons merges higher_degree updates'
[[3,three],[4,four]].each do |genus, matrices|
  matrices.each do |p,rows|
    cols = rows.flat_map(&:keys).uniq.sort
    l = moduli.fetch(p)
    reference = solve(rows,cols,l)
    [1,2,:bounded_fill].each do |degree|
      core, keep, history, updates = filter(rows,cols,l,degree)
      x = solve(core,keep,l)
      history.reverse_each do |c,row,inv|
        x[c] = -inv * row.sum { |j,v| j==c ? 0 : v*x.fetch(j) } % l
      end
      raise 'invalid kernel' unless rows.all? { |r| (r.sum { |j,v| v*x.fetch(j) } % l).zero? }
      j = reference.key(1)
      raise 'wrong projective solution' unless x.fetch(j)!=0 && cols.all? { |c| x.fetch(c)==reference.fetch(c)*x.fetch(j)%l }
      puts [genus,p,degree,rows.size,cols.size,rows.sum(&:size),core.size,keep.size,core.sum(&:size),
            history.count { |e| e[3]==1 },history.count { |e| e[3]==2 },history.count { |e| e[3]>2 },updates].join(' ')
    end
  end
end
