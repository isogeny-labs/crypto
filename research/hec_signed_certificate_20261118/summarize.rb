require 'csv'

def panel(path)
  File.read(path).split(/^BEGIN /).drop(1).to_h do |block|
    id = block.lines.first.strip
    phases = block.lines.grep(/^PHASE /)
    finish = block.lines.grep(/^END /).fetch(0)
    totals = block.scan(/^TOTAL modulus=\d+ vector=\[(.*?)\]/)
    raise id unless totals.size == 2 && finish.include?('solved=true')
    [id, [finish, phases, totals.sum { |v| v.first.split(',').fetch(8).to_i }]]
  end
end

prior = '../hec_phase_budget_20261118'
reference = panel("#{prior}/CONTROL.txt")
candidate = panel('CONTROL.txt')
raise unless candidate.keys == reference.keys && candidate.size == 48
puts %w[g p case original prior signed].join("\t")
CSV.foreach("#{prior}/BUDGET.tsv", headers: true, col_sep: "\t").zip(reference.keys).each do |row, id|
  key = %w[g p case].map { |field| "#{field}=#{row[field]}" }.join(' ')
  raise key unless id.start_with?(key + ' ')
  finish, phases, total = candidate.fetch(id)
  old_finish, old_phases, old_total = reference.fetch(id)
  stable = ->(v) { v.reject { |line| line.start_with?('PHASE name=eligibility ') } }
  raise id unless finish == old_finish && stable.call(phases) == stable.call(old_phases)
  raise id unless old_total == row['current_redc'].to_i
  puts [row['g'], row['p'], row['case'], row['original_redc'], old_total, total].join("\t")
end
