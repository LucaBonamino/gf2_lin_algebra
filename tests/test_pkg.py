import unittest
from gf2_lin_algebra import GF2Matrix

class TestLinAlgebra(unittest.TestCase):

    def test_to_list(self):
        l = [
            [1,1,0],
            [1,1,1],
            [0,0,1]
        ]
        m = GF2Matrix(l)
        self.assertEqual(m.to_list(), l)

    def test_nrows_ncols(self):
        l = [
            [1,1,0],
            [1,1,1]
        ]
        m = GF2Matrix(l)
        self.assertEqual(m.nrows(), 2)
        self.assertEqual(m.ncols(), 3)

        m = GF2Matrix([])
        self.assertEqual(m.nrows(), 0)
        self.assertEqual(m.ncols(), 0)

    def test_get_element(self):
        l = [
            [1,1,0],
            [1,1,1]
        ]
        m = GF2Matrix(l)

        for i in range(2):
            for j in range(3):
                self.assertEqual(m.get_element(i,j), l[i][j])

    def test_rank(self):
        l = [
            [1,1,0],
            [1,1,1],
            [0,0,1]
        ]
        m = GF2Matrix(l)
        self.assertEqual(m.rank(), 2)

    
    def is_reduced_echelon(self):
        l = [
            [1,1,0],
            [1,1,1],
            [0,0,1]
        ]
        self.assertFalse(GF2Matrix(l).is_reduced_echelon())

        l = [
            [1,0,1],
            [0,1,0]
        ]
        self.assertTrue(GF2Matrix(l).is_reduced_echelon())


    def test_solve(self):
        l = [
            [0,1,0],
            [1,0,1],
            [0,0,1]
        ]
        m = GF2Matrix(l)
        b = [1, 0, 1]
        self.assertEqual(m.solve(b), [1,1,1])


    def test_solve_matrix_system(self):
        l = [
            [0,1,0],
            [1,0,1],
            [0,0,1]
        ]
        y = [
            [1,1,0],
            [1,0,1],
            [0,0,1]
        ]
        m = GF2Matrix(l)
        self.assertEqual(m.solve_matrix_system(GF2Matrix(y)).to_list(), [[1, 1, 0], [0, 1, 0], [0, 0, 1]])

    def test_kernel(self):
        l = [
            [0,1,0],
            [1,0,1],
            [0,0,1]
        ]
        m = GF2Matrix(l)
        self.assertEqual(m.kernel(), [])
    
        l = [
            [1,0,1],
            [1,0,1],
            [0,0,1]
        ]
        m = GF2Matrix(l)
        self.assertEqual(m.kernel(), [[0,1,0]])


    def test_imege(self):
        l = [
            [0,1,0],
            [1,0,1],
            [0,0,1]
        ]
        m = GF2Matrix(l)
        self.assertEqual(m.image(), [[1, 0, 0], [0, 1, 0], [0, 0, 1]])