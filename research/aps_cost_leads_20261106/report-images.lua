-- Markdown links the vector diagram; LaTeX embeds the same DOT's PDF.
function Image(image)
  if image.src == "pipeline.svg" then
    image.src = "pipeline.pdf"
  end
  return image
end

-- Put the tall flow on its own final page, after the report body. This
-- preserves legible labels without interrupting a paragraph with a float.
function Para(paragraph)
  if #paragraph.content == 1 and paragraph.content[1].t == "Image"
     and paragraph.content[1].src == "pipeline.pdf" then
    return {pandoc.RawBlock("latex", "\\clearpage"), paragraph}
  end
end
