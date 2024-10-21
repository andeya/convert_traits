/// `my_convert!` Add a prefix based on the [::std::convert::AsRef], [::std::convert::AsMut],
///  [::std::convert::From], [::std::convert::Into], [::std::convert::TryFrom] and [::std::convert::TryInto].
/// For example: [my_convert!(a_Bc)]
/// ```
/// pub trait ABcAsRef<T: ?Sized> {
///     fn a_bc_as_ref(&self) -> &T;
/// }
/// impl<T: ?Sized, U: ?Sized> ABcAsRef<U> for &T
/// where
///     T: ABcAsRef<U>,
/// {
///     #[inline]
///     fn a_bc_as_ref(&self) -> &U {
///         <T as ABcAsRef<U>>::a_bc_as_ref(*self)
///     }
/// }
/// impl<T: ?Sized, U: ?Sized> ABcAsRef<U> for &mut T
/// where
///     T: ABcAsRef<U>,
/// {
///     #[inline]
///     fn a_bc_as_ref(&self) -> &U {
///         <T as ABcAsRef<U>>::a_bc_as_ref(*self)
///     }
/// }
///
///
/// pub trait ABcAsMut<T: ?Sized> {
///     fn a_bc_as_mut(&mut self) -> &mut T;
/// }
/// impl<T: ?Sized, U: ?Sized> ABcAsMut<U> for &mut T
/// where
///     T: ABcAsMut<U>,
/// {
///     #[inline]
///     fn a_bc_as_mut(&mut self) -> &mut U {
///         (*self).a_bc_as_mut()
///     }
/// }
///
///
///  pub trait ABcFrom<T>: Sized {
///     fn a_bc_from(value: T) -> Self;
/// }
/// pub trait ABcInto<T>: Sized {
///     fn a_bc_into(self) -> T;
/// }
/// impl<T, U> ABcInto<U> for T
/// where
///     U: ABcFrom<T>,
/// {
///     #[inline]
///     fn a_bc_into(self) -> U {
///         U::a_bc_from(self)
///     }
/// }
///
///
/// pub trait ABcTryFrom<T>: Sized {
///     type Error;
///
///     fn a_bc_try_from(value: T) -> ::core::result::Result<Self, Self::Error>;
/// }
/// impl<T, U> ABcTryFrom<U> for T
/// where
///     U: ABcInto<T>,
/// {
///     type Error = ::std::convert::Infallible;
///
///     #[inline]
///     fn a_bc_try_from(value: U) -> ::core::result::Result<Self, Self::Error> {
///         ::core::result::Result::Ok(U::a_bc_into(value))
///     }
/// }
///
/// pub trait ABcTryInto<T>: Sized {
///     type Error;
///
///     fn a_bc_try_into(self) -> ::core::result::Result<T, Self::Error>;
/// }
/// impl<T, U> ABcTryInto<U> for T
/// where
///     U: ABcTryFrom<T>,
/// {
///     type Error = U::Error;
///
///     #[inline]
///     fn a_bc_try_into(self) -> ::core::result::Result<U, U::Error> {
///         U::a_bc_try_from(self)
///     }
/// }
/// ```
pub use paste::paste;

#[macro_export]
macro_rules! my_convert {
    ($prefix:ident) => {
        $crate::paste! {
            pub trait [<$prefix:camel AsRef>]<T: ?Sized> {
                fn [<$prefix:snake _as_ref>](&self) -> &T;
            }

            impl<T: ?Sized, U: ?Sized> [<$prefix:camel AsRef>]<U> for &T
            where
                T: [<$prefix:camel AsRef>]<U>,
            {
                #[inline]
                fn [<$prefix:snake _as_ref>](&self) -> &U {
                    <T as [<$prefix:camel AsRef>]<U>>::[<$prefix:snake _as_ref>](*self)
                }
            }

            impl<T: ?Sized, U: ?Sized> [<$prefix:camel AsRef>]<U> for &mut T
            where
                T: [<$prefix:camel AsRef>]<U>,
            {
                #[inline]
                fn [<$prefix:snake _as_ref>](&self) -> &U {
                    <T as [<$prefix:camel AsRef>]<U>>::[<$prefix:snake _as_ref>](*self)
                }
            }

            pub trait [<$prefix:camel AsMut>]<T: ?Sized> {
                fn [<$prefix:snake _as_mut>](&mut self) -> &mut T;
            }

            impl<T: ?Sized, U: ?Sized> [<$prefix:camel AsMut>]<U> for &mut T
            where
                T: [<$prefix:camel AsMut>]<U>,
            {
                #[inline]
                fn [<$prefix:snake _as_mut>](&mut self) -> &mut U {
                    (*self).[<$prefix:snake _as_mut>]()
                }
            }


            pub trait [<$prefix:camel From>]<T>: Sized {
                fn [<$prefix:snake _from>](value: T) -> Self;
            }

            pub trait [<$prefix:camel Into>]<T>: Sized {
                fn [<$prefix:snake _into>](self) -> T;
            }

            impl<T, U> [<$prefix:camel Into>]<U> for T
            where
                U: [<$prefix:camel From>]<T>,
            {
                #[inline]
                fn [<$prefix:snake _into>](self) -> U {
                    U::[<$prefix:snake _from>](self)
                }
            }

            // Reflexive implementation for [<$prefix:camel From>] trait
            impl<T> [<$prefix:camel From>]<T> for T {
                #[inline(always)]
                fn [<$prefix:snake _from>](value: T) -> T {
                    value
                }
            }


            pub trait [<$prefix:camel TryFrom>]<T>: Sized {
                type Error;

                fn [<$prefix:snake _try_from>](value: T) -> ::core::result::Result<Self, Self::Error>;
            }

            impl<T, U> [<$prefix:camel TryFrom>]<U> for T
            where
                U: [<$prefix:camel Into>]<T>,
            {
                type Error = std::convert::Infallible;

                #[inline]
                fn [<$prefix:snake _try_from>](value: U) -> ::core::result::Result<Self, Self::Error> {
                    ::core::result::Result::Ok(U::[<$prefix:snake _into>](value))
                }
            }

            pub trait [<$prefix:camel TryInto>]<T>: Sized {
                type Error;

                fn [<$prefix:snake _try_into>](self) -> ::core::result::Result<T, Self::Error>;
            }

            impl<T, U> [<$prefix:camel TryInto>]<U> for T
            where
                U: [<$prefix:camel TryFrom>]<T>,
            {
                type Error = U::Error;

                #[inline]
                fn [<$prefix:snake _try_into>](self) -> ::core::result::Result<U, U::Error> {
                    U::[<$prefix:snake _try_from>](self)
                }
            }
        }
    };
}

#[cfg(test)]
#[allow(non_local_definitions)]
mod tests {
    use super::my_convert;

    #[derive(Debug, PartialEq)]
    struct A(B);
    #[derive(Debug, PartialEq)]
    struct B;

    my_convert!(a_Bc);

    #[test]
    fn test_as_ref() {
        impl ABcAsRef<B> for A {
            fn a_bc_as_ref(&self) -> &B {
                &self.0
            }
        }
        assert_eq!(&A(B).0, A(B).a_bc_as_ref())
    }

    #[test]
    fn test_as_mut() {
        impl ABcAsMut<B> for A {
            fn a_bc_as_mut(&mut self) -> &mut B {
                &mut self.0
            }
        }
        assert_eq!(&mut B, A(B).a_bc_as_mut())
    }
    #[test]
    fn test_from_into() {
        impl ABcFrom<B> for A {
            fn a_bc_from(value: B) -> Self {
                Self(value)
            }
        }
        assert_eq!(A(B), A::a_bc_from(B));
        assert_eq!(A(B), B.a_bc_into());
    }
    #[test]
    fn test_try_from_into() {
        impl ABcTryFrom<A> for B {
            type Error = std::convert::Infallible;
            fn a_bc_try_from(value: A) -> Result<Self, Self::Error> {
                Ok(value.0)
            }
        }
        assert_eq!(B, B::a_bc_try_from(A(B)).unwrap());
        assert_eq!(B, A(B).a_bc_try_into().unwrap());
    }
}
