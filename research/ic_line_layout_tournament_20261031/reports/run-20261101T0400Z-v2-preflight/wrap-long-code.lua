-- Preserve exact long identities in the Markdown source while allowing the
-- generated LaTeX/PDF artifact to wrap them at neutral visual boundaries.
local function latex_escape(value)
  value = value:gsub("\\", "\\textbackslash{}")
  value = value:gsub("([%%#$&{}_])", "\\%1")
  value = value:gsub("%^", "\\textasciicircum{}")
  value = value:gsub("~", "\\textasciitilde{}")
  return value
end

local function wrap_chunks(value)
  local chunks = {}
  for offset = 1, #value, 12 do
    table.insert(chunks, latex_escape(value:sub(offset, offset + 11)))
  end
  return table.concat(chunks, "\\allowbreak{}")
end

function Code(element)
  if FORMAT:match("latex") and #element.text > 28 then
    return pandoc.RawInline(
      "latex",
      "\\texttt{" .. wrap_chunks(element.text) .. "}"
    )
  end
end
