let createOdds (stop: int64) =
    seq {
        for n in 1L..2L..stop do
            n
    }
    
let createSpiral (stop: int64): seq<int64> =
    seq {
        for pa,pb in Seq.zip (createOdds stop) (Seq.tail (createOdds stop)) do
            yield! seq {(pa*pa)..(pa+1L)..(pb*pb-1L)}
    }

let rawData = Set.ofSeq (createSpiral (int64 (1001.0**2.0)))
let numbers = Set.ofSeq (seq { 0L .. 1001L*1001L })
Set.intersect rawData numbers
|> Set.toSeq
|> Seq.sum
