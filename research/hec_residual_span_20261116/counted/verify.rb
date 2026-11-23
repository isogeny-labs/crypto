root='research/hec_residual_span_20261116/counted'
Dir['crates/hec-native-ic/src/*.rs'].each do |original|
 s=File.read("#{root}/native/src/#{File.basename(original)}").lines.reject{|l|l.include?('// COUNTED')}.join
 s=s.sub(/\npub mod counted;\n\z/,'') if File.basename(original)=='lib.rs'
 raise "arithmetic source mismatch #{original}" unless s==File.read(original)
end
puts 'All native source files reverse byte-for-byte to originals.'
