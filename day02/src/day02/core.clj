(ns day02.core
  (:gen-class))

(require '[clojure.string :as str])

(def input
  (vec (map read-string
            (str/split (slurp "input.txt")
                       #","))))

(defn execute
  [noun verb]
  (loop [memory (assoc input 1 noun 2 verb)
         position 0]
    (let [[op a b out] (subvec memory position (+ position 4))]
      (if (= op 99)
        (first memory)
        (let [a (get memory a) b (get memory b)
              op (case op 1 + 2 *)]
          (recur (assoc memory out (op a b))
                 (+ 4 position)))))))

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
