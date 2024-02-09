@test "player 1 win (1,1,2,_)" {
    run bash -c 'echo "0
2
1" | TESTING="1" cargo run --quiet --bin cli'
    [[ "$output" = "Player 1 wins!" ]]
}

@test "player 1 win (2,_,1,1)" {
    run bash -c 'echo "2
0
1" | TESTING="1" cargo run --quiet --bin cli'
    [[ "$output" = "Player 1 wins!" ]]
}

@test "player 2 win (1,2,2,1)" {
    run bash -c 'echo "0
1
3
2" | TESTING="1" cargo run --quiet --bin cli'
    [[ "$output" = "Player 2 wins!" ]]
}

@test "draw (1,2,1,2)" {
    run bash -c 'echo "0
1
2
3" | TESTING="1" cargo run --quiet --bin cli'
    [[ "$output" = "draw!" ]]
}
