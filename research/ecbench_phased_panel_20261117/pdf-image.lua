-- Keep the web report on SVG while giving LaTeX an equivalent PDF image.
function Image(image)
  if FORMAT:match("latex") and image.src:match("%.svg$") then
    image.src = image.src:gsub("%.svg$", ".pdf")
    image.attributes.height = "8in"
  end
  return image
end

function Code(code)
  if FORMAT:match("latex") and #code.text >= 32 and code.text:match("^[0-9a-f]+$") then
    return pandoc.RawInline("latex", "\\texttt{\\seqsplit{" .. code.text .. "}}")
  end
  return code
end
