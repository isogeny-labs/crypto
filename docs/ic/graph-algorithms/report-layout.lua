-- Layout only. Mathematical content is authored in README.md and SOURCES.md.
function Table(el)
  local widths = nil
  if #el.colspecs == 3 then widths = {0.24, 0.29, 0.47} end
  if #el.colspecs == 2 then widths = {0.25, 0.75} end
  if widths then
    local specs = el.colspecs
    for i, width in ipairs(widths) do
      specs[i] = {pandoc.AlignLeft, width}
    end
    el.colspecs = specs
  end
  return el
end

function Image(el)
  if el.src == './pipeline.svg' or el.src == 'pipeline.svg' then
    el.attributes.width = '85%'
  end
  return el
end
