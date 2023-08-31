use casper_types::ApiError;

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum Error {
    InvalidContext = 10000,
    ContractLocked,
    OnlyOwnerCanRevoke,
    ContractPaused,
    InvalidKey,
    FailedToCreateDictionary,
    ContractAlreadyInitialized,
    Phantom,
    FailedToGetArgBytes,
    InvalidTestingMode,
    TooManyConsumers,
    InsufficientBalance,
    InvalidConsumer,
    InvalidSubscription,
    OnlyCallableFromLink,
    InvalidCalldata,
    MustBeSubOwner,
    PendingRequestExists,
    MustBeRequestedOwner,
    BalanceInvariantViolated,
    InvalidRequestConfirmations,
    GasLimitTooBig,
    NumWordsTooBig,
    ProvingKeyAlreadyRegistered,
    NoSuchProvingKey,
    InvalidLinkWeiPrice,
    InsufficientGasForConsumer,
    NoCorrespondingRequest,
    IncorrectCommitment,
    BlockhashNotInStore,
    PaymentTooLarge,
    Reentrant,
    FailedToDecodeInputBytes,
    InvalidProvingKeyLength,
    InvalidXCordinate,
    InvalidYCordinate,
    KeyNotOnCurve,
    BadWitness,
    BadLinearCombinationWithGenerator,
    PointsSumMustBeDistinct,
    FirstMulCheckFailed,
    SecondMulCheckFailed,
    ScalarZero,
    InvZMustBeInverseOfZ,
    InvalidProof,
    ErrorGettingSignature,
    InvalidSignature,
}

impl From<Error> for ApiError {
    fn from(e: Error) -> Self {
        ApiError::User(e as u16)
    }
}

pub fn as_u16(err: Error) -> u16 {
    err as u16
}
