-- Keep SVG as the canonical report image, but use Graphviz's vector PDF
-- rendering when Pandoc targets LaTeX/XeLaTeX.  This avoids a dependency on
-- rsvg-convert while preserving vector text and lines in REPORT.pdf.
function Image(image)
  if FORMAT:match("latex") and image.src:match("%.svg$") then
    image.src = image.src:gsub("%.svg$", ".pdf")
  end
  return image
end
