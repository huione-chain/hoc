use crate::validator_commands::{call_0x5, write_transaction_response};
use anyhow::{anyhow, Result};
use clap::Parser;
use colored::Colorize;
use serde::Serialize;
use std::fmt::{Debug, Display, Formatter, Write};
use sui_json_rpc_types::{SuiObjectDataOptions, SuiTransactionBlockResponse};
use sui_sdk::wallet_context::WalletContext;
use sui_types::{
    base_types::{ObjectID, ObjectRef, SuiAddress},
    transaction::{CallArg, ObjectArg},
};
use tracing::info;

const DEFAULT_GAS_BUDGET: u64 = 200_000_000; // 0.2 SUI
#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub enum SuiSupperCommitteeCommand {
    #[clap(name = "update-committee-validator-proposal")]
    CreateUpdateCommitteeValidatorProposal {
        #[clap(name = "operate", long)]
        operate: bool,
        #[clap(name = "committee_validator", long)]
        committee_validator: SuiAddress,
        /// Gas budget for this transaction.
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
    },
    #[clap(name = "update-committee-validator-proposal")]
    CreateUpdateTrustedValidatorProposal {
        #[clap(name = "operate", long)]
        operate: bool,
        #[clap(name = "validator", long)]
        validator: SuiAddress,
        /// Gas budget for this transaction.
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
    },
    #[clap(name = "update-validator-only-staking-proposal")]
    CreateUpdateValidatorOnlyStakingProposal {
        #[clap(name = "validator_only_staking", long)]
        validator_only_staking: bool,
        /// Gas budget for this transaction.
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
    },
    #[clap(name = "vote-proposal")]
    VoteProposal {
        #[clap(name = "proposal-id", long)]
        proposal_id: ObjectID,
        #[clap(name = "agree", long)]
        agree: bool,
        /// Gas budget for this transaction.
        #[clap(name = "gas-budget", long)]
        gas_budget: Option<u64>,
    },
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum SuiSupperCommitteeResponse {
    CreateUpdateCommitteeValidatorProposal(SuiTransactionBlockResponse),
    CreateUpdateTrustedValidatorProposal(SuiTransactionBlockResponse),
    CreateUpdateValidatorOnlyStakingProposal(SuiTransactionBlockResponse),
    VoteProposal(SuiTransactionBlockResponse),
}

impl SuiSupperCommitteeCommand {
    pub async fn execute(
        self,
        context: &mut WalletContext,
    ) -> anyhow::Result<SuiSupperCommitteeResponse, anyhow::Error> {
        let ret = Ok(match self {
            SuiSupperCommitteeCommand::CreateUpdateCommitteeValidatorProposal {
                operate,
                committee_validator,
                gas_budget,
            } => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);

                let args = vec![
                    CallArg::Pure(bcs::to_bytes(&operate).unwrap()),
                    CallArg::Pure(bcs::to_bytes(&committee_validator).unwrap()),
                    CallArg::CLOCK_IMM,
                ];
                let response = call_0x5(context, "create_update_committee_validator_proposal", args, gas_budget).await?;
                SuiSupperCommitteeResponse::CreateUpdateCommitteeValidatorProposal(response)
            }
            SuiSupperCommitteeCommand::CreateUpdateTrustedValidatorProposal { operate, validator, gas_budget } => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);

                let args = vec![
                    CallArg::Pure(bcs::to_bytes(&operate).unwrap()),
                    CallArg::Pure(bcs::to_bytes(&validator).unwrap()),
                    CallArg::CLOCK_IMM,
                ];
                let response = call_0x5(context, "create_update_trusted_validator_proposal", args, gas_budget).await?;
                SuiSupperCommitteeResponse::CreateUpdateTrustedValidatorProposal(response)
            }
            SuiSupperCommitteeCommand::CreateUpdateValidatorOnlyStakingProposal {
                validator_only_staking,
                gas_budget,
            } => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
                let args = vec![CallArg::Pure(bcs::to_bytes(&validator_only_staking).unwrap()), CallArg::CLOCK_IMM];
                let response =
                    call_0x5(context, "create_update_validator_only_staking_proposal", args, gas_budget).await?;

                SuiSupperCommitteeResponse::CreateUpdateValidatorOnlyStakingProposal(response)
            }
            SuiSupperCommitteeCommand::VoteProposal { proposal_id, agree, gas_budget } => {
                let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
                let proposal_ref = get_proposal_ref(context, proposal_id).await?;

                let proposal_obj_mut =
                    ObjectArg::SharedObject { id: proposal_id, initial_shared_version: proposal_ref.1, mutable: true };

                let args = vec![
                    CallArg::Object(proposal_obj_mut),
                    CallArg::Pure(bcs::to_bytes(&agree).unwrap()),
                    CallArg::CLOCK_IMM,
                ];
                let response = call_0x5(context, "vote_proposal", args, gas_budget).await?;

                SuiSupperCommitteeResponse::VoteProposal(response)
            }
        });
        ret
    }
}

async fn get_proposal_ref(context: &mut WalletContext, proposal_id: ObjectID) -> Result<ObjectRef> {
    let sui_client = context.get_client().await?;
    let proposal_obj_ref = sui_client
        .read_api()
        .get_object_with_options(proposal_id, SuiObjectDataOptions::default().with_owner())
        .await?
        .object_ref_if_exists()
        .ok_or_else(|| anyhow!("OperationCap {} does not exist", proposal_id))?;
    Ok(proposal_obj_ref)
}

impl Display for SuiSupperCommitteeResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        match self {
            SuiSupperCommitteeResponse::CreateUpdateCommitteeValidatorProposal(response) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
            SuiSupperCommitteeResponse::CreateUpdateTrustedValidatorProposal(response) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
            SuiSupperCommitteeResponse::CreateUpdateValidatorOnlyStakingProposal(response) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
            SuiSupperCommitteeResponse::VoteProposal(response) => {
                write!(writer, "{}", write_transaction_response(response)?)?;
            }
        }
        write!(f, "{}", writer.trim_end_matches('\n'))
    }
}

impl SuiSupperCommitteeResponse {
    pub fn print(&self, pretty: bool) {
        let line = if pretty { format!("{self}") } else { format!("{:?}", self) };
        // Log line by line
        for line in line.lines() {
            // Logs write to a file on the side.  Print to stdout and also log to file, for tests to pass.
            println!("{line}");
            info!("{line}")
        }
    }
}

impl Debug for SuiSupperCommitteeResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = serde_json::to_string_pretty(self);
        let s = string.unwrap_or_else(|err| format!("{err}").red().to_string());
        write!(f, "{}", s)
    }
}
