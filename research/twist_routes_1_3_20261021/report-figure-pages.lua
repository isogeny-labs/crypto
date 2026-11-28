-- Keep the text together; append each original vector figure as its own page.
function Image(figure)
  return pandoc.Str("Figure on the following pages: " .. pandoc.utils.stringify(figure.caption) .. ".")
end

function Figure(figure)
  return pandoc.Para({pandoc.Str("Vector figures and editable data accompany this report on the following pages.")})
end
