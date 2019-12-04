;; This solution is incorrect

(ns day03.core
  (:gen-class))

(require '[clojure.string :as str] '[clojure.set :as hashset])

(def directions {\U [1 0] \D [-1 0] \R [0 1] \L [0 -1]})

(defn section->locations
   [section pos steps]
            (for [i (range 1 (inc (:len section)))]
            [(map + (map #(* % i) (:dir section)) pos) (+ i steps)]))

(defn reduce-section
      [acc section]
      (let [locs (section->locations section (:pos acc) (:steps acc))
            [pos steps] (last locs)]
            {:locs (into (:locs acc) locs) :pos pos :steps steps}))

(defn wire->locations
   [wire]
   (:locs (reduce reduce-section
                  {:locs {} :pos [0 0] :steps 0}
                  wire)))

(defn wire->locset
   [wire]
   (into #{} (map first (wire->locations wire))))

(defn wire-intersection
   [w1 w2]
   (hashset/intersection (wire->locset w1) (wire->locset w2)))

(defn string->section
     [string]
     {:dir (directions (first string))
      :len (read-string (apply str (rest string)))})

(defn string->wire
    [string]
    (let [substrings (str/split string #",")]
         (map string->section substrings)))

(def wires
  (->> (slurp "input.txt")
       str/split-lines
       (map string->wire)))

(defn abs [n] (max n (- n)))

(defn part1 []
  (->> (wire-intersection (first wires) (second wires))
       (map #(map abs %))
       (map #(reduce + %))
       sort
       second))

(defn part2 []
  (let [a (first wires) b (second wires)
        wla (wire->locations a) wlb (wire->locations b)]
        (->> (wire-intersection a b)
            (map #(+ (wla %) (wlb %)))
            sort
            second)))

(defn -main
  [& args]
  (println "Part 1:" (part1))
  (println "Part 2:" (part2)))
