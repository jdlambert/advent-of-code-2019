#!/usr/bin/env bash

if [ $# -eq 0 ]
then
  echo "Usage: ./advent.sh day [-f]"
  echo "-f: creates the folder for the given day from a template"
  exit 1
fi

RED="\e[31m"
GREEN="\e[32m"

echo "DAY $1"
DIR=$(printf "day%02d" $1)

if [[ $# -gt 1 && "$2" == "-f" ]]; then
  [ -d $DIR ] && echo "Directory already exists!" && exit 1
  cp -r day00 $DIR
  mv $DIR/src/day00 $DIR/src/$DIR
  grep -rlI "day00" $DIR | xargs sed -i .sed "s/day00/$DIR/g"
  find $DIR -name "*.sed" -delete
  echo "Directory created!"
  exit 0
fi

cd $DIR || exit 1
[ -f Cargo.toml ] && printf "$RED\nRUST:\n" && cargo fmt && cargo run --quiet
[ -f project.clj ] && printf "$GREEN\nCLOJURE\n" && lein run
