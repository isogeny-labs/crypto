-- Embed the PDF produced from the same DOT; Markdown keeps its SVG link.
function Image(image)
  if image.src == "leads.svg" then
    image.src = "leads.pdf"
  end
  return image
end
