(ns day02.core
  (:gen-class))

(require '[clojure.string :as str])

(def input
  (map read-string 
       (str/split (slurp "input.txt")
                  #",")))

(defn execute
    [noun verb]
    (let [program (-> (vec input)
                      (assoc 1 noun)
                      (assoc 2 verb))]
         (loop [memory program
                position 0]
                (let [op  (get memory position)
                      a   (get memory (get memory (inc position)))
                      b   (get memory (get memory (+ 2 position)))
                      out (get memory (+ 3 position))]
                      (case op
                            1  (recur (assoc memory out (+ a b)) (+ 4 position))
                            2  (recur (assoc memory out (* a b)) (+ 4 position))
                            99 (get memory 0))))))

(defn part1 []
  (execute 12 2))

(defn part2 []
  (first
    (for [n (range 100) 
          v (range 100) 
          :let [val (execute n v)] 
          :when (= 19690720 val)]
      (+ (* 100 n) v))))

(defn -main
  [& args]
  (println "Part 1:" (part1))
  (println "Part 2:" (part2)))
