function Image(image)
  image.src = image.src:gsub("%.svg$", ".graph.pdf")
  image.attributes.width = "100%"
  return image
end
