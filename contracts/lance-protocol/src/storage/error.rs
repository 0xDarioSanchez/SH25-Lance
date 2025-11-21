use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    ContractInitialized = 0,
    ContractNotInitialized = 1,
    NotAuthorized = 2,
    UserNotFound = 3,
    ServiceNotFound = 4,
    DisputeNotFound = 5,
    InsufficientTime = 6,
    InvalidDuration = 7,
    BalanceIsZero = 8,
    InvalidStatus = 9,
    InvalidService = 10,
    InvalidServiceStatus = 11,
    DisputeAlreadyCreated = 12,
    InvalidDisputeStatus = 13,
    JudgeNotFound = 14,
    InvalidAmount = 15,
    NoPositionInBlend = 16,
    InsufficientFundsInBlend = 17,
    NoTokensToLend = 18,
    DisputeAlreadyResolved = 19,
    DisputeNotOpen = 20,
    JudgeNotAllowedToVote = 21,
    JudgeAlreadyVoted = 22,
    InvalidReveal = 23,
    JudgeNotCommitted = 24,
    UnauthorizedSigner = 25,
    InvalidKey = 26,
    TallySeedError = 27,
    NoAnonymousVotingConfig = 28,
    AdminNotFound = 29,
}
