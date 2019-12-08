(defproject day07 "0.1.0-SNAPSHOT"
  :dependencies [[org.clojure/clojure "1.10.0"] [org.clojure/math.combinatorics "0.1.6"]  [org.clojure/core.async "0.6.532"]]
  :main ^:skip-aot day07.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all}})
