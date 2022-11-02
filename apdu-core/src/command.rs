/// An APDU command to be transmitted
pub struct Command<'a> {
    pub cla: u8,
    pub ins: u8,
    pub p1: u8,
    pub p2: u8,
    pub le: Option<u16>,
    pub payload: Option<&'a [u8]>,
}

impl<'a> Command<'a> {
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
    pub fn new_with_le(cla: u8, ins: u8, p1: u8, p2: u8, le: u16) -> Self {
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
    pub fn new_with_payload(cla: u8, ins: u8, p1: u8, p2: u8, payload: &'a [u8]) -> Self {
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
    pub fn new_with_payload_le(
        cla: u8,
        ins: u8,
        p1: u8,
        p2: u8,
        le: u16,
        payload: &'a [u8],
    ) -> Self {
        Self {
            cla,
            ins,
            p1,
            p2,
            le: Some(le),
            payload: Some(payload),
        }
    }

    /// Writes a serialised byte stream onto the mutable buffer.
    pub fn write(&self, buf: &mut [u8]) {
        let Command {
            cla,
            ins,
            p1,
            p2,
            le,
            payload,
        } = self;

        // &mut [u8] does not have push or extend methods,
        // So we are using a mutator to have a reference of the buffer and a cursor on them.
        struct Mutator<'a> {
            buf: &'a mut [u8],
            i: usize,
        }

        impl<'a> Mutator<'a> {
            fn push(&mut self, b: u8) {
                self.buf[self.i] = b;
                self.i += 1;
            }

            fn extend(&mut self, b: &[u8]) {
                let len = b.len();
                self.buf[self.i..self.i + len].copy_from_slice(b);
                self.i += len;
            }
        }

        let mut m = Mutator { buf, i: 0 };
        m.extend(&[*cla, *ins, *p1, *p2]);

        let has_payload = &payload.is_some();
        if let Some(p) = payload {
            // According to spec, length can be 0, 1 or 2 bytes
            // 2 bytes is prefaced by 00 to differentiate between 1 byte lengths
            if cfg!(feature = "longer_payloads") && p.len() > u8::MAX as usize {
                m.push(0u8);
                m.push(p.len() as u8);
                m.push((p.len() >> 8) as u8);
            } else {
                m.push(p.len() as u8);
            }

            m.extend(p);
        }

        if let Some(l) = *le {
            if cfg!(feature = "longer_payloads") && l > u8::MAX.into() {
                if !has_payload {
                    m.push(0u8);
                }
                m.push(l as u8);
                m.push((l >> 8) as u8);
            } else {
                m.push(l as u8);
            }
        }
    }

    /// Calculates the length of entire the command.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        let (lc, payload) = match self.payload {
            Some(p) => (
                match cfg!(feature = "longer_payloads") && p.len() > u8::MAX as usize {
                    true => 2,
                    _ => 1,
                },
                p.len(),
            ),
            _ => (0, 0),
        };

        let le = match self.le {
            Some(l) => match cfg!(feature = "longer_payloads") && l > u8::MAX.into() {
                true => match self.payload.is_some() {
                    true => 2,
                    _ => 3,
                },
                _ => 1,
            },
            _ => 0,
        };

        // Header is always 4 bytes
        4 + lc + payload + le
    }
}

#[cfg(feature = "std")]
impl<'a> From<Command<'a>> for Vec<u8> {
    /// Converts the command into octets.
    fn from(command: Command) -> Self {
        let len = command.len();
        let mut buf = Vec::with_capacity(len);

        #[allow(clippy::uninit_vec)]
        unsafe {
            buf.set_len(len);
        }

        command.write(&mut buf);

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_to_vec() {
        assert_eq!(
            vec![0x01, 0x02, 0x03, 0x04, 0x03, 0x05, 0x06, 0x07, 0x08],
            Vec::from(Command::new_with_payload_le(
                0x01,
                0x02,
                0x03,
                0x04,
                0x08,
                &[0x05, 0x06, 0x07]
            )),
        );
    }
}
