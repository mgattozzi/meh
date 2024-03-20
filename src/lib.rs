#![feature(try_trait_v2)]
use std::convert::Infallible;
use std::ops::ControlFlow;
use std::ops::FromResidual;
use std::ops::Try;

pub enum Retryable<T, R, E> {
    Ok(T),
    Retry(R),
    Err(E),
}

pub enum Retry<T, R> {
    Ok(T),
    Retry(R),
}

impl<T, R, E> Try for Retryable<T, R, E> {
    type Output = Retry<T, R>;
    type Residual = Result<Infallible, E>;
    fn from_output(output: Self::Output) -> Self {
        match output {
            Retry::Ok(o) => Self::Ok(o),
            Retry::Retry(r) => Retryable::Retry(r),
        }
    }
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Ok(o) => ControlFlow::Continue(Retry::Ok(o)),
            Self::Retry(r) => ControlFlow::Continue(Retry::Retry(r)),
            Self::Err(e) => ControlFlow::Break(Result::Err(e)),
        }
    }
}

impl<T, R, E> FromResidual<Result<Infallible, E>> for Retryable<T, R, E> {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        match residual {
            Err(e) => Retryable::Err(e),
            _ => unreachable!(),
        }
    }
}
