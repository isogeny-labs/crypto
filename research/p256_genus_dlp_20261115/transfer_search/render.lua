function Image(el)
  if FORMAT:match('latex') and el.src == 'routes.svg' then
    el.src = 'routes.pdf'
  end
  return el
end
