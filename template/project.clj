(defproject template "0.1.0-SNAPSHOT"
  :dependencies [[org.clojure/clojure "1.10.0"]]
  :main ^:skip-aot template.core
  :target-path "target/%s"
  :profiles {:uberjar {:aot :all}})
