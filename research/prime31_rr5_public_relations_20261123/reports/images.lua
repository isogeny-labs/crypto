-- Layout only: constrain the existing SVGs to the report's text width.
function Image(image)
  image.attributes.width = '100%'
  image.attributes.height = nil
  return image
end
