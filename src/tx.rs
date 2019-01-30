use bitcoin;

use bitcoin::util::hash::BitcoinHash;

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct InputScriptInfo {
	pub hex: Option<::HexBytes>,
	pub asm: Option<String>,
}

pub struct InputScript<'a>(pub &'a bitcoin::Script);

impl<'a> ::GetInfo<InputScriptInfo> for InputScript<'a> {
	fn get_info(&self, _network: ::bitcoin::Network) -> InputScriptInfo {
		InputScriptInfo {
			hex: Some(self.0.to_bytes().into()),
			asm: Some(format!("{:?}", self.0)), //TODO(stevenroose) asm
		}
	}
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct InputInfo {
	pub prevout: Option<String>,
	pub txid: Option<bitcoin::util::hash::Sha256dHash>,
	pub vout: Option<u32>,
	pub script_sig: Option<InputScriptInfo>,
	pub sequence: Option<u32>,
	pub witness: Option<Vec<::HexBytes>>,
}

impl ::GetInfo<InputInfo> for ::bitcoin::TxIn {
	fn get_info(&self, network: ::bitcoin::Network) -> InputInfo {
		InputInfo {
			prevout: Some(self.previous_output.to_string()),
			txid: Some(self.previous_output.txid),
			vout: Some(self.previous_output.vout),
			sequence: Some(self.sequence),
			script_sig: Some(InputScript(&self.script_sig).get_info(network)),
			witness: if self.witness.len() > 0 {
				Some(self.witness.iter().map(|h| h.clone().into()).collect())
			} else {
				None
			},
		}
	}
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct OutputScriptInfo {
	pub hex: Option<::HexBytes>,
	pub asm: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "type")]
	pub type_: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub address: Option<bitcoin::Address>,
}

pub struct OutputScript<'a>(pub &'a bitcoin::Script);

impl<'a> ::GetInfo<OutputScriptInfo> for OutputScript<'a> {
	fn get_info(&self, network: ::bitcoin::Network) -> OutputScriptInfo {
		OutputScriptInfo {
			hex: Some(self.0.to_bytes().into()),
			asm: Some(format!("{:?}", self.0)), //TODO(stevenroose) asm
			type_: Some(
				if self.0.is_p2pk() {
					"p2pk"
				} else if self.0.is_p2pkh() {
					"p2pkh"
				} else if self.0.is_op_return() {
					"opreturn"
				} else if self.0.is_p2sh() {
					"p2sh"
				} else if self.0.is_v0_p2wpkh() {
					"p2wpkh"
				} else if self.0.is_v0_p2wsh() {
					"p2wsh"
				} else {
					"unknown"
				}
				.to_owned(),
			),
			address: ::address::address_from_script(&self.0, network),
		}
	}
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct OutputInfo {
	pub value: Option<u64>,
	pub script_pub_key: Option<OutputScriptInfo>,
}

impl ::GetInfo<OutputInfo> for bitcoin::TxOut {
	fn get_info(&self, network: ::bitcoin::Network) -> OutputInfo {
		OutputInfo {
			value: Some(self.value),
			script_pub_key: Some(OutputScript(&self.script_pubkey).get_info(network)),
		}
	}
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct TransactionInfo {
	pub txid: Option<bitcoin::util::hash::Sha256dHash>,
	pub hash: Option<bitcoin::util::hash::Sha256dHash>,
	pub size: Option<usize>,
	pub weight: Option<usize>,
	pub vsize: Option<usize>,
	pub version: Option<u32>,
	pub locktime: Option<u32>,
	pub inputs: Option<Vec<InputInfo>>,
	pub outputs: Option<Vec<OutputInfo>>,
}

impl ::GetInfo<TransactionInfo> for bitcoin::Transaction {
	fn get_info(&self, network: ::bitcoin::Network) -> TransactionInfo {
		TransactionInfo {
			txid: Some(self.txid()),
			hash: Some(self.bitcoin_hash()),
			version: Some(self.version),
			locktime: Some(self.lock_time),
			size: Some(bitcoin::consensus::encode::serialize(self).len()),
			weight: Some(self.get_weight() as usize),
			vsize: Some((self.get_weight() / 4) as usize),
			inputs: Some(self.input.iter().map(|i| i.get_info(network)).collect()),
			outputs: Some(self.output.iter().map(|o| o.get_info(network)).collect()),
		}
	}
}
