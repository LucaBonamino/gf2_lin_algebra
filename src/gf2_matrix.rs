use lin_algebra::GF2Matrix;
use lin_algebra::matrix::MatrixTrait;
use pyo3::prelude::*;
use pyo3::types::PyList;

#[pyclass]
pub struct PyGF2Matrix {
    inner: GF2Matrix,
}

#[pymethods]
impl PyGF2Matrix {
    #[new]
    pub fn new(elements: Vec<Vec<u8>>) -> Self {
        Self {
            inner: GF2Matrix::new(elements)
        }
    }

    pub fn to_list(&self) -> Vec<Vec<u64>> {
        self.inner.elements
            .iter()
            .map(|row| row.iter().map(|&x| x as u64).collect())
            .collect()
    }

    pub fn nrows(&self) -> usize {
        self.inner.nrows()
    }

    pub fn ncols(&self) -> usize {
        self.inner.ncols()
    }

    pub fn size(&self) -> (usize, usize) {
        (self.nrows(), self.ncols())
    }

    pub fn rank(&self) -> usize {
        self.inner.rank()
    }
    
    pub fn kernel(&self) -> Vec<Vec<u64>> {
        let k = self.inner.kernel();
        k.iter().map(|item| item.iter().map(|&x| x as u64).collect()).collect()
    }

    pub fn echelon_form(&self) -> (Self, Vec<(usize, usize)>){
        let (m, ops) = self.inner.echelon_form();
        (PyGF2Matrix::new(m.elements), ops)
    }

    pub fn image(&self) -> Vec<Vec<u64>> {
        let im = self.inner.image();
        im.iter().map(|item| item.iter().map(|&x| x as u64).collect()).collect()
    }

    pub fn solve(&self, b: Vec<u8>) -> Vec<u64>{
        let x = self.inner.solve(&b);
        x.into_iter().map(|item| item as u64).collect()
    }

    pub fn solve_matrix_system(&self, y: &Self) -> Self{
        let x = self.inner.solve_matrix_system(&y.inner);
        PyGF2Matrix::new(x.elements)
    }

    pub fn is_reduced_echelon(&self) -> bool {
        self.inner.is_reduced_echelon()
    }

    pub fn get_element(&self, row_idx: usize, column_idx: usize) -> u8 {
        self.inner.elements[row_idx][column_idx]
    }
}
