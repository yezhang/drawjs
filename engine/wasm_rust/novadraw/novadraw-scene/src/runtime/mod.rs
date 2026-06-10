//! Runtime services around the figure graph.
//!
//! Runtime modules coordinate dispatch, context, deferred mutations, updates,
//! and system composition without owning the graph structure itself.

pub mod context;
pub mod event;
pub mod mutation;
pub mod system;
pub mod update;
