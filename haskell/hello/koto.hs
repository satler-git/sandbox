three :: (a -> a) -> a -> a
three f x = f $ f $ f x

squeare x = x ^ 2

main = do print $ three squeare 3
