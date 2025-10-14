-- This module serves as the root of the `Hello` library.
-- Import modules here that should be built as part of the library.
import Hello.Basic

#eval 1 + 2
#eval 1 + 2 * 5

#eval String.removeLeadingSpaces "   a"

#eval if true then 5 else 1

#eval (1 + 2 : Nat)
#eval (1 + 1.1 : Float)
#eval (0: Nat)
#eval (-1: Int)

#check Function.uncurry String.append

def add1 (n: Nat) : Nat := n + 1
def add  (n: Nat) (k: Nat) : Nat := n + k
def uncurried_add := Function.uncurry add

theorem add_comm (n m: Nat) : n + m = m + n := by
  induction n with
  | zero =>
    simp
  | succ n ih =>


theorem pointwise_to_add (x y: Nat) : add x y = add y x := by
  simp [add]
  apply add_comm
