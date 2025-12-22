# gf2_lin_algebra

`gf2_lin_algebra` is a high-performance Python library for doing linear algebra over the **finite field GF(2)** (binary matrices and vectors).  
The implementation is written in Rust for speed and safety, and exposed to Python using PyO3.

---

## Features

- Native Rust backend (fast, safe, no dependencies)
- Matrix and vector representation over GF(2)
- Rank, kernel, row-reduction, solving linear system of equations
- Pythonic API (`GF2Matrix`)
- Actively developed and open source

---

# Documentation structure


- **Installation**: how to install the package  
- **Examples**: practical demonstrations and use cases  
- **API Reference**: complete class and method documentation  
---

## Installation
```bash
pip install gf2-lin-algebra
```
---

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