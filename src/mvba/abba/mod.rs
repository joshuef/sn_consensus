pub(super) mod context;
pub(super) mod message;
pub(super) mod state;

mod pre_process;
mod error;
mod message_set;


use self::error::{Error, Result};
use self::message::Message;
use self::pre_process::ProposeState;
use self::state::State;
use crate::mvba::crypto::public::PubKey;
use crate::mvba::{broadcaster::Broadcaster, proposal::Proposal};
use std::cell::RefCell;
use std::rc::Rc;

use super::ProposalChecker;

// VCBC is a verifiably authenticatedly c-broadcast protocol.
// Each party $P_i$ c-broadcasts the value that it proposes to all other parties
// using verifiable authenticated consistent broadcast.
pub(crate) struct ABBA {
    state: Option<Box<dyn State>>,
}

impl ABBA {
    pub fn new(
        parties: &Vec<PubKey>,
        threshold: usize,
        broadcaster: Rc<RefCell<Broadcaster>>,
    ) -> Self {
        let ctx =
            context::Context::new(parties, threshold,  broadcaster);

        Self {
            state: Some(Box::new(ProposeState::new(ctx))),
        }
    }


    pub fn process_message(&mut self, sender: &PubKey, message: &[u8]) -> Result<()> {
        let msg: Message = minicbor::decode(message)?;

        if let Some(mut s) = self.state.take() {
            s.process_message(sender, &msg)?;
            self.state = Some(s.decide()?);
        }
        Ok(())
    }

    pub fn is_decided(&self) -> bool {
        self.state.as_ref().unwrap().context().decided
    }
}

#[cfg(test)]
#[path = "./tests.rs"]
mod tests;
