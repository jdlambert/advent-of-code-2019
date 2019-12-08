#!/usr/bin/env bash

function print_usage() {
  echo "Usage: ./advent.sh day [-f/-r]"
  echo "-f: creates the folder for the given day from a template"
  echo "-r: starts a Leiningen REPL with the given week's namespace"
  exit 1
}

[[ $# -eq 0 ]] && print_usage

echo "🦀 DAY $1 🎄"
DIR=$(printf "day%02d" $1)

if [[ $# -gt 1 ]]; then
  case "$2" in
    "-f" )
      [ -d $DIR ] && echo "Directory already exists!" && exit 1
      URL=https://adventofcode.com/2019/day/$1
      cp -r template $DIR
      mv $DIR/src/template $DIR/src/$DIR
      grep -rlI "template" $DIR | xargs sed -i .sed "s/template/$DIR/g"
      find $DIR -name "*.sed" -delete
      curl $URL/input -H "cookie: $(cat cookie)" > $DIR/input.txt
      echo "Directory created!"
      exit 0
      ;;
    "-r" )
      cd $DIR || exit 1
      lein repl
      ;;
    * )
      print_usage
  esac
fi

RED="\e[31m"
GREEN="\e[32m"

cd $DIR || exit 1
[ -f Cargo.toml ] && printf "$RED\nRUST\n" && cargo fmt && cargo run --quiet
[ -f project.clj ] && printf "$GREEN\nCLOJURE\n" && lein run
