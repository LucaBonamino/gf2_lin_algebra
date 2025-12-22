use lin_algebra::GF2Matrix;
use lin_algebra::matrix::MatrixTrait;
use pyo3::prelude::*;

/// GF(2) matrix.
///
/// This class exposes binary linear algebra over GF(2)
/// implemented in Rust using PyO3.
#[pyclass(name = "GF2Matrix",  module = "gf2_lin_algebra.gf2_lin_algebra")]
pub struct PyGF2Matrix {
    inner: GF2Matrix,
}

#[pymethods]
impl PyGF2Matrix {

    /// Create a GF2Matrix from a list of lists.
    ///
    /// # Arguments
    /// * `elements` - (list[list[int]]): Binary matrix values, where each entry is 0 or 1.
    ///
    /// # Returns:
    /// GF2Matrix: New matrix instance.
    #[new]
    pub fn new(elements: Vec<Vec<u8>>) -> Self {
        Self {
            inner: GF2Matrix::new(elements)
        }
    }

    /// Convert PyGF2Matrix to vector of vectors.
    /// 
    /// # Returns
    /// List[List[int]] correspomding to the matrix elements 
    pub fn to_list(&self) -> Vec<Vec<u64>> {
        self.inner.elements
            .iter()
            .map(|row| row.iter().map(|&x| x as u64).collect())
            .collect()
    }

    /// Get the number or rows of a PyGf2matrix.
    /// 
    /// # Returns
    /// int: Number of rows of the GF2Matrix.
    pub fn nrows(&self) -> usize {
        self.inner.nrows()
    }

    /// Get the number or columns of a GF2Matrix.
    /// 
    /// # Returns
    /// int: Number of columns of the GF2Matrix.
    pub fn ncols(&self) -> usize {
        self.inner.ncols()
    }

    /// Get the number of rows and columns of a GF2Matrix
    /// 
    /// # Returns
    /// Tuple(int, int): (number_of_rows, number_of_columns).
    pub fn shape(&self) -> (usize, usize) {
        (self.nrows(), self.ncols())
    }

    /// Compue the rank of a GF2Matrix
    /// 
    /// # Returns
    /// int: Rank of the matrix.
    pub fn rank(&self) -> usize {
        self.inner.rank()
    }

    /// Compue the base otf the kernel of a GF2Matrix
    /// 
    /// # Retuens
    /// List[list[int]]: Basis vectors spanning the kernel.
    pub fn kernel(&self) -> Vec<Vec<u64>> {
        let k = self.inner.kernel();
        k.iter().map(|item| item.iter().map(|&x| x as u64).collect()).collect()
    }

    /// Compute the echelon form of a GF2Matrix
    /// 
    /// # Returns
    /// (GF2Matrix, list[tuple[int, int]]): The first element is the RREF form of the matrix and 
    /// the second is the history of the row operations applied to the matrix to compute the RREF.
    /// Each element of the row operations vector, is a tuple heving the modified row as first element and the row to which it has been summed as second element:
    ///     R1 -> R1 + R2 is represented the entry (R1, R2)
    /// The swap of two rows is represented as 3 entries:
    ///     swap(R1, R2) is represented as (R1, R2), (R2,R1), (R1,R2) 
    pub fn echelon_form(&self) -> (Self, Vec<(usize, usize)>){
        let (m, ops) = self.inner.echelon_form();
        (PyGF2Matrix::new(m.elements), ops)
    }

    /// Compute the base of the image of a GF2Matric
    /// 
    /// # Returns
    ///  List[list[int]]: Basis vector spanning the image.
    pub fn image(&self) -> Vec<Vec<u64>> {
        let im = self.inner.image();
        im.iter().map(|item| item.iter().map(|&x| x as u64).collect()).collect()
    }

    /// Solve the system of linear equation defined by a GF2Matrix and a right-hand-side vector b.
    /// 
    /// # Arguments
    /// * `b` - RHS vector of the system of equation
    /// 
    /// # Returns
    /// List[int]: Solution vector.
    pub fn solve(&self, b: Vec<u8>) -> Vec<u64>{
        let x = self.inner.solve(&b);
        x.into_iter().map(|item| item as u64).collect()
    }

    /// Solve the system of equationd efines by the GF2Matrix and by a right hand side GF2Matrix.
    /// 
    /// # Arguments
    /// * `ỳ`- RHS GF2Matrix
    /// 
    /// # Returns
    /// GF2Matrix: Solution matrix.
    pub fn solve_matrix_system(&self, y: &Self) -> Self{
        let x = self.inner.solve_matrix_system(&y.inner);
        PyGF2Matrix::new(x.elements)
    }

    /// Check if matrix is in reduced echelon form (RREF).
    ///
    /// # Returns:
    /// bool: True if matrix is in RREF, otherwise False.
    pub fn is_reduced_echelon(&self) -> bool {
        self.inner.is_reduced_echelon()
    }

    /// Get the element of a GF2Matrix from the indexes.
    /// 
    /// # Arguments
    /// 
    /// * `row_idx` - row index of the element
    /// * `column_idx` - column inde of the element
    /// 
    /// # Returns
    /// int: Value at selected position.
    pub fn get_element(&self, row_idx: usize, column_idx: usize) -> u8 {
        self.inner.elements[row_idx][column_idx]
    }
}
