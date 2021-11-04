<p align="center">
  <img src=https://rustacean.net/assets/rustacean-flat-happy.svg width=256>
</p>

<h1>
  <p align="center">
    rusty_pi
  </p>
</h1>

Implementation of Monte Carlo PI approximation algorithm in Rust Python bindings.

Time of 100M iterations approximation on Core i7 10th gen:
- Python ~ 31.82 s
- Rust ~ 1 s

# How to run code locally:

1. python3.10 -m venv .venv
2. source .venv/bin/activate
3. pip install -r requirements.txt
4. maturin develop --release
5. python calculate_pi.py

# Useful links:
- PyO3 docs: https://pyo3.rs/v0.15.0/
