(ns day01.core
  (:gen-class))

(def input (clojure.string/split-lines (slurp "input.txt")))
(def data (map read-string input))

(defn fuel
      [mass]
      (-> mass
          (/ 3)
          Math/floor
          int
          (- 2)))

(defn part1 []
  (reduce + (map fuel data)))

(defn total-fuel
  [mass]
  (->> (fuel mass)
       (iterate fuel)
       (take-while pos?)
       (reduce +)))

(defn part2 []
  (reduce + (map total-fuel data)))

(defn -main
  [& args]
  (println "Part 1:" (part1))
  (println "Part 2:" (part2)))
