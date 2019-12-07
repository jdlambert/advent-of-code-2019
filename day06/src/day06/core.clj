(ns day06.core
  (:gen-class))

(require '[clojure.string :as str])

(def orbits
  (->> (slurp "input.txt")
       clojure.string/split-lines
       (map #(str/split % #"\)"))
       (reduce (fn [dict [parent child]] (assoc dict child parent)) {})))

(defn enumerate-orbits [planet]
  (->> (iterate orbits planet)
       (take-while #(not= "COM" %))))

(defn part1 []
  (->> (keys orbits)
       (map enumerate-orbits)
       (map count)
       (reduce +)))

(defn part2 []
  (->> (concat (enumerate-orbits (orbits "YOU"))  ;; Find the path to COM 
               (enumerate-orbits (orbits "SAN"))) ;; for each target
       frequencies                ;; The 'planets' that were seen twice
       (filter #(= 1 (second %))) ;; are seen after the paths join
       count)) ;; Those that remain form the unique path

(defn -main
  [& args]
  (println "Part 1:" (part1))
  (println "Part 2:" (part2)))
