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
    Primitive(PrimitiveType),
    Array(ArrayPropertyRecord),
}

#[derive(Debug)]
pub enum PrimitiveType {
    TwoByteSignedInteger(TwoByteSignedInteger),
    OneBitBoolean(OneBitBoolean),
    FourByteSignedInteger(FourByteSignedInteger),
    None,
}

#[derive(Debug)]
pub struct FourByteSignedInteger {
    pub data: i32,
}

#[derive(Debug)]
pub struct OneBitBoolean {
    pub data: bool,
}

#[derive(Debug)]
pub struct TwoByteSignedInteger {
    pub data: i16,
}

#[derive(Debug)]
pub struct ArrayPropertyRecord {
    type_code: ArrayTypeCode,
}

#[derive(Debug)]
pub enum ArrayTypeCode {
    FourByteSinglePrecisionIEEE754NumberArray,
    EightByteDoublePrecisionIEEE754NumberArray,
    EightByteSignedIntegerArray,
    FourByteSignedIntegerArrray,
    OneByteBooleanArray,
}
