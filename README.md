# emola-rs

emola-rs is a kind of lisp language written by Rust.


## Syntax


### Arithmetic operator    
```clojure
(+ 1 1 1)  
(- 2 1 1)  
(* 2 2 2)  
(/ 4 2 2)  
(= 2 2)    

### Binding
```clojure
(def hoge 1)
```

### Evaluates the expressions in order.
```clojure
(do 
  (def hoge 1)
  (+ hoge 1))
```





