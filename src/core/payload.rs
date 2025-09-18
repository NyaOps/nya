use std::{any::{Any, type_name}, sync::Arc};

type DynAny = dyn Any + Send + Sync + 'static;

#[derive(Clone, Default)]
pub struct Payload {
    value: Option<Arc<DynAny>>,
    type_name: Option<&'static str>,
}

impl Payload {
    pub fn empty() -> Self { Self::default() }
    pub fn new<T>(v: T) -> Self
    where
        T: Any + Send + Sync + 'static,
    {
        Self {
            value: Some(Arc::new(v)),
            type_name: Some(type_name::<T>()),
        }
    }
}

#[derive(Debug)]
pub enum PayloadError {
    Empty,
    TypeMismatch { expected: &'static str, actual: &'static str },
}

pub trait Get {
    fn get<T: Any>(&self) -> Result<&T, PayloadError>;
    fn into_arc<T: Any + Send + Sync + 'static>(self) -> Result<Arc<T>, PayloadError>
    where
        Self: Sized;
}

impl Get for Payload {
    fn get<T: Any>(&self) -> Result<&T, PayloadError> {
        let some = self.value.as_ref().ok_or(PayloadError::Empty)?;
        // Borrowed downcast
        (&**some as &dyn Any)
            .downcast_ref::<T>()
            .ok_or_else(|| PayloadError::TypeMismatch {
                expected: type_name::<T>(),
                actual: self.type_name.unwrap_or("unknown"),
            })
    }

    fn into_arc<T: Any + Send + Sync + 'static>(self) -> Result<Arc<T>, PayloadError> {
        let some = self.value.ok_or(PayloadError::Empty)?;
        // Owning downcast (requires owning the Arc)
        Arc::downcast::<T>(some).map_err(|_| PayloadError::TypeMismatch {
            expected: type_name::<T>(),
            actual: self.type_name.unwrap_or("unknown"),
        })
    }
}
