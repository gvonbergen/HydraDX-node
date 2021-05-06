// This file is part of HydraDX.

// Copyright (C) 2020-2021  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;
use frame_support::traits::GetPalletVersion;
use hex::FromHex;
use primitives::Balance;

pub fn import_initial_claims<T: Config>(claims_data: &[(&'static str, Balance)]) -> frame_support::weights::Weight {
	let version = <Pallet<T> as GetPalletVersion>::storage_version();
	if version == None {
		for (addr, amount) in claims_data.iter() {
			let balance: BalanceOf<T> = T::CurrencyBalance::from(*amount).into();

			Claims::<T>::insert(
				EthereumAddress(<[u8; 20]>::from_hex(&addr[2..]).unwrap_or_else(|addr| {
					frame_support::log::warn!("Error encountered while migrating Ethereum address: {}", addr);
					EthereumAddress::default().0
				})),
				balance,
			);
		}
		T::DbWeight::get().reads_writes(2, 3)
	} else {
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::Test;

	#[test]
	fn data_migration_should_work() {
		sp_io::TestExternalities::default().execute_with(|| {
			let claims_data: [(&'static str, Balance); 4] = [
				("0x8202c0af5962b750123ce1a9b12e1c30a4973557", 555),
				("0xb3e7104ea029874c36da42ca115c8c90b5938ef5", 666),
				("0x30503adcd76c9bf9d068a15be4a8cf6e874fef6c", 777),
				("0x19ad3978b233a91a30f9ddda6c6f6c92ba97b8f2", 888),
			];
			let (first_addr, first_balance) = claims_data[0];
			let (second_addr, second_balance) = claims_data[1];
			let (last_addr, last_balance) = claims_data.last().copied().unwrap();

			let first_addr = EthereumAddress(<[u8; 20]>::from_hex(&first_addr[2..]).unwrap());
			let second_addr = EthereumAddress(<[u8; 20]>::from_hex(&second_addr[2..]).unwrap());
			let last_addr = EthereumAddress(<[u8; 20]>::from_hex(&last_addr[2..]).unwrap());
			assert_eq!(Claims::<Test>::get(first_addr), 0);
			assert_eq!(Claims::<Test>::get(second_addr), 0);
			assert_eq!(Claims::<Test>::get(last_addr), 0);

			import_initial_claims::<Test>(&claims_data);

			assert_eq!(Claims::<Test>::get(first_addr), first_balance);
			assert_eq!(Claims::<Test>::get(second_addr), second_balance);
			assert_eq!(Claims::<Test>::get(last_addr), last_balance);
		})
	}
}
