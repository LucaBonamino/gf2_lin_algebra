# gf2_lin_algebra

[![Documentation](https://img.shields.io/badge/docs-online-blue)](https://LucaBonamino.github.io/gf2_lin_algebra/)


**Python package for linear algebra operations in GF(2)**

`gf2_lin_algebra` is a high-performance Python library for doing linear algebra over the **finite field GF(2)** (binary matrices and vectors).  
The implementation is written in Rust for speed and safety, and exposed to Python using PyO3.

This package is useful for:

- cryptography research  
- boolean matrix algebra  
- solving linear system of equations over over GF(2)  
- teaching and exploration of finite fields  

---

## Features

- Native Rust backend (fast, safe, no dependencies)
- Matrix and vector representation over GF(2)
- Rank, image, kernel, row-reduction, solving linear system of equations
- Pythonic API (`GF2Matrix`)
- Designed to be simple and educational
- Actively developed and open source

---

## Installation

```bash
pip install gf2-lin-algebra
```

or clone the repo and install locally:

```bash
git clone https://github.com/LucaBonamino/gf2_lin_algebra.git
cd lin_algebra_py
pip install .
```

## Quick Example

```python
from gf2_lin_algebra import GF2Matrix

# Create matrix
mat = GF2Matrixx([
    [1,0,1],
    [0,1,1],
    [1,1,0]
    ])

# Check shape
print(mat.shape())     # -> (3, 3)

# Compute rank
print(mat.rank())      # -> 3

# Compute kernel
print(mat.kernel())

# Compute image
print(mat.image())

# Solve system of equations
print(mat.solve([1,1,0]))

# Solve matrix sytem of equation
mat_y = GF2Matrixx([
    [0,1,1],
    [1,1,0],
    [0,0,1]
    ])
print(mat.solve_matrix_system(mat_y))

```