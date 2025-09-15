pub struct BufferReader {
    offset: usize,
    buffer: Vec<u8>,
}

impl BufferReader {
    pub fn new(buffer: Vec<u8>) -> Self {
        BufferReader { offset: 0, buffer }
    }

    pub fn read_u32le(&mut self) -> Result<u32, std::io::Error> {
        self.validate_length(4)?;
        let value = u32::from_le_bytes(
            self.buffer[self.offset..self.offset + 4]
                .try_into()
                .unwrap(),
        );
        self.offset += 4;
        Ok(value)
    }

    pub fn read_u32be(&mut self) -> Result<u32, std::io::Error> {
        self.validate_length(4)?;
        let value = u32::from_be_bytes(
            self.buffer[self.offset..self.offset + 4]
                .try_into()
                .unwrap(),
        );
        self.offset += 4;
        Ok(value)
    }

    pub fn read_u16le(&mut self) -> Result<u16, std::io::Error> {
        self.validate_length(2)?;
        let value = u16::from_le_bytes(
            self.buffer[self.offset..self.offset + 2]
                .try_into()
                .unwrap(),
        );
        self.offset += 2;
        Ok(value)
    }

    pub fn read_u16be(&mut self) -> Result<u16, std::io::Error> {
        self.validate_length(2)?;
        let value = u16::from_be_bytes(
            self.buffer[self.offset..self.offset + 2]
                .try_into()
                .unwrap(),
        );
        self.offset += 2;
        Ok(value)
    }

    pub fn read_i32le(&mut self) -> Result<i32, std::io::Error> {
        self.validate_length(4)?;

        let value = i32::from_le_bytes(
            self.buffer[self.offset..self.offset + 4]
                .try_into()
                .unwrap(),
        );

        self.offset += 4;

        Ok(value)
    }

    pub fn read_i32be(&mut self) -> Result<i32, std::io::Error> {
        self.validate_length(4)?;

        let value = i32::from_be_bytes(
            self.buffer[self.offset..self.offset + 4]
                .try_into()
                .unwrap(),
        );

        self.offset += 4;

        Ok(value)
    }

    pub fn read_i16le(&mut self) -> Result<i16, std::io::Error> {
        self.validate_length(2)?;

        let value = i16::from_le_bytes(
            self.buffer[self.offset..self.offset + 2]
                .try_into()
                .unwrap(),
        );

        self.offset += 2;

        Ok(value)
    }

    pub fn read_i16be(&mut self) -> Result<i16, std::io::Error> {
        self.validate_length(2)?;

        let value = i16::from_be_bytes(
            self.buffer[self.offset..self.offset + 2]
                .try_into()
                .unwrap(),
        );

        self.offset += 2;

        Ok(value)
    }

    fn validate_length(&self, length: usize) -> Result<(), std::io::Error> {
        if self.buffer.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Buffer is empty",
            ));
        }

        if self.offset + length > self.buffer.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Not enough data to read",
            ));
        }
        Ok(())
    }
}
