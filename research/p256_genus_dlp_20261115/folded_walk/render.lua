function Image(el)
  if FORMAT:match('latex') and el.src == 'walk.svg' then
    el.src = 'walk.pdf'
  end
  return el
end
