pub mod numbers;
#[macro_use]
pub mod array;
pub mod dense;
pub mod layer;
pub mod model;

#[cfg(test)]
mod tests {
    #[test]
    fn test_backward() {
        assert_eq!(2 + 2, 4);
    }
}
