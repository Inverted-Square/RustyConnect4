# Connect4 Bot

Author: Ashley Leonard

---

## Description

A work in progress for a Connect4 solver using Victor Allis' thesis on 'A Knowledge-based Approach of Connect-Four'. Only the basic setup has been done so far and actually getting on with implementing the strategic rules is still to be done. So far, this includes implementing the boards as bit-boards, implementing minimax with alpha-beta pruning and move ordering, and using transposition tables where the boards are hashed using Zobrist hashing.

Optimising the transposition tables using mirrored boards + indifferent sets (from section 12.5 of the thesis), are currently being explored in separate branches. Surprisingly however, these have not yielded any significant improvements so far. 
