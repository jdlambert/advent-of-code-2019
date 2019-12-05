(defproject day04 "0.1.0-SNAPSHOT"
  :dependencies [[org.clojure/clojure "1.10.0"]]
  :main ^:skip-aot day04.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all}})
