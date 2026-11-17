-- Give the descriptive column enough width in the proposal index.
function Table(t)
  if FORMAT:match('latex') and #t.colspecs == 5 then
    local widths = {0.07, 0.53, 0.10, 0.07, 0.23}
    for i, spec in ipairs(t.colspecs) do
      spec[2] = widths[i]
    end
  end
  return t
end
