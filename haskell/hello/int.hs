zero :: Int -> Int
zero 0 = 0
zero x = zero $ x - 1

main = print $ zero 50
