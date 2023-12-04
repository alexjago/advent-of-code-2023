#! /usr/bin/env zsh

# setopt verbose

daynum=$(( $(basename day-*/ | sed -e 's/day-//' | tail -n 1) + 1))

printf -v newday -- 'day-%02d' "$daynum"

cp -R template "$newday"

sed -e "s/template/$newday/" -i 'bak' "$newday/Cargo.toml"

sed -e "s/^version = \"(\d+)\.\d+\.\d+\"$/version = i\"i\\1.$daynum.0\"" -i bak Cargo.toml

git add .
git commit -am "day $daynum"
