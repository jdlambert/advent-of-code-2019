(ns day03.core
  (:gen-class))

(def input
  (->> (slurp "input.txt")
       clojure.string/split-lines
       (map read-string)))

(defn part1 []
  'nil)

(defn part2 []
  'nil)

(defn -main
  [& args]
  (println "Part 1:" (part1))
  (println "Part 2:" (part2)))
