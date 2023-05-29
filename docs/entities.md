
session:
- player_one (player)
- player_two (player)
- board (board)

game:
- new (function(player_one, player_two))
- player_one (&player)
- player_two (&player)
- board (board)
- pieces (pieces[])
- turn (usize)
- cur_player (&player)
- state (state enum)

board:
- squares
- new (function(pieces))
- get_piece_at (function)
- move_piece (function(piece, position))
- remove_piece (function(piece))
- get_square_at (function(position))

piece:
- type (type enum)
- color (color enum)
- ruleset -> rule.available_moves
- position (coordinates)
- is_alive (bool)

ruleset:
- type (type enum)
- available_moves (function(type, position) )
