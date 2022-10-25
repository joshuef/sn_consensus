pub mod consensus;

pub(crate) mod proposal;

mod abba;
mod broadcaster;
mod bundle;
mod doc;
mod hash;
mod vcbc;

pub type ProposalChecker = fn(&proposal::Proposal) -> bool;
