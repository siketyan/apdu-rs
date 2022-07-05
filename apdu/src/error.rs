/// An error that was returned from the card or reader.
#[derive(Debug, apdu_derive::Response, thiserror::Error)]
pub enum Error {
    #[apdu(0x06, _)]
    #[apdu(0x6E, _)]
    #[error("Class not supported")]
    ClassNotSupported,

    #[apdu(0x61, _)]
    #[error("Command successfully executed; {0} bytes of data are still available")]
    ResponseBytesStillAvailable(#[sw2] u8),

    #[apdu(0x62, 0x00)]
    #[apdu(0x64, 0x00)]
    #[error("NVRAM not changed")]
    NvRamNotChanged,

    #[apdu(0x62, 0x81)]
    #[error("Part of returned data may be corrupted")]
    DataCorrupted,

    #[apdu(0x62, 0x82)]
    #[error("End of file")]
    EndOfFile,

    #[apdu(0x62, 0x83)]
    #[error("The file is already invalidated")]
    FileInvalidated,

    #[apdu(0x62, 0x84)]
    #[error("The file has an invalid format")]
    FileInvalid,

    #[apdu(0x62, 0x85)]
    #[error("No input available from a sensor on the card")]
    NoInputAvailable,

    #[apdu(0x62, 0xA2)]
    #[error("Wrong R-MAC found")]
    WrongRMac,

    #[apdu(0x62, 0xA4)]
    #[error("The card is locked")]
    CardLocked,

    #[apdu(0x62, 0xC0..=0xCF)]
    #[error("Counter with value {0}")]
    CounterValue(
        #[sw2]
        #[mask(0x0F)]
        u8,
    ),

    #[apdu(0x62, 0xF1)]
    #[error("Wrong C-MAC found")]
    WrongCmac,

    #[apdu(0x62, 0xF3)]
    #[error("Internal reset occurred")]
    InternalReset,

    #[apdu(0x62, 0xF5)]
    #[error("The default agent is locked")]
    DefaultAgentLocked,

    #[apdu(0x62, 0xF7)]
    #[error("The cardholder is locked")]
    CardholderLocked,

    #[apdu(0x62, 0xF8)]
    #[error("Basement is the current agent")]
    BasementIsCurrentAgent,

    #[apdu(0x63, 0x00)]
    #[apdu(0x65, 0x00)]
    #[error("NVRAM changed")]
    NvRamChanged,

    #[apdu(0x63, 0x81)]
    #[error("The file was filled up by the last write: loading or updating it is not allowed")]
    FileFilledUp,

    #[apdu(0x63, 0x82)]
    #[error("The card key is not supported")]
    CardKeyNotSupported,

    #[apdu(0x63, 0x83)]
    #[error("The reader key is not supported")]
    ReaderKeyNotSupported,

    #[apdu(0x63, 0x84)]
    #[error("Plaintext transmission is not supported")]
    PlaintextTransmissionNotSupported,

    #[apdu(0x63, 0x85)]
    #[error("Secured transmission is not supported")]
    SecuredTransmissionNotSupported,

    #[apdu(0x63, 0x86)]
    #[error("Volatile memory is not available")]
    VmNotAvailable,

    #[apdu(0x63, 0x87)]
    #[error("Non-volatile memory is not available")]
    NvmNotAvailable,

    #[apdu(0x63, 0x88)]
    #[error("The key number is invalid")]
    KeyNumberInvalid,

    #[apdu(0x63, 0x89)]
    #[error("The key length is not correct")]
    KeyLengthNotCorrect,

    #[apdu(0x63, 0xC0..=0xCF)]
    #[error("Verify failed: {0} tries left")]
    VerifyFailed(
        #[sw2]
        #[mask(0x0F)]
        u8,
    ),

    #[apdu(0x63, 0xF1)]
    #[error("Received insufficient data")]
    InsufficientData,

    #[apdu(0x63, 0xF2)]
    #[error("Received insufficient data: proactive command pending")]
    InsufficientDataPending,

    #[apdu(0x64, 0x01)]
    #[error("Command timed out: immediate response required by the card")]
    CommandTimeout,

    #[apdu(0x65, 0x01)]
    #[error("Failed to write to memory: maybe a hardware issue?")]
    MemoryWriteFailed,

    #[apdu(0x65, 0x81)]
    #[error("Memory failure occurred: maybe a hardware issue?")]
    MemoryFailure,

    #[apdu(0x66, 0x00)]
    #[error("Receiving timeout")]
    ReceivingTimeout,

    #[apdu(0x66, 0x01)]
    #[error("Parity error occurred while receiving")]
    ReceivingParityError,

    #[apdu(0x66, 0x02)]
    #[error("Checksum mismatch")]
    WrongChecksum,

    #[apdu(0x66, 0x03)]
    #[error("The DF file has no FCI: file control information")]
    DfFileNoFci,

    #[apdu(0x66, 0x04)]
    #[error("No SF or KF file found under the DF file")]
    NoSfOrKfFound,

    #[apdu(0x66, 0x69)]
    #[error("Incorrect encryption or decryption padding found")]
    IncorrectCryptoPadding,

    #[apdu(0x67, _)]
    #[error("Wrong length found")]
    WrongLength,

    #[apdu(0x68, 0x00)]
    #[error("The requested function is not supported by the card")]
    NotSupportedByCard,

    #[apdu(0x68, 0x81)]
    #[error("Logical channel is not supported")]
    LogicalChannelNotSupported,

    #[apdu(0x68, 0x82)]
    #[error("Secure messaging is not supported")]
    SecureMessagingNotSupported,

    #[apdu(0x68, 0x83)]
    #[error("Last command expected on the chain")]
    LastCommandExpected,

    #[apdu(0x68, 0x84)]
    #[error("Command chaining is not supported")]
    CommandChainingNotSupported,

    #[apdu(0x69, 0x00)]
    #[error("The command is not allowed")]
    CommandNotAllowed,

    #[apdu(0x69, 0x01)]
    #[error("The command is not accepted")]
    CommandNotAccepted,

    #[apdu(0x69, 0x81)]
    #[error("The command is incompatible with the file structure")]
    CommandIncompatible,

    #[apdu(0x69, 0x82)]
    #[error("Security condition is not satisfied")]
    SecurityConditionNotSatisfied,

    #[apdu(0x69, 0x83)]
    #[error("Authentication method is blocked")]
    AuthenticationMethodBlocked,

    #[apdu(0x69, 0x84)]
    #[error("Referenced data is reversibly blocked or invalidated")]
    ReferencedDataBlocked,

    #[apdu(0x69, 0x85)]
    #[error("Conditions of use is not satisfied")]
    ConditionNotSatisfied,

    #[apdu(0x69, 0x86)]
    #[error("Command not allowed: no current EF")]
    NoCurrentEf,

    #[apdu(0x69, 0x87)]
    #[error("Expected Secure Messaging object is missing")]
    SmObjectMissing,

    #[apdu(0x69, 0x88)]
    #[error("Incorrect Secure Messaging object found")]
    SmObjectIncorrect,

    #[apdu(0x69, 0x96)]
    #[error("The data must be updated again")]
    DataMustBeUpdated,

    #[apdu(0x69, 0xE1)]
    #[error("POL1 of the currently enabled profile prevents the action")]
    ActionPrevented,

    #[apdu(0x69, 0xF0)]
    #[error("Permission denied")]
    PermissionDenied,

    #[apdu(0x69, 0xF1)]
    #[apdu(0x99, 0x86)]
    #[error("Permission denied: missing privileges")]
    MissingPrivileges,

    #[apdu(0x6A, 0x00)]
    #[apdu(0x6B, 0x00)]
    #[error("P1 and/or P2 are incorrect")]
    IncorrectParameters,

    #[apdu(0x6A, 0x80)]
    #[error("The parameters in the data field is incorrect")]
    IncorrectDataFieldParameters,

    #[apdu(0x6A, 0x81)]
    #[error("The function is not supported")]
    FunctionNotSupported,

    #[apdu(0x6A, 0x82)]
    #[error("The file is not found")]
    FileNotFound,

    #[apdu(0x6A, 0x83)]
    #[error("The record is not found")]
    RecordNotFound,

    #[apdu(0x6A, 0x84)]
    #[error("Insufficient memory space in the record or file")]
    InsufficientMemory,

    #[apdu(0x6A, 0x85)]
    #[error("Lc value is inconsistent with the TLV structure")]
    LcInconsistent,

    #[apdu(0x6A, 0x86)]
    #[error("The referenced data is not found")]
    ReferencedDataNotFound,

    #[apdu(0x6A, 0x89)]
    #[error("The file already exists")]
    FileAlreadyExists,

    #[apdu(0x6A, 0x8A)]
    #[error("The name of DF already exists")]
    DfNameAlreadyExists,

    #[apdu(0x6A, 0xF0)]
    #[error("Wrong parameter value found")]
    WrongParameterValue,

    #[apdu(0x6B, _)]
    #[error("Incorrect reference found")]
    IncorrectReference,

    #[apdu(0x6C, 0x00)]
    #[error("Incorrect length found")]
    IncorrectLength,

    #[apdu(0x6C, _)]
    #[error("Incorrect length found: it is {0} actually")]
    IncorrectLengthOf(#[sw2] u8),

    #[apdu(0x6D, _)]
    #[error("The instruction is not supported or invalid")]
    InstructionInvalid,

    #[apdu(0x6F, 0x00)]
    #[error("Command aborted: internal error occurred")]
    CommandAborted,

    #[apdu(0x6F, 0xFF)]
    #[error("The card is dead")]
    Dead,

    #[apdu(0x6F, _)]
    #[error("Internal error occurred: code {0}")]
    InternalError(#[sw2] u8),

    #[apdu(0x90, 0x04)]
    #[error("PIN is not verified successfully: 3 or more tries left")]
    PinNotVerified,

    #[apdu(0x90, 0x80)]
    #[error("Unblock try counter has reached to zero")]
    UnblockTryCounterZero,

    #[apdu(0x92, 0x10)]
    #[error("Insufficient memory: no more storage is available")]
    NoStorageAvailable,

    #[apdu(0x92, 0x40)]
    #[error("Failed to write to EEPROM")]
    EepromFailed,

    #[apdu(0x93, 0x03)]
    #[error("The application is permanently locked")]
    ApplicationLocked,

    #[apdu(0x94, 0x00)]
    #[error("No EF is selected")]
    NoEfSelected,

    #[apdu(0x94, 0x02)]
    #[error("Access range exceeded")]
    AccessRangeExceeded,

    #[apdu(0x94, 0x04)]
    #[error("FID, record or comparison pattern is not found")]
    FidNotFound,

    #[apdu(0x94, 0x06)]
    #[error("Required MAC is unavailable")]
    MacUnavailable,

    #[apdu(0x94, 0x08)]
    #[error("The selected file type does not match command")]
    FileTypeNotMatch,

    #[apdu(0x98, 0x02)]
    #[error("No PIN is defined")]
    NoPinDefined,

    #[apdu(0x98, 0x04)]
    #[error("Authentication failed: access conditions are not satisfied")]
    AuthenticationFailed,

    #[apdu(0x98, 0x35)]
    #[error("ASK RANDOM or GIVE RANDOM was not executed")]
    RandomNotExecuted,

    #[apdu(0x98, 0x40)]
    #[error("PIN verification was not successful")]
    PinVerificationFailed,

    #[apdu(0x98, 0x50)]
    #[error("INCREASE or DECREASE was not executed: reached to its limit")]
    IncreaseOrDecreaseNotExecuted,

    #[apdu(0x98, 0x62)]
    #[error("Error occurred during the authentication")]
    AuthenticationError,

    #[apdu(0x9D, 0x05)]
    #[error("Incorrect certificate type")]
    IncorrectCertificateType,

    #[apdu(0x9D, 0x07)]
    #[error("Incorrect size of the session data")]
    IncorrectSessionDataSize,

    #[apdu(0x9D, 0x08)]
    #[error("Incorrect size of DIR file record")]
    IncorrectDirFileRecordSize,

    #[apdu(0x9D, 0x09)]
    #[error("Incorrect size of FCI record")]
    IncorrectFciRecordSize,

    #[apdu(0x9D, 0x0A)]
    #[error("Incorrect size of code")]
    IncorrectCodeSize,

    #[apdu(0x9D, 0x10)]
    #[error("Insufficient memory found to load the application")]
    InsufficientMemoryLoadApplication,

    #[apdu(0x9D, 0x11)]
    #[error("Invalid AID found")]
    InvalidAid,

    #[apdu(0x9D, 0x12)]
    #[error("Duplicated AID found")]
    DuplicatedAid,

    #[apdu(0x9D, 0x13)]
    #[error("The application was loaded previously")]
    ApplicationLoadedPreviously,

    #[apdu(0x9D, 0x14)]
    #[error("Application history is full")]
    ApplicationHistoryFull,

    #[apdu(0x9D, 0x15)]
    #[error("Application is not opened")]
    ApplicationNotOpen,

    #[apdu(0x9D, 0x17)]
    #[error("Invalid offset found")]
    InvalidOffset,

    #[apdu(0x9D, 0x18)]
    #[error("The application is already loaded")]
    ApplicationLoaded,

    #[apdu(0x9D, 0x19)]
    #[error("Invalid certificate found")]
    InvalidCertificate,

    #[apdu(0x9D, 0x1A)]
    #[error("Invalid signature found")]
    InvalidSignature,

    #[apdu(0x9D, 0x1B)]
    #[error("Invalid KTU found")]
    InvalidKtu,

    #[apdu(0x9D, 0x1D)]
    #[error("MTU controls are not set")]
    MsmControlsNotSet,

    #[apdu(0x9D, 0x1E)]
    #[error("Application signature does not exist")]
    ApplicationSignatureNotExists,

    #[apdu(0x9D, 0x1F)]
    #[error("KTU does not exist")]
    KtuNotExists,

    #[apdu(0x9D, 0x20)]
    #[error("The application was not loaded")]
    ApplicationNotLoaded,

    #[apdu(0x9D, 0x21)]
    #[error("Invalid data length found in open command")]
    InvalidOpenCommandLength,

    #[apdu(0x9D, 0x30)]
    #[error("Invalid start address found in check")]
    InvalidStartAddress,

    #[apdu(0x9D, 0x31)]
    #[error("Invalid length found in check")]
    InvalidLength,

    #[apdu(0x9D, 0x32)]
    #[error("Illegal memory check area found")]
    IllegalMemoryCheckArea,

    #[apdu(0x9D, 0x40)]
    #[error("Invalid ciphertext found of MSM controls")]
    InvalidMsmControlsCiphertext,

    #[apdu(0x9D, 0x41)]
    #[error("MSM controls are already set")]
    MsmControlsAlreadySet,

    #[apdu(0x9D, 0x42)]
    #[error("Data length of MSM controls is less than 2 bytes")]
    MsmControlsLengthTooShort,

    #[apdu(0x9D, 0x44)]
    #[error("Excess ciphertext found of MSM controls")]
    ExcessMsmControlsCiphertext,

    #[apdu(0x9D, 0x45)]
    #[error("Verification failed for MSM controls")]
    MsmControlsVerificationFailed,

    #[apdu(0x9D, 0x50)]
    #[error("Invalid MCD issuer production ID found")]
    InvalidMcdIssuerProductionId,

    #[apdu(0x9D, 0x51)]
    #[error("Invalid MCD issuer ID found")]
    InvalidMcdIssuerId,

    #[apdu(0x9D, 0x52)]
    #[error("Invalid data date of MSM controls was set")]
    InvalidMsmControlsDataDate,

    #[apdu(0x9D, 0x53)]
    #[error("Invalid MCD number found")]
    InvalidMcdNumber,

    #[apdu(0x9D, 0x60)]
    #[error("MAC verification failed")]
    MacVerificationFailed,

    #[apdu(0x9D, 0x61)]
    #[error("Reached to the maximum number of unblocks")]
    UnblockLimitReached,

    #[apdu(0x9D, 0x62)]
    #[error("The card was not blocked")]
    CardNotBlocked,

    #[apdu(0x9D, 0x63)]
    #[error("Crypto functions are not available")]
    CryptoFunctionsNotAvailable,

    #[apdu(0x9D, 0x64)]
    #[error("No applications are loaded")]
    NoApplicationLoaded,

    #[apdu(_, _)]
    #[error("Unknown APDU error ({0:#X}, {1:#X})")]
    Unknown(u8, u8),
}
