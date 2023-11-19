/// The actual Matrix type.
pub type Matrix = Vec<Vec<f64>>;

#[derive(Debug)]
pub enum MatrixError {
    DimensionMismatch,
}

/// This is a trait that implement some functionalities for the type Matrix (alias of vec<vec<f64>>).
pub trait MatrixTrait {
    fn add(&self, other: Self) -> Result<Self, MatrixError>
    where
        Self: Sized;
    fn sub(&self, other: Self) -> Result<Self, MatrixError>
    where
        Self: Sized;
    fn mul(&self, other: Self) -> Result<Self, MatrixError>
    where
        Self: Sized;
}

impl MatrixTrait for Matrix {
    /// Adds two Matrix together.
    ///
    /// # Examples
    ///
    /// ```
    /// let matrix_a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    /// let matrix_b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    /// let result = matrix_a.add(matrix_b);
    ///
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Result<Matrix, MatrixError>.

    fn add(&self, other: Matrix) -> Result<Self, MatrixError> {
        if self.len() != other.len() {
            return Err(MatrixError::DimensionMismatch);
        }

        let mut result: Vec<Vec<f64>> = vec![vec![0.0; self[0].len()]; self.len()];

        for i in 0..self.len() {
            for j in 0..self[i].len() {
                result[i][j] = self[i][j] + other[i][j];
            }
        }

        Ok(result)
    }

    /// Subtracts one Matrix from the other.
    ///
    /// # Examples
    ///
    /// ```
    /// let matrix_a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    /// let matrix_b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    /// let result = matrix_a.sub(matrix_b);
    ///
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Result<Matrix, MatrixError>.

    fn sub(&self, other: Matrix) -> Result<Self, MatrixError> {
        if self.len() != other.len() {
            return Err(MatrixError::DimensionMismatch);
        }

        let mut result: Vec<Vec<f64>> = vec![vec![0.0; self[0].len()]; self.len()];

        for i in 0..self.len() {
            for j in 0..self[i].len() {
                result[i][j] = self[i][j] - other[i][j];
            }
        }

        Ok(result)
    }

    /// Calculates the Matrix Multiplication of the two given Matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// let matrix_a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    /// let matrix_b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    /// let result = matrix_a.mul(matrix_b);
    ///
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Result<Matrix, MatrixError>.

    fn mul(&self, other: Matrix) -> Result<Self, MatrixError> {
        let a_rows = self.len();
        let a_cols = self[0].len();
        let b_rows = other.len();
        let b_cols = other[0].len();

        if a_cols != b_rows {
            return Err(MatrixError::DimensionMismatch);
        }

        let mut result: Vec<Vec<f64>> = vec![vec![0.0; b_cols]; a_rows];

        for i in 0..a_rows {
            for j in 0..b_cols {
                for k in 0..a_cols {
                    result[i][j] += self[i][k] * other[k][j];
                }
            }
        }

        Ok(result)
    }
}
