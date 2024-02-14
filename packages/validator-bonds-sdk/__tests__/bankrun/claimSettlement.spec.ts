import {
  Config,
  Errors,
  MerkleTreeNode,
  SETTLEMENT_CLAIM_SEED,
  ValidatorBondsProgram,
  claimSettlementInstruction,
  fundSettlementInstruction,
  getConfig,
  getSettlementClaim,
  settlementClaimAddress,
} from '../../src'
import {
  BankrunExtendedProvider,
  assertNotExist,
  currentEpoch,
  initBankrunTest,
  warpOffsetEpoch,
  warpToNextEpoch,
} from './bankrun'
import {
  createUserAndFund,
  executeInitBondInstruction,
  executeInitConfigInstruction,
  executeInitSettlement,
} from '../utils/testTransactions'
import { ProgramAccount } from '@coral-xyz/anchor'
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SYSVAR_STAKE_HISTORY_PUBKEY,
  SYSVAR_CLOCK_PUBKEY,
  StakeProgram,
  SystemProgram,
} from '@solana/web3.js'
import {
  createBondsFundedStakeAccount,
  createStakeAccount,
  createVoteAccount,
} from '../utils/staking'
import { signer, pubkey } from '@marinade.finance/web3js-common'
import {
  MERKLE_ROOT_BUF,
  configAccountKeypair,
  totalClaimVoteAccount1,
  totalClaimVoteAccount2,
  treeNodeBy,
  voteAccount1Keypair,
  voteAccount2Keypair,
  withdrawer1,
  withdrawer1Keypair,
  withdrawer2,
  withdrawer2Keypair,
  withdrawer3,
  withdrawer3Keypair,
} from '../utils/merkleTreeTestData'
import { checkErrorMessage, verifyError } from '@marinade.finance/anchor-common'
import BN from 'bn.js'

describe('Validator Bonds claim settlement', () => {
  const epochsToClaimSettlement = 3
  let provider: BankrunExtendedProvider
  let program: ValidatorBondsProgram
  let config: ProgramAccount<Config>
  let bondAccount: PublicKey
  let operatorAuthority: Keypair
  let validatorIdentity1: Keypair
  let voteAccount1: PublicKey
  let validatorIdentity2: Keypair
  let voteAccount2: PublicKey
  let settlementAccount1: PublicKey
  let settlementAccount2: PublicKey
  let settlementEpoch: number
  let rentCollector: Keypair
  let stakeAccount1: PublicKey
  let stakeAccount2: PublicKey

  beforeAll(async () => {
    ;({ provider, program } = await initBankrunTest())
    const { configAccount, operatorAuthority: operatorAuth } =
      await executeInitConfigInstruction({
        program,
        provider,
        epochsToClaimSettlement,
        configAccountKeypair: configAccountKeypair,
      })
    operatorAuthority = operatorAuth
    config = {
      publicKey: configAccount,
      account: await getConfig(program, configAccount),
    }
    ;({ voteAccount: voteAccount1, validatorIdentity: validatorIdentity1 } =
      await createVoteAccount({
        voteAccount: voteAccount1Keypair,
        provider,
      }))
    await executeInitBondInstruction({
      program,
      provider,
      config: config.publicKey,
      voteAccount: voteAccount1,
      validatorIdentity: validatorIdentity1,
    })
    ;({ voteAccount: voteAccount2, validatorIdentity: validatorIdentity2 } =
      await createVoteAccount({
        voteAccount: voteAccount2Keypair,
        provider,
      }))
    ;({ bondAccount } = await executeInitBondInstruction({
      program,
      provider,
      config: config.publicKey,
      voteAccount: voteAccount2,
      validatorIdentity: validatorIdentity2,
    }))

    rentCollector = Keypair.generate()
    settlementEpoch = await currentEpoch(provider)
    ;({ settlementAccount: settlementAccount1 } = await executeInitSettlement({
      config: config.publicKey,
      program,
      provider,
      voteAccount: voteAccount1,
      operatorAuthority,
      currentEpoch: settlementEpoch,
      rentCollector: rentCollector.publicKey,
      merkleRoot: MERKLE_ROOT_BUF,
      maxMerkleNodes: 1,
      maxTotalClaim: totalClaimVoteAccount1,
    }))
    ;({ settlementAccount: settlementAccount2 } = await executeInitSettlement({
      config: config.publicKey,
      program,
      provider,
      voteAccount: voteAccount2,
      operatorAuthority,
      currentEpoch: settlementEpoch,
      merkleRoot: MERKLE_ROOT_BUF,
      // wrongly setup to be able to get errors from contract
      maxMerkleNodes: 1,
      maxTotalClaim: 100, // has to be lower than 111111
    }))
    stakeAccount1 = await createBondsFundedStakeAccount({
      program,
      provider,
      config: config.publicKey,
      voteAccount: voteAccount1,
      lamports: totalClaimVoteAccount1.toNumber() + LAMPORTS_PER_SOL * 5,
    })
    stakeAccount2 = await createBondsFundedStakeAccount({
      program,
      provider,
      config: config.publicKey,
      voteAccount: voteAccount2,
      lamports: totalClaimVoteAccount2.toNumber() + LAMPORTS_PER_SOL * 6,
    })

    await warpToNextEpoch(provider) // activate stake account

    const { instruction: fundIx1, splitStakeAccount: split1 } =
      await fundSettlementInstruction({
        program,
        settlementAccount: settlementAccount1,
        stakeAccount: stakeAccount1,
      })
    const { instruction: fundIx2, splitStakeAccount: split2 } =
      await fundSettlementInstruction({
        program,
        settlementAccount: settlementAccount2,
        stakeAccount: stakeAccount2,
      })
    await provider.sendIx(
      [signer(split1), signer(split2), operatorAuthority],
      fundIx1,
      fundIx2
    )
    if ((await provider.context.banksClient.getAccount(withdrawer1)) === null) {
      await createUserAndFund(provider, LAMPORTS_PER_SOL, withdrawer1Keypair)
      await createUserAndFund(provider, LAMPORTS_PER_SOL, withdrawer2Keypair)
      await createUserAndFund(provider, LAMPORTS_PER_SOL, withdrawer3Keypair)
    }
  })

  it('claim settlement various', async () => {
    const treeNode1Withdrawer1 = treeNodeBy(voteAccount1, withdrawer1)
    const stakeAccountLamportsBefore = 123 * LAMPORTS_PER_SOL
    const stakeAccountTreeNode1Withdrawer1 = await createStakeAccount({
      provider,
      lamports: stakeAccountLamportsBefore,
      voteAccount: voteAccount1,
      newStakerAuthority: treeNode1Withdrawer1.treeNode.stakeAuthority,
      newWithdrawerAuthority: treeNode1Withdrawer1.treeNode.withdrawAuthority,
    })
    const { instruction: ixWrongTreeNode } = await claimSettlementInstruction({
      program,
      claimAmount: treeNode1Withdrawer1.treeNode.data.claim.subn(1),
      merkleProof: treeNode1Withdrawer1.proof,
      settlementAccount: settlementAccount1,
      stakeAccountFrom: stakeAccount1,
      stakeAccountTo: stakeAccountTreeNode1Withdrawer1,
      stakeAccountStaker: treeNode1Withdrawer1.treeNode.stakeAuthority,
      stakeAccountWithdrawer: treeNode1Withdrawer1.treeNode.withdrawAuthority,
    })
    try {
      await provider.sendIx([], ixWrongTreeNode)
      throw new Error('should have failed; wrong tree node proof')
    } catch (e) {
      verifyError(e, Errors, 6029, 'claim proof failed')
    }

    const rentPayer = await createUserAndFund(provider, LAMPORTS_PER_SOL)
    const { instruction, settlementClaimAccount } =
      await claimSettlementInstruction({
        program,
        claimAmount: treeNode1Withdrawer1.treeNode.data.claim,
        merkleProof: treeNode1Withdrawer1.proof,
        settlementAccount: settlementAccount1,
        stakeAccountFrom: stakeAccount1,
        stakeAccountTo: stakeAccountTreeNode1Withdrawer1,
        stakeAccountStaker: treeNode1Withdrawer1.treeNode.stakeAuthority,
        stakeAccountWithdrawer: treeNode1Withdrawer1.treeNode.withdrawAuthority,
        rentPayer: rentPayer,
      })
    try {
      await provider.sendIx([signer(rentPayer)], instruction)
      throw new Error('should have failed; stake is not deactivated')
    } catch (e) {
      expect(checkErrorMessage(e, 'insufficient funds')).toBeTruthy()
    }

    const notAStakeAccount = await createUserAndFund(provider, LAMPORTS_PER_SOL)
    const { instruction: ixWrongStakeAccountTo } =
      await claimSettlementInstruction({
        program,
        claimAmount: treeNode1Withdrawer1.treeNode.data.claim,
        merkleProof: treeNode1Withdrawer1.proof,
        settlementAccount: settlementAccount1,
        stakeAccountFrom: stakeAccount1,
        stakeAccountTo: pubkey(notAStakeAccount),
        stakeAccountStaker: treeNode1Withdrawer1.treeNode.stakeAuthority,
        stakeAccountWithdrawer: treeNode1Withdrawer1.treeNode.withdrawAuthority,
      })
    try {
      await provider.sendIx([], ixWrongStakeAccountTo)
      throw new Error('should have failed; wrong stake account')
    } catch (e) {
      expect((e as Error).message).toMatch('custom program error: 0xbbf')
    }
    const stakeAccountWrongStaker = await createStakeAccount({
      provider,
      lamports: 3 * LAMPORTS_PER_SOL,
      voteAccount: voteAccount1,
      newStakerAuthority: pubkey(notAStakeAccount),
      newWithdrawerAuthority: treeNode1Withdrawer1.treeNode.withdrawAuthority,
    })
    const { instruction: ixWrongStaker } = await claimSettlementInstruction({
      program,
      claimAmount: treeNode1Withdrawer1.treeNode.data.claim,
      merkleProof: treeNode1Withdrawer1.proof,
      settlementAccount: settlementAccount1,
      stakeAccountFrom: stakeAccount1,
      stakeAccountTo: stakeAccountWrongStaker,
      stakeAccountStaker: treeNode1Withdrawer1.treeNode.stakeAuthority,
      stakeAccountWithdrawer: treeNode1Withdrawer1.treeNode.withdrawAuthority,
    })
    try {
      await provider.sendIx([], ixWrongStaker)
      throw new Error('should have failed; wrong staker')
    } catch (e) {
      verifyError(e, Errors, 6051, 'Wrong staker authority')
    }
    const stakeAccountWrongWithdrawer = await createStakeAccount({
      provider,
      lamports: 3 * LAMPORTS_PER_SOL,
      voteAccount: voteAccount1,
      newStakerAuthority: treeNode1Withdrawer1.treeNode.stakeAuthority,
      newWithdrawerAuthority: pubkey(notAStakeAccount),
    })
    const { instruction: ixWrongWithdrawer } = await claimSettlementInstruction(
      {
        program,
        claimAmount: treeNode1Withdrawer1.treeNode.data.claim,
        merkleProof: treeNode1Withdrawer1.proof,
        settlementAccount: settlementAccount1,
        stakeAccountFrom: stakeAccount1,
        stakeAccountTo: stakeAccountWrongWithdrawer,
        stakeAccountStaker: treeNode1Withdrawer1.treeNode.stakeAuthority,
        stakeAccountWithdrawer: treeNode1Withdrawer1.treeNode.withdrawAuthority,
      }
    )
    try {
      await provider.sendIx([], ixWrongWithdrawer)
      throw new Error('should have failed; wrong withdrawer')
    } catch (e) {
      verifyError(e, Errors, 6012, 'Wrong withdrawer authority')
    }

    warpToNextEpoch(provider) // deactivate stake account

    await provider.sendIx([signer(rentPayer)], instruction)

    const stakeAccountInfo = await provider.connection.getAccountInfo(
      stakeAccountTreeNode1Withdrawer1
    )
    expect(stakeAccountInfo?.lamports).toEqual(
      stakeAccountLamportsBefore +
        treeNode1Withdrawer1.treeNode.data.claim.toNumber()
    )

    const [settlementClaimAddr, bump] = settlementClaimAddress(
      {
        settlement: settlementAccount1,
        voteAccount: voteAccount1,
        stakeAccountStaker: treeNode1Withdrawer1.treeNode.stakeAuthority,
        stakeAccountWithdrawer: treeNode1Withdrawer1.treeNode.withdrawAuthority,
        claim: treeNode1Withdrawer1.treeNode.data.claim,
      },
      program.programId
    )
    expect(settlementClaimAccount).toEqual(settlementClaimAddr)
    const settlementClaim = await getSettlementClaim(
      program,
      settlementClaimAccount
    )
    expect(settlementClaim.amount).toEqual(
      treeNode1Withdrawer1.treeNode.data.claim
    )
    expect(settlementClaim.bump).toEqual(bump)
    expect(settlementClaim.rentCollector).toEqual(pubkey(rentPayer))
    expect(settlementClaim.settlement).toEqual(pubkey(settlementAccount1))
    expect(settlementClaim.stakeAccountStaker).toEqual(
      treeNode1Withdrawer1.treeNode.stakeAuthority
    )
    expect(settlementClaim.stakeAccountWithdrawer).toEqual(
      treeNode1Withdrawer1.treeNode.withdrawAuthority
    )
    expect(settlementClaim.voteAccount).toEqual(pubkey(voteAccount1))
    const settlementClaimAccountInfo = await provider.connection.getAccountInfo(
      settlementClaimAccount
    )
    expect(
      (await provider.connection.getAccountInfo(pubkey(rentPayer)))?.lamports
    ).toEqual(LAMPORTS_PER_SOL - settlementClaimAccountInfo!.lamports)
    // TODO: add expect on size of account
    console.log(
      'settlement claim account length',
      settlementClaimAccountInfo?.data.byteLength
    )

    await warpToNextEpoch(provider)

    try {
      await provider.sendIx([signer(rentPayer)], instruction)
      throw new Error('should have failed; already claimed')
    } catch (e) {
      expect((e as Error).message).toMatch('custom program error: 0x0')
    }

    const stakeAccountToWrongBump = await createStakeAccount({
      provider,
      lamports: 233 * LAMPORTS_PER_SOL,
      voteAccount: voteAccount1,
      newStakerAuthority: treeNode1Withdrawer1.treeNode.stakeAuthority,
      newWithdrawerAuthority: treeNode1Withdrawer1.treeNode.withdrawAuthority,
    })
    try {
      const wrongBumpIx = await claimSettlementWrongBump({
        proof: treeNode1Withdrawer1.proof,
        claim: treeNode1Withdrawer1.treeNode.data.claim,
        configAccount: config.publicKey,
        bondAccount: bondAccount,
        settlementAccount: settlementAccount1,
        stakeAccountFrom: stakeAccount1,
        stakeAccountTo: stakeAccountToWrongBump,
        stakeAccountStaker: treeNode1Withdrawer1.treeNode.stakeAuthority,
        stakeAccountWithdrawer: treeNode1Withdrawer1.treeNode.withdrawAuthority,
        voteAccount: voteAccount1,
      })
      await provider.sendIx([signer(rentPayer)], wrongBumpIx, instruction)
      throw new Error('should have failed; already claimed')
    } catch (e) {
      expect((e as Error).message).toMatch('custom program error: 0x7d6')
    }

    const treeNode1Withdrawer2 = treeNodeBy(voteAccount1, withdrawer2)
    const stakeAccountTreeNode1Withdrawer2 = await createStakeAccount({
      provider,
      lamports: 369 * LAMPORTS_PER_SOL,
      voteAccount: voteAccount1,
      newStakerAuthority: treeNode1Withdrawer2.treeNode.stakeAuthority,
      newWithdrawerAuthority: treeNode1Withdrawer2.treeNode.withdrawAuthority,
    })
    const { instruction: ixWrongMerkleTreeNodes } =
      await claimSettlementInstruction({
        program,
        claimAmount: treeNode1Withdrawer2.treeNode.data.claim,
        merkleProof: treeNode1Withdrawer2.proof,
        settlementAccount: settlementAccount1,
        stakeAccountFrom: stakeAccount1,
        stakeAccountTo: stakeAccountTreeNode1Withdrawer2,
        stakeAccountStaker: treeNode1Withdrawer2.treeNode.stakeAuthority,
        stakeAccountWithdrawer: treeNode1Withdrawer2.treeNode.withdrawAuthority,
      })
    try {
      await provider.sendIx([], ixWrongMerkleTreeNodes)
      throw new Error('should have failed; wrong stake account')
    } catch (e) {
      verifyError(e, Errors, 6033, 'exceeded number of claimable nodes')
    }

    const treeNode2Withdrawer2 = treeNodeBy(voteAccount2, withdrawer2)
    const stakeAccountTreeNode2Withdrawer2 = await createStakeAccount({
      provider,
      lamports: 32 * LAMPORTS_PER_SOL,
      voteAccount: voteAccount1,
      newStakerAuthority: treeNode2Withdrawer2.treeNode.stakeAuthority,
      newWithdrawerAuthority: treeNode2Withdrawer2.treeNode.withdrawAuthority,
    })
    const { instruction: treeNode2Withdrawer2Ix } =
      await claimSettlementInstruction({
        program,
        claimAmount: treeNode2Withdrawer2.treeNode.data.claim,
        merkleProof: treeNode2Withdrawer2.proof,
        settlementAccount: settlementAccount2,
        stakeAccountFrom: stakeAccount2,
        stakeAccountTo: stakeAccountTreeNode2Withdrawer2,
        stakeAccountStaker: treeNode2Withdrawer2.treeNode.stakeAuthority,
        stakeAccountWithdrawer: treeNode2Withdrawer2.treeNode.withdrawAuthority,
      })
    try {
      await provider.sendIx([], treeNode2Withdrawer2Ix)
      throw new Error(
        'should have failed; over claimed (wrong argument on settlement)'
      )
    } catch (e) {
      verifyError(e, Errors, 6032, 'the max total claim')
    }

    const treeNode2Withdrawer1 = treeNodeBy(voteAccount2, withdrawer1)
    const stakeAccountTreeNode2Withdrawer1 = await createStakeAccount({
      provider,
      lamports: 11 * LAMPORTS_PER_SOL,
      voteAccount: voteAccount1,
      newStakerAuthority: treeNode2Withdrawer1.treeNode.stakeAuthority,
      newWithdrawerAuthority: treeNode2Withdrawer1.treeNode.withdrawAuthority,
    })
    const { instruction: ixWrongStakeAccount } =
      await claimSettlementInstruction({
        program,
        claimAmount: treeNode2Withdrawer1.treeNode.data.claim,
        merkleProof: treeNode2Withdrawer1.proof,
        settlementAccount: settlementAccount2,
        stakeAccountFrom: stakeAccount1,
        stakeAccountTo: stakeAccountTreeNode2Withdrawer1,
        stakeAccountStaker: treeNode2Withdrawer1.treeNode.stakeAuthority,
        stakeAccountWithdrawer: treeNode2Withdrawer1.treeNode.withdrawAuthority,
      })
    try {
      await provider.sendIx([], ixWrongStakeAccount)
      throw new Error('should have failed; wrong stake account')
    } catch (e) {
      verifyError(e, Errors, 6036, 'not funded under the settlement')
    }

    const { instruction: treeNode2Withdrawer1Ix } =
      await claimSettlementInstruction({
        program,
        claimAmount: treeNode2Withdrawer1.treeNode.data.claim,
        merkleProof: treeNode2Withdrawer1.proof,
        settlementAccount: settlementAccount2,
        stakeAccountFrom: stakeAccount2,
        stakeAccountTo: stakeAccountTreeNode2Withdrawer1,
        stakeAccountStaker: treeNode2Withdrawer1.treeNode.stakeAuthority,
        stakeAccountWithdrawer: treeNode2Withdrawer1.treeNode.withdrawAuthority,
      })
    await provider.sendIx([], treeNode2Withdrawer1Ix)

    await warpToNotBeClaimable()

    const treeNode1Withdrawer3 = treeNodeBy(voteAccount1, withdrawer3)
    const stakeAccountTreeNode1Withdrawer3 = await createStakeAccount({
      provider,
      lamports: 11 * LAMPORTS_PER_SOL,
      voteAccount: voteAccount1,
      newStakerAuthority: treeNode1Withdrawer3.treeNode.stakeAuthority,
      newWithdrawerAuthority: treeNode1Withdrawer3.treeNode.withdrawAuthority,
    })
    const { instruction: ixTooLate, settlementClaimAccount: accTooLate } =
      await claimSettlementInstruction({
        program,
        claimAmount: treeNode1Withdrawer3.treeNode.data.claim,
        merkleProof: treeNode1Withdrawer3.proof,
        settlementAccount: settlementAccount1,
        stakeAccountFrom: stakeAccount1,
        stakeAccountTo: stakeAccountTreeNode1Withdrawer3,
        stakeAccountStaker: treeNode1Withdrawer3.treeNode.stakeAuthority,
        stakeAccountWithdrawer: treeNode1Withdrawer3.treeNode.withdrawAuthority,
      })
    try {
      await provider.sendIx([], ixTooLate)
      throw new Error('should have failed; too late to claim')
    } catch (e) {
      verifyError(e, Errors, 6023, 'already expired')
    }
    assertNotExist(provider, accTooLate)
  })

  async function warpToNotBeClaimable() {
    await warpOffsetEpoch(provider, epochsToClaimSettlement + 1)
  }

  async function claimSettlementWrongBump({
    proof,
    claim,
    configAccount,
    bondAccount,
    settlementAccount,
    stakeAccountFrom,
    stakeAccountTo,
    stakeAccountStaker,
    stakeAccountWithdrawer,
    voteAccount,
  }: {
    proof: number[][]
    claim: BN | number
    configAccount: PublicKey
    bondAccount: PublicKey
    settlementAccount: PublicKey
    stakeAccountFrom: PublicKey
    stakeAccountTo: PublicKey
    stakeAccountStaker: PublicKey
    stakeAccountWithdrawer: PublicKey
    voteAccount: PublicKey
  }) {
    let [, bump] = settlementClaimAddress(
      {
        settlement: settlementAccount,
        stakeAccountStaker,
        voteAccount,
        stakeAccountWithdrawer,
        claim,
      },
      program.programId
    )
    let settlementAccountWrongBump: PublicKey | undefined
    const treeNodeHash = MerkleTreeNode.hash({
      stakeAuthority: stakeAccountStaker,
      withdrawAuthority: stakeAccountWithdrawer,
      voteAccount: voteAccount,
      claim: claim,
    })
    const seeds = [
      SETTLEMENT_CLAIM_SEED,
      settlementAccount.toBytes(),
      treeNodeHash.buffer,
    ]
    while (settlementAccountWrongBump === undefined && bump > 0) {
      bump--
      const seedsWithBump = seeds.concat(Buffer.from([bump]))
      try {
        settlementAccountWrongBump = PublicKey.createProgramAddressSync(
          seedsWithBump,
          program.programId
        )
      } catch (e) {
        if (e instanceof TypeError) {
          throw e
        }
      }
    }
    // console.log('correct claim settlement', correct, 'wrong bump', settlementAccountWrongBump?.toBase58(), 'with bump', bump)
    return await program.methods
      .claimSettlement({
        proof,
        treeNodeHash: treeNodeHash.words,
        stakeAccountStaker,
        stakeAccountWithdrawer,
        claim: new BN(claim),
      })
      .accounts({
        config: configAccount,
        bond: bondAccount,
        settlement: settlementAccount,
        settlementClaim: settlementAccountWrongBump,
        stakeAccountFrom,
        stakeAccountTo,
        rentPayer: provider.walletPubkey,
        systemProgram: SystemProgram.programId,
        stakeHistory: SYSVAR_STAKE_HISTORY_PUBKEY,
        clock: SYSVAR_CLOCK_PUBKEY,
        stakeProgram: StakeProgram.programId,
      })
      .instruction()
  }
})
