#!/usr/bin/env bash

if [ $# -eq 0 ]
then
  echo "Usage: ./advent.sh <day number>"
  exit 1
fi

RED="\e[31m"
GREEN="\e[32m"

echo "DAY $1"

cd $(printf "%02d" $1)
[ -f rust.rs ] && printf "$RED\nRUST:\n" && rustc rust.rs && (./rust; rm rust)
[ -f clojure.clj ] && printf "$GREEN\nCLOJURE:\n" && clojure clojure.clj