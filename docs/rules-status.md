pp-chess Rules Status
=====================

Current Scope
-------------

This repo now contains a small playable CLI chess core. It is meant to be a clean, explicit rules implementation, not a polished engine or GUI product.

Implemented Rules
-----------------

The current implementation supports the core gameplay rules needed to play normal chess in the terminal:

- standard piece movement
- captures
- alternating turns
- legal-move filtering
- check detection
- checkmate
- stalemate
- castling
- en passant
- promotion
- threefold repetition tracking
- fifty-move rule
- basic insufficient-material draw detection

What "legal-move filtering" means here:

- moves that leave your own king in check are rejected
- castling is blocked through check
- normal check/checkmate flow works

What Is Still Missing or Simplified
-----------------------------------

This is not yet a full tournament-grade chess product.

Still missing or simplified:

- no SAN notation
- no PGN export/import
- no engine search / AI
- no GUI
- no draw-by-agreement command
- no resignation command
- no distinction between "claimable draw" and "automatic draw"
  - repetition and fifty-move are currently treated as immediate status
- insufficient-material detection is simplified
  - common trivial cases are handled
  - obscure edge cases are not exhaustively modeled

Practical Summary
-----------------

If the goal is:

- "can two people play a real game of chess from the terminal?"
  - yes

- "is this a complete tournament-perfect rules implementation?"
  - no

Future Work
-----------

If this repo is expanded later, the next sensible additions would be:

1. resignation / draw offer commands
2. clearer move history
3. SAN or PGN support
4. more exact insufficient-material logic
5. AI/search if it ever becomes more than a rules core
