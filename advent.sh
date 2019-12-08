#!/usr/bin/env bash
[[ $# -eq 0 ]] && echo "Usage: ./advent.sh day"

echo "🦀 DAY $1 🎄"
DIR=$(printf "day%02d" $1)

RED="\e[31m"
GREEN="\e[32m"

if [ ! -d $DIR ]; then
  URL=https://adventofcode.com/2019/day/$1
  cp -r template $DIR
  mv $DIR/src/template $DIR/src/$DIR
  grep -rlI "template" $DIR | xargs sed -i .sed "s/template/$DIR/g"
  find $DIR -name "*.sed" -delete
  curl -f $URL/input -H "cookie: $(cat cookie)" > $DIR/input.txt 2> /dev/null
  if [ $? ]; then
    echo "Input not available!"
    rm -rf $DIR
    exit 1
  else
    echo "Directory created!"
  fi
fi

cd $DIR
[ -f Cargo.toml ] && printf "$RED\nRUST\n" && cargo fmt && cargo run --quiet
[ -f project.clj ] && printf "$GREEN\nCLOJURE\n" && lein run