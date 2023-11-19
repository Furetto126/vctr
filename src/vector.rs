/// The actual Vector type.
pub type Vector = Vec<f64>;

#[derive(Debug)]
pub enum VectorError {
    LengthMismatch,
}

/// This is a trait that implement some functionalities for the type Vector (alias of vec<f64>).
pub trait VectorTrait {
    fn add(&self, other: Self) -> Result<Self, VectorError>
    where
        Self: Sized;
    fn sub(&self, other: Self) -> Result<Self, VectorError>
    where
        Self: Sized;
    fn mul(&self, other: Self) -> Result<Self, VectorError>
    where
        Self: Sized;
    fn div(&self, other: Self) -> Result<Self, VectorError>
    where
        Self: Sized;
}

impl VectorTrait for Vector {
    /// Adds two Vector together.
    ///
    /// # Examples
    ///
    /// ```
    /// let vector_a = vec![0.0, 1.0, 2.0];
    /// let vector_b = vec![3.0, 4.0, 5.0];
    /// let result = vector_a.add(vector_b);
    ///
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Result<Vector, VectorError>.

    fn add(&self, other: Vector) -> Result<Self, VectorError> {
        if self.len() != other.len() {
            return Err(VectorError::LengthMismatch);
        }

        let mut result = vec![];

        for i in 0..self.len() {
            result.push(self[i] + other[i]);
        }

        Ok(result)
    }

    /// Subtracts one Vector to the other.
    ///
    /// # Examples
    ///
    /// ```
    /// let vector_a = vec![0.0, 1.0, 2.0];
    /// let vector_b = vec![3.0, 4.0, 5.0];
    /// let result = vector_a.sub(vector_b);
    ///
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Result<Vector, VectorError>.

    fn sub(&self, other: Self) -> Result<Self, VectorError> {
        if self.len() != other.len() {
            return Err(VectorError::LengthMismatch);
        }

        let mut result = vec![];

        for i in 0..self.len() {
            result.push(self[i] - other[i]);
        }

        Ok(result)
    }

    /// Multiplies two Vector together.
    ///
    /// # Examples
    ///
    /// ```
    /// let vector_a = vec![0.0, 1.0, 2.0];
    /// let vector_b = vec![3.0, 4.0, 5.0];
    /// let result = vector_a.mul(vector_b);
    ///
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Result<Vector, VectorError>.

    fn mul(&self, other: Self) -> Result<Self, VectorError> {
        if self.len() != other.len() {
            return Err(VectorError::LengthMismatch);
        }

        let mut result = vec![];

        for i in 0..self.len() {
            result.push(self[i] * other[i]);
        }

        Ok(result)
    }

    /// Divides one Vector to the other.
    ///
    /// # Examples
    ///
    /// ```
    /// let vector_a = vec![0.0, 1.0, 2.0];
    /// let vector_b = vec![3.0, 4.0, 5.0];
    /// let result = vector_a.div(vector_b);
    ///
    /// ```
    ///
    /// # Returns
    ///
    /// Returns a Result<Vector, VectorError>.

    fn div(&self, other: Self) -> Result<Self, VectorError> {
        if self.len() != other.len() {
            return Err(VectorError::LengthMismatch);
        }

        let mut result = vec![];

        for i in 0..self.len() {
            result.push(self[i] / other[i]);
        }

        Ok(result)
    }
}
