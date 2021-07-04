// This file is modified from Substrate.
// The rights and restrictions of the original copyright below apply.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The tests for the public proposal queue.

use super::*;
use std::convert::TryFrom;

fn aye(x: u8, balance: u64) -> AccountVote<u64> {
	AccountVote::Standard {
		vote: Vote { aye: true, conviction: Conviction::try_from(x).unwrap() },
		balance
	}
}

fn nay(x: u8, balance: u64) -> AccountVote<u64> {
	AccountVote::Standard {
		vote: Vote { aye: false, conviction: Conviction::try_from(x).unwrap() },
		balance
	}
}

#[test]
fn accept_treasury_proposal_simple_majority() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&Treasury::account_id(), 100);
		// Block 1: Treasury proposal 0.
		assert_ok!(Treasury::propose_spend(Origin::signed(1), 10, 3));
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		// Block 2: Democracy proposal: approve treasury proposal 0 with simple majority.
		assert_ok!(Democracy::propose_treasury_spend_simple_majority(Origin::signed(2),0,1));
		assert_eq!(Balances::free_balance(2), 19);
		assert_eq!(Balances::free_balance(1), 9);
		assert_eq!(Balances::free_balance(3), 30);
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		// Block 3: Referendum on hold.
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		// Block 4: Referendum starts.
		assert_ok!(Democracy::vote(Origin::signed(3), 0, nay(3,6)));
		assert_eq!(Balances::free_balance(1), 9);
		assert_eq!(Balances::free_balance(2), 20);
		assert_eq!(Balances::free_balance(3), 30);
		assert_eq!(Balances::locks(3)[0].amount, 6);
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		// Block 5: Voting.
		assert_ok!(Democracy::vote(Origin::signed(2), 0, aye(2,10)));
		assert_eq!(Balances::free_balance(1), 9);
		assert_eq!(Balances::free_balance(2), 20);
		assert_eq!(Balances::locks(2)[0].amount, 10);
		assert_eq!(Balances::free_balance(3), 30);
		assert_eq!(Balances::locks(3)[0].amount, 6);
		assert_eq!(tally(0), Tally { ayes: 20, nays: 18, turnout: 16 });
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		// Block 6: Referendum ends.
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		// Block 8: Can remove vote but unlock does not apply.
		assert_ok!(Democracy::remove_vote(Origin::signed(2), 0));
		assert_ok!(Democracy::unlock(Origin::signed(2), 2));
		assert_eq!(Balances::locks(2)[0].amount, 10);
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		// Block 9: Referendum not yet enacted for treasury proposal 0.
		assert_eq!(Balances::free_balance(3), 30);
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		// Block 10: Referendum enacted for treasury proposal 0 + Can unlock.
		assert_eq!(Balances::free_balance(3), 40);
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		assert_ok!(Democracy::unlock(Origin::signed(2), 2));
		assert_eq!(Balances::locks(2), vec![]);
	});
}

#[test]
fn reject_treasury_proposal_simple_majority() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&Treasury::account_id(), 100);
		// Block 1: Treasury proposal 0.
		assert_ok!(Treasury::propose_spend(Origin::signed(1), 10, 3));
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		// Block 2: Democracy proposal: approve treasury proposal 0 with simple majority.
		assert_ok!(Democracy::propose_treasury_spend_simple_majority(Origin::signed(2),0,1));
		assert_eq!(Balances::free_balance(2), 19);
		assert_eq!(Balances::free_balance(1), 9);
		assert_eq!(Balances::free_balance(3), 30);
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		// Block 3: Referendum on hold.
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		// Block 4: Referendum starts.
		assert_ok!(Democracy::vote(Origin::signed(3), 0, nay(3,7)));
		assert_eq!(Balances::free_balance(1), 9);
		assert_eq!(Balances::free_balance(2), 20);
		assert_eq!(Balances::free_balance(3), 30);
		assert_eq!(Balances::locks(3)[0].amount, 7);
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		// Block 5: Voting.
		assert_ok!(Democracy::vote(Origin::signed(2), 0, aye(2,10)));
		assert_eq!(Balances::free_balance(1), 9);
		assert_eq!(Balances::free_balance(2), 20);
		assert_eq!(Balances::locks(2)[0].amount, 10);
		assert_eq!(Balances::free_balance(3), 30);
		assert_eq!(Balances::locks(3)[0].amount, 7);
		assert_eq!(tally(0), Tally { ayes: 20, nays: 21, turnout: 17 });
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		// Block 6: Referendum ends.
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		// Block 8: Can remove vote. Unlock applies to losing side.
		assert_ok!(Democracy::remove_vote(Origin::signed(2), 0));
		assert_ok!(Democracy::unlock(Origin::signed(2), 2));
		assert_eq!(Balances::locks(2), vec![]);
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		assert_eq!(Balances::free_balance(3), 30);
		next_block();
		<Treasury as OnInitialize<u64>>::on_initialize(System::block_number());
		// Block 10: Referendum would be enacted for treasury proposal 0 but did not pass.
		assert_eq!(Balances::free_balance(3), 30);
	});
}


