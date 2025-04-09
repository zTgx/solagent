# hatch build
hatch build

# publish
twine check dist/*
twine upload dist/*