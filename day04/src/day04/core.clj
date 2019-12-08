(ns day04.core
  (:gen-class))

(defn monotonic [code]
  (= code (clojure.string/join (sort code))))

(def input
  (->> (slurp "input.txt")
       (re-seq #"\d+")
       (map read-string)
       (#(range (first %) (inc (second %))))
       (map str)
       (filter monotonic))) ;; Used in both parts, might as well do it here

(defn counts [code]
  (set (vals (frequencies code))))

(defn part1 []
  (->> input
       (filter #(not= (counts %) #{1}))
       (count)))

(defn part2 []
  (->> input
       (filter #((counts %) 2))
       (count)))

(defn -main
  [& args]
  (println "Part 1:" (part1))
  (println "Part 2:" (part2)))
