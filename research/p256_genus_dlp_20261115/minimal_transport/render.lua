function Image(el)
  if FORMAT:match('latex') and el.src == 'transport.svg' then
    el.src = 'transport.pdf'
  end
  return el
end
