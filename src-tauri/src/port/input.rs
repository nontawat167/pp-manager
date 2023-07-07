#[derive(Debug, PartialEq, Eq)]
pub enum SearchOperator<T> {
    Equal(T),
    // GreaterThan(T),
    // LessThan(T),
}