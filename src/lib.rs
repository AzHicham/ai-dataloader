#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

//! The `dataloader_rs` crate provides a Rust implementation to the [`PyTorch`] `DataLoader`.
//!
//!
//! Unlike the python version where almost everithing happen in runtime, `dataloader_rs` is built on Rust's powerful trait system.
//!
//!
//! ## Highlights
//!
//! - Shuffle or Sequential Sampler.
//! - Customisable sampler
//! - Default collate function that cover most of the type of the `std`, supporting nested type.
//! - Cutomizable collate function
//!
//! ## Examples
//!
//! Examples can be found in the [examples] folder.
//!
//! ## `PyTorch` `DataLoader` function equivalents
//!
//! ### `DataLoader` creation
//!
//! `PyTorch` | `dataloader_rs` | Notes
//! --------|-----------------|-------
//! `DataLoader(dataset)` | `DataLoader::builder(dataset).build()` | Create a `DataLoader` with default parameter
//! `DataLoader(dataset, batch_size=2)` | `DataLoader::builder(dataset).batch_size(2).build()` | Setup the batch size
//! `DataLoader(dataset, shuffle=True)` | `DataLoader::builder(dataset).shuffle().build()` | Shuffle the data
//! `DataLoader(dataset, sampler=CustomSampler)` | `DataLoader::builder(dataset).sampler::<CustomSampler>().build()` | Provide a custom sampler
//!
//! ### Combined options
//!
//! `PyTorch` | `dataloader_rs`
//! --------|-----------------
//! `DataLoader(dataset, shuffle=True, batch_size=2, drop_last=True, collate_fn=CustomCollate)` | `DataLoaderBuilder::new(dataset).shuffle().batch_size(2).drop_last().collate_fn(CustomCollate).build()`
//!
//! ### `DataLoader` iteration
//!
//! `PyTorch` | `dataloader_rs` | Notes
//! --------|-----------------|-------
//! `for text, label in data_loader:` | `for (text, label) in data_loader.iter()` | Simple iteration
//!
//! ## To be done
//!
//! - Parallel `DataLoader`
//! - Iterable dataset (dataset than are not indexable and have possibly no length)
//! - Integrate with a Tensor library on GPU when one will be mature enough (Open for suggestion).
//!
//! [PyTorch]: https://pytorch.org/
//! [examples]: https://github.com/Tudyx/dataloader_rs/tree/main/examples
//!

mod dataloader;
mod dataset;
mod fetch;

pub mod collate;
pub mod sampler;

pub use dataloader::{builder::DataLoaderBuilder, DataLoader};
pub use dataset::{Dataset, GetSample, Len};
