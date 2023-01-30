import { HeliumEntityManager } from "@helium/idls/lib/types/helium_entity_manager";
import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";
import { heliumEntityManagerResolvers } from "./resolvers";

export const init = async (
  provider: AnchorProvider,
  programId: PublicKey = PROGRAM_ID,
  idl?: Idl | null
): Promise<Program<HeliumEntityManager>> => {
  if (!idl) {
    idl = await Program.fetchIdl(programId, provider);
  }

  const heliumEntityManager = new Program<HeliumEntityManager>(
    idl as HeliumEntityManager,
    programId,
    provider,
    undefined,
    () => heliumEntityManagerResolvers
  ) as Program<HeliumEntityManager>;

  return heliumEntityManager;
};
