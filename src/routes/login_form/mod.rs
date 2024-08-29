mod get;
pub mod post;

pub use get::*;
// Not publishing everything since
// `FormData` is defined in other places too
pub use post::login;
