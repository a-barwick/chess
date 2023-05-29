

Turn starts
Player selects piece
Board shows available moves
Player selects square
Game validates square is available move
    If square is empty, move player
    If square contains enemy player, take enemy piece, move player
Calculate enemy check/mate
    If check, set check status on player
    If mate, end game
End turn



Start game
[Turn]
End game

Turn:
- cur_player
- score
- p_white piece's
- p_black piece's
- move:
    - piece
    - start_location
    - end_location
