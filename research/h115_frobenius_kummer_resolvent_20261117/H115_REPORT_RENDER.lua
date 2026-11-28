-- Deterministic Pandoc layout filter for H115_REPORT.html.
-- The HTML remains the editable report source; this filter only supplies
-- print-specific page breaks and column widths to the XeLaTeX backend.

local page_index = 0
local page_total = 3

function Meta(meta)
  -- The HTML <title> is document metadata, not an extra printed title.
  meta.title = nil
  return meta
end

function Div(div)
  if div.classes:includes("footer") then
    return {}
  end
  if div.classes:includes("page") then
    page_index = page_index + 1
    if page_index < page_total then
      div.content:insert(pandoc.RawBlock("latex", "\\newpage"))
    end
  end
  return div
end

function Table(tbl)
  local n = #tbl.colspecs
  local widths
  if n == 7 then
    widths = {0.07, 0.16, 0.07, 0.07, 0.19, 0.20, 0.20}
  elseif n == 4 then
    widths = {0.15, 0.11, 0.35, 0.35}
  else
    return tbl
  end
  for i = 1, n do
    tbl.colspecs[i][2] = widths[i]
  end
  return tbl
end
