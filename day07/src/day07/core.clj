(ns day07.core
  (:gen-class))

(require '[clojure.math.combinatorics :as combo])
(require '[clojure.string :as str])
(require '[clojure.core.async :as async :refer [>! <! >!! <!! go-loop chan]])

(def input
  (vec (map read-string 
       (str/split (slurp "input.txt")
                  #","))))

(defn execute
    [in-c out-c]
    (go-loop [memory input i 0 rv 0]
      (let [op (memory i)
            a-i (= 1 (mod (quot op 100) 10))
            b-i (= 1 (mod (quot op 1000) 10))
            a (get memory (if a-i (inc i) (get memory (inc i))))
            b (get memory (if b-i (+ 2 i) (get memory (+ 2 i))))
            out (get memory (+ 3 i))]
            (case (mod op 10)
                  1 (recur (assoc memory out (+ a b)) (+ 4 i) rv)
                  2 (recur (assoc memory out (* a b)) (+ 4 i) rv)
                  3 (recur (assoc memory (get memory (inc i)) (<! in-c)) (+ 2 i) rv)
                  4 (do (>! out-c a) (recur memory (+ 2 i) a))
                  5 (recur memory (if (not= 0 a) b (+ 3 i)) rv)
                  6 (recur memory (if (= 0 a) b (+ 3 i)) rv)
                  7 (recur (assoc memory out (if (< a b) 1 0)) (+ 4 i) rv)
                  8 (recur (assoc memory out (if (= a b) 1 0)) (+ 4 i) rv)
                  9 rv))))

(defn get-output [[a b c d e]]
  (let [ea (chan 2) ab (chan 2) bc (chan 2) cd (chan 2) de (chan 2)]
        (>!! ea a) (>!! ab b) (>!! bc c) (>!! cd d) (>!! de e) (>!! ea 0)
        (execute ea ab) (execute ab bc) (execute bc cd) (execute cd de) (<!! (execute de ea))))
      
(defn part1 []
  (->> (combo/permutations (range 5))
       (map get-output)
       (apply max)))

(defn part2 []
  (->> (combo/permutations (range 5 10))
       (map get-output)
       (apply max)))

(defn -main
  [& args]
  (println "Part 1:" (part1))
  (println "Part 2:" (part2)))
