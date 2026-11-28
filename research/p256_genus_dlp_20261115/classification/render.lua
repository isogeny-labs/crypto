function Image(image)
  if FORMAT:match('latex') and image.src == 'framework.svg' then
    image.src = 'framework.pdf'
  end
  return image
end
