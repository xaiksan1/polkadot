// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! The disputes module is responsible for selecting dispute votes to be sent with the inherent data.

use crate::LOG_TARGET;
use futures::channel::oneshot;
use polkadot_node_primitives::CandidateVotes;
use polkadot_node_subsystem::{messages::DisputeCoordinatorMessage, overseer};
use polkadot_primitives::{CandidateHash, SessionIndex};

/// Request the relevant dispute statements for a set of disputes identified by `CandidateHash` and the `SessionIndex`.
async fn request_votes(
	sender: &mut impl overseer::ProvisionerSenderTrait,
	disputes_to_query: Vec<(SessionIndex, CandidateHash)>,
) -> Vec<(SessionIndex, CandidateHash, CandidateVotes)> {
	let (tx, rx) = oneshot::channel();
	// Bounded by block production - `ProvisionerMessage::RequestInherentData`.
	sender.send_unbounded_message(DisputeCoordinatorMessage::QueryCandidateVotes(
		disputes_to_query,
		tx,
	));

	match rx.await {
		Ok(v) => v,
		Err(oneshot::Canceled) => {
			gum::warn!(target: LOG_TARGET, "Unable to query candidate votes");
			Vec::new()
		},
	}
}

pub(crate) mod prioritized_selection;
