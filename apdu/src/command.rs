//! High-level API to compose commands easily.

/// Default CLA (class) value of commands
const CLA_DEFAULT: u8 = 0x00;

const INS_SELECT_FILE: u8 = 0xA4;
const INS_READ_BINARY: u8 = 0xB0;
const INS_VERIFY: u8 = 0x20;

macro_rules! impl_into_vec {
    ($name: ty) => {
        impl<'a> From<$name> for Vec<u8> {
            fn from(cmd: $name) -> Self {
                crate::Command::from(cmd).into()
            }
        }
    };
}

/// `SELECT FILE` (0xA4) command.
pub struct SelectFileCommand<'a> {
    p1: u8,
    p2: u8,
    payload: &'a [u8],
}

impl<'a> SelectFileCommand<'a> {
    /// Constructs a `SELECT FILE` command.
    pub fn new(p1: u8, p2: u8, payload: &'a [u8]) -> Self {
        Self { p1, p2, payload }
    }
}

impl<'a> From<SelectFileCommand<'a>> for crate::Command<'a> {
    fn from(cmd: SelectFileCommand<'a>) -> Self {
        match cmd.payload.len() {
            0 => Self::new(CLA_DEFAULT, INS_SELECT_FILE, cmd.p1, cmd.p2),
            _ => Self::new_with_payload(CLA_DEFAULT, INS_SELECT_FILE, cmd.p1, cmd.p2, cmd.payload),
        }
    }
}

impl_into_vec!(SelectFileCommand<'a>);

/// Constructs a `SELECT FILE` command.
pub fn select_file(p1: u8, p2: u8, payload: &[u8]) -> SelectFileCommand {
    SelectFileCommand::new(p1, p2, payload)
}

/// `READ BINARY` (0xB0) command.
pub struct ReadBinaryCommand {
    p1: u8,
    p2: u8,
    le: u8,
}

impl ReadBinaryCommand {
    /// Constructs a `READ BINARY` command.
    pub fn new(p1: u8, p2: u8, le: u8) -> Self {
        Self { p1, p2, le }
    }
}

impl<'a> From<ReadBinaryCommand> for crate::Command<'a> {
    fn from(cmd: ReadBinaryCommand) -> Self {
        Self::new_with_le(CLA_DEFAULT, INS_READ_BINARY, cmd.p1, cmd.p2, cmd.le.into())
    }
}

impl_into_vec!(ReadBinaryCommand);

/// Constructs a `READ BINARY` command.
pub fn read_binary(p1: u8, p2: u8, le: u8) -> ReadBinaryCommand {
    ReadBinaryCommand::new(p1, p2, le)
}

/// `VERIFY` (0x20) command.
pub struct VerifyCommand<'a> {
    p2: u8,
    payload: &'a [u8],
}

impl<'a> VerifyCommand<'a> {
    /// Constructs a `VERIFY` command.
    pub fn new(p2: u8, payload: &'a [u8]) -> Self {
        Self { p2, payload }
    }
}

impl<'a> From<VerifyCommand<'a>> for crate::Command<'a> {
    fn from(cmd: VerifyCommand<'a>) -> Self {
        match cmd.payload.len() {
            0 => Self::new(CLA_DEFAULT, INS_VERIFY, 0x00, cmd.p2),
            _ => Self::new_with_payload(CLA_DEFAULT, INS_VERIFY, 0x00, cmd.p2, cmd.payload),
        }
    }
}

impl_into_vec!(VerifyCommand<'a>);

/// Constructs a `VERIFY` command.
pub fn verify(p2: u8, payload: &[u8]) -> VerifyCommand {
    VerifyCommand::new(p2, payload)
}
