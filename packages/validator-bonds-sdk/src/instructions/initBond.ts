import {
  Keypair,
  PublicKey,
  Signer,
  TransactionInstruction,
} from '@solana/web3.js'
import { CONFIG_ADDRESS, ValidatorBondsProgram, bondAddress } from '../sdk'
import { walletPubkey } from '../utils'
import BN from 'bn.js'

export async function initBondInstruction({
  program,
  configAccount = CONFIG_ADDRESS,
  validatorVoteAccount,
  validatorVoteWithdrawer = walletPubkey(program),
  bondAuthority = walletPubkey(program),
  revenueShareHundredthBps,
  rentPayer = walletPubkey(program),
}: {
  program: ValidatorBondsProgram
  configAccount?: PublicKey
  validatorVoteAccount: PublicKey
  validatorVoteWithdrawer?: PublicKey | Keypair | Signer // signer
  bondAuthority?: PublicKey
  revenueShareHundredthBps: BN | number
  rentPayer?: PublicKey | Keypair | Signer // signer
}): Promise<{
  instruction: TransactionInstruction
  bondAccount: PublicKey
}> {
  const authorizedWithdrawer =
    validatorVoteWithdrawer instanceof PublicKey
      ? validatorVoteWithdrawer
      : validatorVoteWithdrawer.publicKey
  const renPayerPubkey =
    rentPayer instanceof PublicKey ? rentPayer : rentPayer.publicKey
  const [bondAccount] = bondAddress(
    configAccount,
    validatorVoteAccount,
    program.programId
  )
  const revenueShare =
    revenueShareHundredthBps instanceof BN
      ? revenueShareHundredthBps.toNumber()
      : revenueShareHundredthBps

  const instruction = await program.methods
    .initBond({
      bondAuthority,
      revenueShare: { hundredthBps: revenueShare },
    })
    .accounts({
      config: configAccount,
      bond: bondAccount,
      validatorVoteAccount,
      authorizedWithdrawer,
      rentPayer: renPayerPubkey,
    })
    .instruction()
  return {
    bondAccount,
    instruction,
  }
}