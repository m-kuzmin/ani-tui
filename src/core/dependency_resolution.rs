/// Resolver trait
pub trait Resolve<T> {
    //! # Example
    //! ```
    //! pub struct A;
    //! let resolved: A = Dependency::resolve();
    //!
    //! # use ani_tui::core::dependency_resolution::{Dependency, Resolve};
    //! #
    //! impl Resolve<A> for Dependency {
    //!     fn resolve() -> A { A }
    //! }
    //! ```

    /// Implement this for types you need to resolve.
    fn resolve() -> T;
}

/// A default resolver implementation
pub struct Dependency;
