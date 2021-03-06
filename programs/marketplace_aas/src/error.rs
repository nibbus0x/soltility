use {
  anchor_lang::prelude::*,
};

#[error_code]
pub enum CustomError {
  #[msg("Nft already listed. Must delist first to update listing.")]
  NftListed,
  #[msg("Nft not listed.")]
  NftUnlisted,
  #[msg("Nft is not part of the passed collection.")]
  MismatchedNft,
  #[msg("Person attempting to delist is not the one who originally listed.")]
  UnknownSeller,
  #[msg("Passed collection id does not match one obtained from passed mint.")]
  InvalidCollectionId,
  #[msg("Passed collection id is not verified.")]
  CollectionIdUnverified,
  #[msg("Passed Creator AccountInfo is missing or incorrect.")]
  BadCreatorInfo,
  #[msg("Invalid fee, must be in basis points.")]
  InvalidFee,
  #[msg("Attempting to list token/nft on incorrect marketplace.")]
  WrongMarketplace,
  #[msg("Missing required account info to execute instruction.")]
  MissingAccountInfo,
  #[msg("Passed vault account info does not match key argument passed to instruction.")]
  InvalidAccountInfo,
  #[msg("Signing organization authority does not match authority on record.")]
  IncorrectOrgAuthority,
  #[msg("A bump required for instruction processing is missing.")]
  MissingBump,
  #[msg("Provided escrow nonce does not match expected value.")]
  MismatchedEscrowNonce,
  #[msg("Too many tokens in either offerings or requestings array.")]
  TooManyOfferings,
  #[msg("Nothing to trade.")]
  EmptyTrade,
  #[msg("Cannot propose trade with yourself.")]
  SelfTrade,
  #[msg("Offerings list contains duplicate mint keys.")]
  DuplicateMint,
  #[msg("Passed remaining accounts array does not contain the expected accounts.")]
  InvalidRemainingAccounts,
  #[msg("Missing a required mint key in offering.")]
  MissingOfferingMint,
  #[msg("Signer key does not match the offerer stored in account.")]
  InvalidOfferer,
  #[msg("Signer key does not match the offeree stored in account.")]
  InvalidOfferee,
  #[msg("Provided reward mint does not match the mint on record.")]
  IncorrectRewardMint,
  #[msg("The locking period for this nft has not ended yet.")]
  NftLocked,
  #[msg("Mint authority of provided reward mint cannot be transferred. Either it is null or current authority has not signed.")]
  InvalidCurrentMintAuth,
  #[msg("Provided token account is empty.")]
  TokenAccountEmpty,
}