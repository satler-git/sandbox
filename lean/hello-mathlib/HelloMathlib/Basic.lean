import Mathlib
import Std.Sat

def hello := "world"

theorem odd_power_zero_or_plus (x : Int) (a : Nat) : 0 ≤ x^(2*a) := by
  rw [pow_mul']
  apply pow_two_nonneg

theorem the (a b : Nat) : a.Coprime b -> (a + b).Coprime (a^2) := by
  intro h
  have h1 : (a + b).Coprime a := by
    rw [Nat.coprime_self_add_left, Nat.coprime_comm]
    exact h
  exact h1.pow_right 2


