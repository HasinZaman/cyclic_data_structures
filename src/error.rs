//! This module contains all enums to the define errors that could occur at run time.

/// Error enum defines all the possible errors that can occur on run time.
#[derive(Debug, PartialEq, Eq)]
pub enum Error{
    /// IndexOutOfRange is thrown when a retrieval function(`.get(index)` or `list[index]`) is run with an index that does not exist within the function co-domain.
    IndexOutOfRange,

    /// Overflow is thrown when data type fails to add another element
    Overflow,

    /// InvalidSize is thrown when a data structure or type is too large to be converted into any of the cyclic data type variant
    InvalidSize
}
