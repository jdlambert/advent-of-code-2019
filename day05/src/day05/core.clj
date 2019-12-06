(ns day05.core
  (:gen-class))

(require '[clojure.string :as str])

(def input
  (vec (map read-string 
       (str/split (slurp "input.txt")
                  #","))))

(defn execute
    [in]
    (loop [memory input
           i 0
           prints []]
      (let [op (memory i)
            a-i (= 1 (mod (quot op 100) 10))
            b-i (= 1 (mod (quot op 1000) 10))
            a (get memory (if a-i (inc i) (get memory (inc i))))
            b (get memory (if b-i (+ 2 i) (get memory (+ 2 i))))
            out (get memory (+ 3 i))]
            (case (mod op 10)
                  1 (recur (assoc memory out (+ a b)) (+ 4 i) prints)
                  2 (recur (assoc memory out (* a b)) (+ 4 i) prints)
                  3 (recur (assoc memory (get memory (inc i)) in) (+ 2 i) prints)
                  4 (recur memory (+ 2 i) (conj prints a))
                  5 (recur memory (if (not= 0 a) b (+ 3 i)) prints)
                  6 (recur memory (if (= 0 a) b (+ 3 i)) prints)
                  7 (recur (assoc memory out (if (< a b) 1 0)) (+ 4 i) prints)
                  8 (recur (assoc memory out (if (= a b) 1 0)) (+ 4 i) prints)
                  9 (last prints)))))

(defn part1 []
  (execute 1))

(defn part2 []
  (execute 5))

(defn -main
  [& args]
  (println "Part 1:" (part1))
  (println "Part 2:" (part2)))
