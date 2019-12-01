(ns day01.core
  (:gen-class))

(def input (clojure.string/split-lines (slurp "input.txt")))
(def data (map read-string input))

(defn fuel
      [mass]
      (-> mass
          (/ 3)
          Math/floor
          (- 2)
          int))

(defn part1 []
  (reduce + (map fuel data)))

(defn total-fuel
      [mass]
      (loop [total (fuel mass)
             current (fuel total)]
        (if (<= current 0) 
            total
            (recur (+ total current) (fuel current)))))

(defn part2 []
  (reduce + (map total-fuel data)))

(defn -main
  [& args]
  (println "Part 1:" (part1))
  (println "Part 2:" (part2)))
