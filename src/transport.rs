//! This module provides code for transporting data from one endpoint to another.

pub use self::{
    client::{Client, ClientId},
    message::*,
    postbox::PostBox,
    postoffice::PostOffice,
};

mod client;
mod message;
mod postbox;
mod postoffice;
pub mod tcp;
