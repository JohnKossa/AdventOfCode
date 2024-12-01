import System.IO
import Control.Monad
import Data.List (sort)

processLine :: String -> (Int, Int)
processLine line = do
    let parts = words line
    let col1 = read (parts !! 0) :: Int
    let col2 = read (parts !! 1) :: Int
    (col1, col2)

processFile :: String -> ([Int], [Int])
processFile contents = do
    let fileLines = lines contents
    let (col1, col2) = unzip (map processLine fileLines)
    (col1, col2)

xSmallest :: [Int] -> Int -> Int
xSmallest list x = do
    let sortedList = sort list
    sortedList !! x

absDiff :: Int -> Int -> Int
absDiff a b = do
    if a > b then a - b else b - a

numberOfTimesInList :: [Int] -> Int -> Int
numberOfTimesInList list x = do
    length (filter (== x) list)

score :: Int -> [Int] -> Int
score x myList = x * (numberOfTimesInList myList x)

main1 = do
    contents <- readFile "1.txt"
    let (col1, col2) = processFile contents
    -- sum the absolute differences between the 2 lists
    let diffs = zipWith absDiff (sort col1) (sort col2)
    let sumDiffs = sum diffs
    putStrLn (show sumDiffs)

main2 = do
    contents <- readFile "1.txt"
    let (col1, col2) = processFile contents
    --collect scores for each element in col1
    --for each element in col1, find the number of times it appears in col2, multiply by the element and sum
    let scores = map (\x -> score x col2) col1
    let sumScores = sum scores
    putStrLn (show sumScores)
