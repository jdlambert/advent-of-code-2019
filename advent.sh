#!/usr/bin/env bash

if [ $# -eq 0 ]
then
  echo "Usage: ./advent.sh <day number>"
  exit 1
fi

echo "DAY $1"

cd $(printf "%02d" $1)
[ -f rust.rs ] && printf "\e[31m\nRUST:\n" && rustc rust.rs && (./rust; rm rust)
[ -f clojure.clj ] && printf "\e[32m\nCLOJURE:\n" && clojure clojure.clj