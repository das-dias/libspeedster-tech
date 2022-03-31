/** ***********************************
* *[author] Diogo André (git-hub : das-dias)
* *[date] 31
* *[filename] lib.rs
* *[summary] Super path file of the libspeedster-tech crate.
* ***********************************
*/
// import local utils crate
pub(crate) use libspeedster_util as utils;
pub use utils::{SerdeFile, SerializationFormat};

//local modules
mod tech;
#[doc(inline)]
pub use tech::*;

mod read;
pub use read::*;

mod write;
pub use write::*;

#[cfg(test)]
mod tests;