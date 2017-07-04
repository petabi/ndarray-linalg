//! Cholesky decomposition

use ndarray::*;

use super::convert::*;
use super::error::*;
use super::layout::*;
use super::triangular::IntoTriangular;
use super::types::*;

pub use lapack_traits::UPLO;

pub trait Cholesky {
    type Output;
    fn cholesky(&self, UPLO) -> Result<Self::Output>;
}

pub trait CholeskyInto: Sized {
    fn cholesky_into(self, UPLO) -> Result<Self>;
}

pub trait CholeskyMut {
    fn cholesky_mut(&mut self, UPLO) -> Result<&mut Self>;
}

impl<A, S> CholeskyInto for ArrayBase<S, Ix2>
where
    A: Scalar,
    S: DataMut<Elem = A>,
{
    fn cholesky_into(mut self, uplo: UPLO) -> Result<Self> {
        A::cholesky(self.square_layout()?, uplo, self.as_allocated_mut()?)?;
        Ok(self.into_triangular(uplo))
    }
}

impl<A, S> CholeskyMut for ArrayBase<S, Ix2>
where
    A: Scalar,
    S: DataMut<Elem = A>,
{
    fn cholesky_mut(&mut self, uplo: UPLO) -> Result<&mut Self> {
        A::cholesky(self.square_layout()?, uplo, self.as_allocated_mut()?)?;
        Ok(self.into_triangular(uplo))
    }
}

impl<A, S> Cholesky for ArrayBase<S, Ix2>
where
    A: Scalar,
    S: Data<Elem = A>,
{
    type Output = Array2<A>;

    fn cholesky(&self, uplo: UPLO) -> Result<Self::Output> {
        let mut a = replicate(self);
        A::cholesky(a.square_layout()?, uplo, a.as_allocated_mut()?)?;
        Ok(a.into_triangular(uplo))
    }
}