import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Benchmarking } from '../target/types/benchmarking';

describe('benchmarking', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Benchmarking as Program<Benchmarking>;

  it('1', async () => {
    // Add your test here.
    const tx = await program.rpc.testRawImmutable({});
    console.log("Your transaction signature", tx);
  });
  it('2', async () => {
    // Add your test here.
    const tx = await program.rpc.testExtend({});
    console.log("Your transaction signature", tx);
  });
  it('3', async () => {
    // Add your test here.
    const tx = await program.rpc.testPush({});
    console.log("Your transaction signature", tx);
  });
  it('4', async () => {
    // Add your test here.
    const tx = await program.rpc.testRawMutableHybridExtend({});
    console.log("Your transaction signature", tx);
  });
  it('5', async () => {
    // Add your test here.
    const tx = await program.rpc.testRawMutableHybridPush({});
    console.log("Your transaction signature", tx);
  });
  it('6', async () => {
    // Add your test here.
    const tx = await program.rpc.testMsg({});
    console.log("Your transaction signature", tx);
  });
});
