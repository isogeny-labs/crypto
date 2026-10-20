-- Permit long literal hashes and filenames to wrap without changing their text.
function Code(el)
  if FORMAT:match('latex') and #el.text > 20 and el.text:match('^[%w%._/%-]+$') then
    local parts = {}
    for char in el.text:gmatch('.') do
      if char == '_' then char = '\\_' end
      table.insert(parts, char .. '\\allowbreak{}')
    end
    return pandoc.RawInline('latex', '\\texttt{' .. table.concat(parts) .. '}')
  end
end
