#[derive(Debug)]
pub struct Node {
    pub end_offset: usize,
    pub num_properties: u32,
    pub property_list_len: u32,
    pub name_len: u8,
    pub name: String,
    pub properties: Vec<PropertyRecord>,
    pub nested_list: Vec<Node>,
}

#[derive(Debug)]
pub enum PropertyRecord {
    Primitive(PrimitivePropertyRecord),
    Array(ArrayPropertyRecord),
}

#[derive(Debug)]
pub struct PrimitivePropertyRecord {
    type_code: PrimitiveTypeCode,
}
#[derive(Debug)]
pub struct ArrayPropertyRecord {
    type_code: ArrayTypeCode,
}

#[derive(Debug)]
enum PrimitiveTypeCode {
    TwoByteSignedInteger,
    OneBitBoolean,
    FourByteSignedInteger,
    FourByteSingePrecisionIEEE754Number,
    EightByteDoublePrecisionIEEE754Number,
    EightByteSignedInteger,
}

#[derive(Debug)]
enum ArrayTypeCode {
    FourByteSinglePrecisionIEEE754NumberArray,
    EightByteDoublePrecisionIEEE754NumberArray,
    EightByteSignedIntegerArray,
    FourByteSignedIntegerArrray,
    OneByteBooleanArray,
}
