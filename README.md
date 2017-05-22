# See branches for actual code

# TODO: Add instructions for use

# TODO: Add instructions for development

## Number commits for easy reference
```
git log --oneline "origin/main..${branch}" | nl -v 0 | sed "s/^ */${branch}~/"
```

## Create log files using filter branch
```
branch=abcd1234
rust_version=1.15.1
num_steps="$(git log --oneline "origin/main..${branch}~1" | wc -l | xargs echo)"
git filter-branch -f --tree-filter "rustup run '${rust_version}' cargo run --quiet --color always >'log/${rust_version}/run.log' 2>&1; rm -rf target" "${branch}~${num_steps}..HEAD"
```
