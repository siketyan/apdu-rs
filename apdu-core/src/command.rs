/// Default CLA (class) value of commands
const CLA_DEFAULT: u8 = 0x00;

const INS_SELECT_FILE: u8 = 0xA4;
const INS_READ_BINARY: u8 = 0xB0;
const INS_VERIFY: u8 = 0x20;

/// An APDU command to be transmitted
pub struct Command {
    pub cla: u8,
    pub ins: u8,
    pub p1: u8,
    pub p2: u8,
    pub le: Option<u8>,
    pub payload: Option<Vec<u8>>,
}

impl Command {
    /// Constructs an command with CLA, INS, P1, and P2.
    /// No payloads will be transmitted or received.
    pub fn new(cla: u8, ins: u8, p1: u8, p2: u8) -> Self {
        Self {
            cla,
            ins,
            p1,
            p2,
            le: None,
            payload: None,
        }
    }

    /// Constructs an command with CLA, INS, P1, P2, and Le.
    /// A payload will be received.
    pub fn new_with_le(cla: u8, ins: u8, p1: u8, p2: u8, le: u8) -> Self {
        Self {
            cla,
            ins,
            p1,
            p2,
            le: Some(le),
            payload: None,
        }
    }

    /// Constructs an command with CLA, INS, P1, P2, and a payload.
    /// No payload will be received.
    pub fn new_with_payload(cla: u8, ins: u8, p1: u8, p2: u8, payload: Vec<u8>) -> Self {
        Self {
            cla,
            ins,
            p1,
            p2,
            le: None,
            payload: Some(payload),
        }
    }

    /// Constructs an command with CLA, INS, P1, P2, Le, and a payload.
    /// A payload will be received.
    pub fn new_with_payload_le(cla: u8, ins: u8, p1: u8, p2: u8, le: u8, payload: Vec<u8>) -> Self {
        Self {
            cla,
            ins,
            p1,
            p2,
            le: Some(le),
            payload: Some(payload),
        }
    }

    /// Constructs a `SELECT FILE` command.
    pub fn select_file(p1: u8, p2: u8, payload: Vec<u8>) -> Self {
        match payload.len() {
            0 => Self::new(CLA_DEFAULT, INS_SELECT_FILE, p1, p2),
            _ => Self::new_with_payload(CLA_DEFAULT, INS_SELECT_FILE, p1, p2, payload),
        }
    }

    /// Constructs a `READ BINARY` command.
    pub fn read_binary(p1: u8, p2: u8, le: u8) -> Self {
        Self::new_with_le(CLA_DEFAULT, INS_READ_BINARY, p1, p2, le)
    }

    /// Constructs a `VERIFY` command.
    pub fn verify(p2: u8, payload: Vec<u8>) -> Self {
        match payload.len() {
            0 => Self::new(CLA_DEFAULT, INS_VERIFY, 0x00, p2),
            _ => Self::new_with_payload(CLA_DEFAULT, INS_VERIFY, 0x00, p2, payload),
        }
    }
}

impl From<Command> for Vec<u8> {
    /// Converts the command into octets.
    fn from(command: Command) -> Self {
        let Command {
            cla,
            ins,
            p1,
            p2,
            le,
            payload,
        } = command;

        let mut buffer: Vec<u8> = vec![cla, ins, p1, p2];
        if let Some(mut p) = payload {
            buffer.push(p.len() as u8);
            buffer.append(&mut p);
        }

        if let Some(l) = le {
            buffer.push(l);
        }

        buffer
    }
}
