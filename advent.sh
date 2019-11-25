#!/usr/bin/env bash

if [ $# -eq 0 ]
then
  echo "Usage: ./advent.sh <day number>"
  exit 1
fi

RED="\e[31m"
GREEN="\e[32m"

echo "DAY $1"

cd $(printf "day%02d" $1)
[ -f Cargo.toml ] && printf "$RED\nRUST:\n" && cargo fmt && cargo run --quiet
[ -f project.clj ] && printf "$GREEN\nCLOJURE:\n" && lein run
