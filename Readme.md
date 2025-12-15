# Hook setup
requirements:
git-bash (windows)
git


curl -fsSL "https://gist.githubusercontent.com/gomszab/56aa1947132d2be70e48fcea2e606a1a/raw/b8d43cc8fb4c0f45e2e2186a135ed5c6133deeb4/setup.sh" | tr -d '\r' | bash -c "bash"

# Use:
git commit -m "message"

# Rules:
- every line should have a comment //
- every variable declaration should have a @type
- var keyword can't be used
- in case of @type the jsdoc should have type and description
- the variable names should have at least 5 character
- the types Object, Array or * can't be used
- typedefs must have a type and a name
- every variable should be used
- every defined function should be used

# Future rules
- jsdoc check improvement: handle @param, @returns
- class definition checker

