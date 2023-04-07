use stalker_utils::tag::Tag;

pub use crate::traits::*;

pub struct Stuck<const MASK: u8>;

impl<const MASK: u8> FaultModel for Stuck<MASK> {
    fn next_mutant(iter: &mut IntoIter<Self>) -> Option<Vec<u8>> {
        let mut bytes = iter.base.clone();
        while let Some(&byte) = bytes.get(iter.bit.0) {
            if byte == MASK {
                iter.bit.0 += 1;
            } else {
                bytes[iter.bit.0] = MASK;
                iter.bit.0 += 1;
                return Some(bytes);
            }
        }
        None
    }
}

impl<const MASK: u8> Tag for Stuck<MASK> {
    fn tag() -> String {
        format!("stuck-{:#04x?}", MASK)
    }

    fn id(&self) -> String {
        Self::tag()
    }
}

#[cfg(test)]
mod tests {
    use super::Tag;
    use super::{RawMutatable, Stuck};
    use anyhow::Result;

    #[test]
    fn test_fault_name() {
        assert_eq!("stuck-0x00", Stuck::<0x00>::tag());
        assert_eq!("stuck-0x05", Stuck::<0x05>::tag());
        assert_eq!("stuck-0xff", Stuck::<0xff>::tag());
    }

    #[test]
    fn test_stuck_raw_mutants() -> Result<()> {
        const MASK: u8 = 0b11111111;
        let bytes_4 = vec![0u8, 1, 2u8, 3u8];
        let bytes_2 = vec![0xff, 0xff, 2u8, 3u8];
        let bytes_0 = vec![0xff; 4];
        assert_eq!(
            4,
            bytes_4
                .raw_mutants::<Stuck<MASK>>()
                .collect::<Vec<Vec<u8>>>()
                .len()
        );
        assert_eq!(
            2,
            bytes_2
                .raw_mutants::<Stuck<MASK>>()
                .collect::<Vec<Vec<u8>>>()
                .len()
        );
        assert_eq!(
            0,
            bytes_0
                .raw_mutants::<Stuck<MASK>>()
                .collect::<Vec<Vec<u8>>>()
                .len()
        );
        Ok(())
    }
}
