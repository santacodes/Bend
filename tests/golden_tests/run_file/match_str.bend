(is_as "As") = 2
(is_as "as") = 2
(is_as "") = 1
(is_as *) = 0

map f (List/Cons x xs) = (List/Cons (f x) (map f xs))
map f [] = []

main = (map is_as ["As" "as" "" "Asd" "qwerty" "AAs"])
