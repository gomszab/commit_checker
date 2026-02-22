# Hook setup
requirements:
git-bash (windows)
git


curl -fsSL "https://gist.githubusercontent.com/gomszab/56aa1947132d2be70e48fcea2e606a1a/raw/b8d43cc8fb4c0f45e2e2186a135ed5c6133deeb4/setup.sh" | tr -d '\r' | bash -c "bash"

# Use:
git commit -m "message"

# Rules:
- every line should have a comment //
- every variable and property declaration should have a @type
- var keyword can't be used
- in case of @type the jsdoc should have type and description
- the variable, property, function and class names should have at least 5 characters
- the types Object, Array or * can't be used
- typedefs must have a type and a name
- functions and methods must have jsdocs attached to them with the correct "@parameter"s and a @return
- classes must contain a constructor

# Future rules
- every defined function should be used



