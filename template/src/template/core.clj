(ns template.core
  (:gen-class))

(def input (clojure.string/split-lines (slurp "input.txt")))
(def data (map read-string input))

(defn part1 []
  'nil)

(defn part2 []
  'nil)

(defn -main
  [& args]
  (println "Part 1:" (part1))
  (println "Part 2:" (part2)))