require 'json'
d = File.dirname(__FILE__)
a = JSON.parse(File.read(File.join(d,'analysis.json')))
rows = a['extension_test']
x = ->(v) { 190 + (v-0.98)/0.035*540 }
s = ['<svg xmlns="http://www.w3.org/2000/svg" width="820" height="430" viewBox="0 0 820 430"><rect width="820" height="430" fill="white"/><g font-family="sans-serif" font-size="15" fill="#172033"><text x="30" y="32" font-size="21">R13 cold base/candidate ratios: rejected</text><text x="30" y="57">Host 06dc5880 · 40 pairs per set · recorded 95% intervals</text>']
[0.98,0.99,1.00,1.01].each do |v|
 p=x.call(v); s << "<line x1='#{p}' x2='#{p}' y1='78' y2='345' stroke='#{v==1.01 ? '#b42318' : '#ddd'}'/><text x='#{p-18}' y='370'>#{format('%.2f',v)}</text>"
end
rows.each_with_index do |r,i|
 y=100+i*42; color=r['set']=='suite' ? '#2456a6' : '#8a428c'
 s << "<text x='28' y='#{y+5}'>#{r['set']} n#{[37,47,57][i%3]}</text><line x1='#{x.call(r['lo'])}' x2='#{x.call(r['hi'])}' y1='#{y}' y2='#{y}' stroke='#{color}' stroke-width='4'/><circle cx='#{x.call(r['geomean'])}' cy='#{y}' r='5' fill='#{color}'/>"
end
s << '<text x="190" y="401">Ratio &gt; 1 favors candidate; red line: required lower bound 1.01</text></g></svg>'
File.write(File.join(d,'ratios.svg'),s.join("\n"))
