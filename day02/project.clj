(defproject day02 "0.1.0-SNAPSHOT"
  :dependencies [[org.clojure/clojure "1.10.0"]]
  :main ^:skip-aot day02.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all}})
