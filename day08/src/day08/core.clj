(ns day08.core
  (:gen-class))

(def layers
  (->> (slurp "input.txt")
       (partition (* 25 6))))

(defn part1 []
  (let [minimum (->> (map frequencies layers)
                     (apply (partial min-key #(% \0))))]
    (* (minimum \1) (minimum \2))))

(defn get-color [i]
  (->> (map #(nth % i) layers)
       (filter #(not= \2 %))
       first))

(defn part2 []
  (->> (range (* 25 6))
       (map get-color)
       (partition 25)
       (map (partial map #(if (= \0 %) \space \X)))
       (map #(apply str %))))

(defn -main
  [& args]
  (println "Part 1:" (part1))
  (println "Part 2:")
  (doseq [row (part2)] (println row)))
