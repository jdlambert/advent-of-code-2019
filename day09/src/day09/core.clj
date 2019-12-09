(ns day09.core
  (:gen-class))

(require '[clojure.string :as str])

(def input
  (zipmap (range) (vec (map read-string
    (str/split (slurp "input.txt")
               #",")))))

(defn get-val [i mode memory rel]
  (case mode
    0 (get memory (get memory i 0) 0)
    1 (get memory i 0)
    2 (get memory (get memory (+ rel i) 0) 0)))

(defn get-addr [i mode memory rel]
  (case mode
    0 (get memory i 0)
    2 (+ rel (get memory (+ rel i) 0))))

(defn execute
  [in]
  (loop [memory input
         i 0
         prints []
         rel 0]
    (let [op (get memory i 0)
          a (get-val (inc i) (mod (quot op 100) 10) memory rel)
          b (get-val (+ 2 i) (mod (quot op 1000) 10) memory rel)
          out (get-addr (+ 3 i) (mod (quot op 10000) 10) memory rel)]
      (case (mod op 100)
        99 (do (println prints) (last prints))
        1 (recur (assoc memory out (+ a b)) (+ 4 i) prints rel)
        2 (recur (assoc memory out (* a b)) (+ 4 i) prints rel)
        3 (let [a (get-addr (inc i) (mod (quot op 100) 10) memory rel)]
            (recur (assoc memory a in) (+ 2 i) prints rel))
        4 (recur memory (+ 2 i) (conj prints a) rel)
        5 (recur memory (if (not= 0 a) b (+ 3 i)) prints rel)
        6 (recur memory (if (= 0 a) b (+ 3 i)) prints rel)
        7 (recur (assoc memory out (if (< a b) 1 0)) (+ 4 i) prints rel)
        8 (recur (assoc memory out (if (= a b) 1 0)) (+ 4 i) prints rel)
        9 (recur memory (+ 2 i) prints (+ a rel))))))

(defn part1 []
  (execute 1))

(defn part2 []
  (execute 2))

(defn -main
  [& args]
  (println "Part 1:" (part1))
  (println "Part 2:" (part2)))
