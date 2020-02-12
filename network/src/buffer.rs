use byteorder::{BigEndian, ReadBytesExt};
use std::fmt;
use std::io;
use std::io::{Cursor, Write};
use std::ops::RangeBounds;
use std::str;
use uuid::Uuid;

use cgmath::Vector3;
use common::chat::Chat;

#[derive(Default)]
pub struct Buffer {
    inner: Vec<u8>,
    cursor: usize,
}

impl<'a> Extend<&'a u8> for Buffer {
    fn extend<I: IntoIterator<Item = &'a u8>>(&mut self, iter: I) {
        self.inner.extend(iter);
    }
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            inner: vec![],
            cursor: 0,
        }
    }

    pub fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> std::vec::Drain<'_, u8> {
        self.inner.drain(range)
    }

    pub fn reset_cursor(&mut self) {
        self.cursor = 0;
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn has_at_least(&self, count: usize) -> bool {
        self.inner.len() - self.cursor >= count
    }

    pub fn as_raw(&self) -> &[u8] {
        &self.inner
    }

    #[allow(dead_code)]
    pub fn read_bool(&mut self) -> io::Result<bool> {
        Ok(self.read_byte()? == 1)
    }

    #[allow(dead_code)]
    pub fn write_bool(&mut self, value: bool) -> io::Result<()> {
        self.write_byte(if value { 0x1 } else { 0x0 })
    }

    #[allow(dead_code)]
    pub fn read_byte(&mut self) -> io::Result<i8> {
        let mut rdr = Cursor::new(&self.inner[self.cursor..]);
        let value = rdr.read_i8()?;
        self.cursor += 1;
        Ok(value)
    }

    #[allow(dead_code)]
    pub fn write_byte(&mut self, value: i8) -> io::Result<()> {
        self.inner.write_all(&i8::to_be_bytes(value))
    }

    #[allow(dead_code)]
    pub fn read_ubyte(&mut self) -> io::Result<u8> {
        let mut rdr = Cursor::new(&self.inner[self.cursor..]);
        let value = rdr.read_u8()?;
        self.cursor += 1;
        Ok(value)
    }

    #[allow(dead_code)]
    pub fn write_ubyte(&mut self, value: u8) -> io::Result<()> {
        self.inner.write_all(&u8::to_be_bytes(value))
    }

    #[allow(dead_code)]
    pub fn read_short(&mut self) -> io::Result<i16> {
        let mut rdr = Cursor::new(&self.inner[self.cursor..]);
        let value = rdr.read_i16::<BigEndian>()?;
        self.cursor += 2;
        Ok(value)
    }

    #[allow(dead_code)]
    pub fn write_short(&mut self, value: i16) -> io::Result<()> {
        self.inner.write_all(&i16::to_be_bytes(value))
    }

    #[allow(dead_code)]
    pub fn read_ushort(&mut self) -> io::Result<u16> {
        let mut rdr = Cursor::new(&self.inner[self.cursor..]);
        let value = rdr.read_u16::<BigEndian>()?;
        self.cursor += 2;
        Ok(value)
    }

    #[allow(dead_code)]
    pub fn write_ushort(&mut self, value: u16) -> io::Result<()> {
        self.inner.write_all(&u16::to_be_bytes(value))
    }

    #[allow(dead_code)]
    pub fn read_int(&mut self) -> io::Result<i32> {
        let mut rdr = Cursor::new(&self.inner[self.cursor..]);
        let value = rdr.read_i32::<BigEndian>()?;
        self.cursor += 4;
        Ok(value)
    }

    #[allow(dead_code)]
    pub fn write_int(&mut self, value: i32) -> io::Result<()> {
        self.inner.write_all(&i32::to_be_bytes(value))
    }

    #[allow(dead_code)]
    pub fn read_uint(&mut self) -> io::Result<u32> {
        let mut rdr = Cursor::new(&self.inner[self.cursor..]);
        let value = rdr.read_u32::<BigEndian>()?;
        self.cursor += 4;
        Ok(value)
    }

    #[allow(dead_code)]
    pub fn write_uint(&mut self, value: u32) -> io::Result<()> {
        self.inner.write_all(&u32::to_be_bytes(value))
    }

    #[allow(dead_code)]
    pub fn read_long(&mut self) -> io::Result<i64> {
        let mut rdr = Cursor::new(&self.inner[self.cursor..]);
        let value = rdr.read_i64::<BigEndian>()?;
        self.cursor += 8;
        Ok(value)
    }

    #[allow(dead_code)]
    pub fn write_long(&mut self, value: i64) -> io::Result<()> {
        self.inner.write_all(&i64::to_be_bytes(value))?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn read_ulong(&mut self) -> io::Result<u64> {
        let mut rdr = Cursor::new(&self.inner[self.cursor..]);
        let value = rdr.read_u64::<BigEndian>()?;
        self.cursor += 8;
        Ok(value)
    }

    #[allow(dead_code)]
    pub fn write_ulong(&mut self, value: u64) -> io::Result<()> {
        self.inner.write_all(&u64::to_be_bytes(value))
    }

    #[allow(dead_code)]
    pub fn read_float(&mut self) -> io::Result<f32> {
        let mut rdr = Cursor::new(&self.inner[self.cursor..]);
        let value = rdr.read_f32::<BigEndian>()?;
        self.cursor += 4;
        Ok(value)
    }

    #[allow(dead_code)]
    pub fn write_float(&mut self, value: f32) -> io::Result<()> {
        self.inner.write_all(&f32::to_be_bytes(value))
    }

    #[allow(dead_code)]
    pub fn read_double(&mut self) -> io::Result<f64> {
        let mut rdr = Cursor::new(&self.inner[self.cursor..]);
        let value = rdr.read_f64::<BigEndian>()?;
        self.cursor += 8;
        Ok(value)
    }

    #[allow(dead_code)]
    pub fn write_double(&mut self, value: f64) -> io::Result<()> {
        self.inner.write_all(&f64::to_be_bytes(value))
    }

    #[allow(dead_code)]
    pub fn read_string(&mut self) -> io::Result<String> {
        let len = self.read_varint()? as usize;

        if !self.has_at_least(len) {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to read a string of length {} from stream", len),
            ))
        } else {
            let tmp = self.inner[self.cursor..self.cursor + len].to_vec();
            match str::from_utf8(&tmp) {
                Ok(value) => {
                    self.cursor += len;
                    Ok(String::from(value))
                }
                Err(_) => Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to read an UTF-8 string",
                )),
            }
        }
    }

    #[allow(dead_code)]
    pub fn write_string(&mut self, value: &str) -> io::Result<()> {
        self.write_varint(value.len() as i32)?;
        self.inner.write_all(value.as_bytes())?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn read_varint(&mut self) -> io::Result<i32> {
        let mut result = 0;
        let msb: u8 = 0b1000_0000;
        let mask: u8 = !msb;

        for i in 0..5 {
            let read = self.read_ubyte()?;
            result |= ((read & mask) as i32) << (7 * i);

            if i == 4 && (read & 0xf0 != 0) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VarInt is too big",
                ));
            }

            if (read & msb) == 0 {
                return Ok(result);
            }
        }
        Err(io::Error::new(io::ErrorKind::InvalidData, "Loop ended"))
    }

    #[allow(dead_code)]
    pub fn write_varint(&mut self, mut value: i32) -> io::Result<()> {
        let msb: u8 = 0b1000_0000;
        let mask: i32 = 0b0111_1111;

        for _ in 0..5 {
            let tmp = (value & mask) as u8;
            value &= !mask;
            value = value.rotate_right(7);

            if value != 0 {
                self.inner.write_all(&[tmp | msb])?;
            } else {
                self.inner.write_all(&[tmp])?;
                return Ok(());
            }
        }
        Err(io::Error::new(io::ErrorKind::InvalidData, "Loop ended"))
    }

    #[allow(dead_code)]
    pub fn read_varlong(&mut self) -> io::Result<i64> {
        let mut result = 0;

        let msb: u8 = 0b1000_0000;
        let mask: u8 = !msb;

        for i in 0..10 {
            let read = self.read_ubyte()?;
            result |= ((read & mask) as i64) << (7 * i);

            if i == 9 && ((read & (!0x1)) != 0) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "VarLong is too big",
                ));
            }

            if (read & msb) == 0 {
                return Ok(result);
            }
        }
        Err(io::Error::new(io::ErrorKind::InvalidData, "Loop ended"))
    }

    #[allow(dead_code)]
    pub fn write_varlong(&mut self, mut value: i64) -> io::Result<()> {
        let msb: u8 = 0b1000_0000;
        let mask: i64 = 0b0111_1111;

        for _ in 0..10 {
            let tmp = (value & mask) as u8;
            value &= !mask;
            value = value.rotate_right(7);

            if value != 0 {
                self.inner.write_all(&[tmp | msb])?;
            } else {
                self.inner.write_all(&[tmp])?;
                return Ok(());
            }
        }
        Err(io::Error::new(io::ErrorKind::InvalidData, "Loop ended"))
    }

    #[allow(dead_code)]
    pub fn read_position(&mut self) -> io::Result<Vector3<i32>> {
        let value = self.read_ulong()?;
        let mut x = (value >> 38) as i32;
        let mut y = ((value >> 26) & 0xfff) as i32;
        let mut z = (value << 38 >> 38) as i32;

        if x >= 1 << 25 {
            x -= 1 << 26;
        }
        if y >= 1 << 11 {
            y -= 1 << 12;
        }
        if z >= 1 << 25 {
            z -= 1 << 26;
        }
        Ok(Vector3::new(x, y, z))
    }

    #[allow(dead_code)]
    pub fn write_position(&mut self, value: &Vector3<i32>) -> io::Result<()> {
        let x = if value.x >= 0 {
            value.x as u64
        } else {
            (value.x + (1 << 26)) as u64
        };

        let y = if value.y >= 0 {
            value.y as u64
        } else {
            (value.y + (1 << 12)) as u64
        };

        let z = if value.z >= 0 {
            value.z as u64
        } else {
            (value.z + (1 << 26)) as u64
        };

        if x & (!0x03ff_ffff) != 0 {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "X is out of range",
            ))
        } else if y & (!0xfff) != 0 {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Y is out of range",
            ))
        } else if z & (!0x03ff_ffff) != 0 {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Z is out of range",
            ))
        } else {
            let encoded = ((x & 0x03ff_ffff) << 38) | ((y & 0xfff) << 26) | (z & 0x03ff_ffff);
            self.write_ulong(encoded)
        }
    }

    #[allow(dead_code)]
    pub fn read_uuid(&mut self) -> io::Result<Uuid> {
        let mut rdr = Cursor::new(&self.inner[self.cursor..]);
        let value = rdr.read_u128::<BigEndian>()?;
        self.cursor += 16;
        Ok(Uuid::from_u128(value))
    }

    #[allow(dead_code)]
    pub fn write_uuid(&mut self, value: &Uuid) -> io::Result<()> {
        self.inner.write_all(&u128::to_be_bytes(value.as_u128()))
    }

    #[allow(dead_code)]
    pub fn read_array<F, T>(&mut self, f: F, size: usize) -> io::Result<Vec<T>>
    where
        F: Fn(&mut Buffer) -> io::Result<T>,
    {
        let mut vec = vec![];

        for _ in 0..size {
            vec.push(f(self)?);
        }
        Ok(vec)
    }

    #[allow(dead_code)]
    pub fn write_array<'a, F, T>(&mut self, f: F, slice: &'a [T]) -> io::Result<()>
    where
        F: Fn(&mut Buffer, &'a T) -> io::Result<()>,
    {
        for item in slice {
            f(self, item)?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn read_byte_array(&mut self, size: usize) -> io::Result<Vec<i8>> {
        self.read_array(Buffer::read_byte, size)
    }

    #[allow(dead_code)]
    pub fn write_byte_array(&mut self, vec: &Vec<i8>) -> io::Result<()> {
        // Create a Vec<u8> out of the backing of Vec<i8>
        let temp_vec = unsafe {
            // Safety: This is safe as long than temp_vec isn't used in a mutable way.
            Vec::from_raw_parts(vec.as_ptr() as *mut u8, vec.len(), vec.capacity())
        };

        let result = self.write_ubyte_array(&temp_vec);

        // Ensure that we leak the temporary object as we don't want it to detroy the real object
        std::mem::forget(temp_vec);

        result
    }

    #[allow(dead_code)]
    pub fn read_ubyte_array(&mut self, size: usize) -> io::Result<Vec<u8>> {
        self.read_array(Buffer::read_ubyte, size)
    }

    #[allow(dead_code)]
    pub fn write_ubyte_array(&mut self, slice: &[u8]) -> io::Result<()> {
        self.inner.write_all(slice)
    }

    #[allow(dead_code)]
    pub fn read_chat(&mut self) -> io::Result<Chat> {
        match Chat::from_string(&self.read_string()?) {
            Ok(chat) => Ok(chat),
            Err(err) => Err(io::Error::new(io::ErrorKind::InvalidData, err)),
        }
    }

    #[allow(dead_code)]
    pub fn write_chat(&mut self, chat: &Chat) -> io::Result<()> {
        match chat.to_string() {
            Ok(value) => self.write_string(&value),
            Err(err) => Err(io::Error::new(io::ErrorKind::InvalidData, err)),
        }
    }
}
