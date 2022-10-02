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
            // According to spec, length can be 0, 1 or 2 bytes
            // 2 bytes is prefaced by 00 to differentiate between 1 byte lengths
            if cfg!(feature = "longer_payloads") && p.len() > u8::MAX as usize {
                buffer.push(0u8);
                buffer.push(p.len() as u8);
                buffer.push((p.len() >> 8) as u8);
                buffer.append(&mut p);
            } else {
                buffer.push(p.len() as u8);
                buffer.append(&mut p);
            }
        }

        if let Some(l) = le {
            buffer.push(l);
        }

        buffer
    }
}
