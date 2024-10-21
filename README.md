# convert_traits

Define your own conversion traits to solve the problem of converting two external types without using new types.


[![Crates.io](https://img.shields.io/crates/v/convert_traits)](https://crates.io/crates/convert_traits)
[![Docs](https://shields.io/docsrs/convert_traits)](https://docs.rs/convert_traits)
[![License](https://img.shields.io/crates/l/convert_traits)](https://github.com/andeya/convert_traits?tab=MIT-1-ov-file)


> Generate a series of conversion traits with a specified prefix based on: [::std::convert::AsRef], [::std::convert::AsMut],
[::std::convert::From], [::std::convert::Into], [::std::convert::TryFrom] and [::std::convert::TryInto].


## Install

Run the following Cargo command in your project directory:

```sh
cargo add convert_traits
```

Or add the following line to your Cargo.toml:

```toml
convert_traits = "1"
```

## Example

```rust
#[cfg(test)]
#[allow(non_local_definitions)]
mod tests {
    # extern crate convert_traits;

    use convert_traits::my_convert;

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
```