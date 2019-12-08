(ns day03.core
  (:gen-class))

(require '[clojure.string :as str] '[clojure.set :as hashset])

(defn new-points
  [[x y] dir len]
  (case dir
    \R (for [i (map inc (range x (+ x len)))] [i y])
    \L (for [i (map dec (range x (- x len) -1))] [i y])
    \U (for [j (map inc (range y (+ y len)))] [x j])
    \D (for [j (map dec (range y (- y len) -1))] [x j])))

(defn add-section
  [wire [dir & len-chars]]
  (let [len (read-string (apply str len-chars))]
    (concat wire (new-points (last wire) dir len))))

(defn string->wire
  [string]
  (reduce add-section
          [[0 0]]
          string))

(def wires
  (->> (slurp "input.txt")
       str/split-lines
       (map #(str/split % #","))
       (map string->wire)))

(defn abs [n] (max n (- n)))

(def a (first wires))
(def b (second wires))
(def intersections (hashset/intersection (set a) (set b)))

(defn part1 []
  (->> intersections
       (map #(map abs %))
       (map #(reduce + %))
       sort
       second))

(defn part2 []
  (->> intersections
       (map #(+ (.indexOf a %) (.indexOf b %)))
       sort
       second))

(defn -main
  [& args]
  (println "Part 1:" (part1))
  (println "Part 2:" (part2)))
