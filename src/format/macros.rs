//! Contains macro for wrapping serde format.

macro_rules! text_format {
    ($type:ident based on $format:ident) => {
        impl<'a, T> Into<$crate::format::Text> for $type<&'a T>
        where
            T: ::serde::Serialize,
        {
            fn into(self) -> $crate::format::Text {
                $format::to_string(&self.0).map_err(::failure::Error::from)
            }
        }

        impl<T> From<$crate::format::Text> for $type<Result<T, ::failure::Error>>
        where
            T: for<'de> ::serde::Deserialize<'de>,
        {
            fn from(value: $crate::format::Text) -> Self {
                match value {
                    Ok(data) => $type($format::from_str(&data).map_err(::failure::Error::from)),
                    Err(reason) => $type(Err(reason)),
                }
            }
        }
    };
}
macro_rules! unimplemented_text_format {
    ($type:ident based on $format:ident) => {
        impl<'a, T> Into<$crate::format::Text> for $type<&'a T>
        where
            T: ::serde::Serialize,
        {
            fn into(self) -> $crate::format::Text {
                unimplemented!()
            }
        }

        impl<T> From<$crate::format::Text> for $type<Result<T, ::failure::Error>>
        where
            T: for<'de> ::serde::Deserialize<'de>,
        {
            fn from(value: $crate::format::Text) -> Self {
                unimplemented!()
            }
        }
    };
}

macro_rules! binary_format {
    ($type:ident based on $format:ident) => {
        impl<'a, T> Into<$crate::format::Binary> for $type<&'a T>
        where
            T: ::serde::Serialize,
        {
            fn into(self) -> $crate::format::Binary {
                $format::to_vec(&self.0).map_err(::failure::Error::from)
            }
        }

        impl<T> From<$crate::format::Binary> for $type<Result<T, ::failure::Error>>
        where
            T: for<'de> ::serde::Deserialize<'de>,
        {
            fn from(value: $crate::format::Binary) -> Self {
                match value {
                    Ok(data) => $type($format::from_slice(&data).map_err(::failure::Error::from)),
                    Err(reason) => $type(Err(reason)),
                }
            }
        }
    };
}