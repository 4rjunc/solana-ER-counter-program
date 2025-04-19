use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum ProgramInstruction {
    InitializeCounter,                    // Initialize counter to 0 (Base Layer)
    IncreaseCounter { increase_by: u64 }, // Increment counter by X (Base Layer or ER)
    Delegate,                             // Delegate counter from Base Layer to ER
    CommitAndUndelegate,                  // Schedule sync from ER to Base Layer AND undelegate (ER)
    Commit,                               // Schedule sync from ER to Base Layer (ER)
    // Vec<Vec<u8>> represents a collection of seeds, where each seed is a byte array
    // This flexible structure allows passing multiple seeds of varying lengths
    Undelegate { pda_seeds: Vec<Vec<u8>> }, // Undelegate counter on Base Layer (through validator CPI)
}

#[derive(BorshDeserialize)]
struct IncreaseCounterPayload {
    increase_by: u64,
}

impl ProgramInstruction {
    // &[u8] since instruction are in byte array
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // check the input has at least 8 bytes for the variant
        if input.len() < 8 {
            return Err(ProgramError::InvalidInstructionData);
        };

        // takes first 8 bytes as variant
        let (ix_discriminator, rest) = input.split_at(8);

        //Match instruction discriminator with the process and deserialize payload
        //The values like [1, 0, 0, 0, 0, 0, 0, 0] and [2, 0, 0, 0, 0, 0, 0, 0] are indeed
        //8-byte arrays where the first byte represents the instruction type (1, 2, 3, etc.)
        //in little-endian format, with the remaining bytes padded with zeros.
        Ok(match ix_discriminator {
            [0, 0, 0, 0, 0, 0, 0, 0] => Self::InitializeCounter,
            [1, 0, 0, 0, 0, 0, 0, 0] => {
                // rest is the remaining bytes after the 8-byte instruction discriminator has been split off from the input data.
                // IncreaseCounterPayload::try_from_slice(rest) attempts to deserialize these remaining
                // bytes into an IncreaseCounterPayload struct according to the Borsh serialization format.
                let payload = IncreaseCounterPayload::try_from_slice(rest)?;
                Self::IncreaseCounter {
                    increase_by: payload.increase_by,
                }
            }
            [2, 0, 0, 0, 0, 0, 0, 0] => Self::Delegate,
            [3, 0, 0, 0, 0, 0, 0, 0] => Self::CommitAndUndelegate,
            [4, 0, 0, 0, 0, 0, 0, 0] => Self::Commit,
            [196, 28, 41, 206, 48, 37, 51, 167] => {
                let pda_seeds: Vec<Vec<u8>> = Vec::<Vec<u8>>::try_from_slice(rest)?;
                Self::Undelegate { pda_seeds }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
