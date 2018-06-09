use error::ParseResult;
use nom::be_u16;
use util::u16_to_bytes;

named!(netflowfield <&[u8], TypeLengthField>,
       dbg!(map!(count!(map!(take!(2), |i| be_u16(i).unwrap().1), 2),
                 |v: Vec<_>| TypeLengthField::new(v[0], v[1]))));

#[derive(Debug, Clone, Copy)]
pub struct TypeLengthField {
    pub type_id: u16,
    pub length: u16,
}

impl TypeLengthField {
    pub fn new(type_id: u16, length: u16) -> TypeLengthField {
        TypeLengthField {
            type_id: type_id,
            length: length,
        }
    }

    pub fn to_vec(count: usize, data: &[u8]) -> ParseResult<Vec<TypeLengthField>> {
        let mut rest = data;
        let mut field_vec = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let (next, field) = netflowfield(&rest).unwrap();
            field_vec.push(field);
            rest = next;
        }

        Ok((rest, field_vec))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut u16_buf = [0u8; 2];

        u16_to_bytes(self.type_id, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        u16_to_bytes(self.length, &mut u16_buf);
        bytes.append(&mut u16_buf.to_vec());

        bytes
    }
}

#[cfg(test)]
mod test_tlf {
    use field::test_data;
    use field::TypeLengthField;

    #[test]
    fn test_to_vec() {
        let (len, testdata) = test_data::TYPE_LENGTH_FIELD;
        let (_rest, fields) = TypeLengthField::to_vec(len, &testdata).unwrap();
        assert_eq!(fields.len(), len);
        assert_eq!(fields[0].type_id, 21);
        assert_eq!(fields[0].length, 4);
    }

    #[test]
    fn test_to_bytes() {
        let (len, testdata) = test_data::TYPE_LENGTH_FIELD;
        let (_rest, fields) = TypeLengthField::to_vec(len, &testdata).unwrap();
        let bytes = fields[0].to_bytes();
        assert_eq!(bytes, [0x00, 0x15, 0x00, 0x04]);
    }
}
