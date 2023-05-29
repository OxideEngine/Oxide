pub struct Node {
    end_offset: u32,
    num_properties: u32,
    property_list_len: u32,
    name_len: u8,
    name: Vec<char>,
    properties: Vec<PropertyRecord>,
    nested_list: NestedList,
}

enum PropertyRecord {
    Primitive(PrimitivePropertyRecord),
    Array(ArrayPropertyRecord),
}

pub struct PrimitivePropertyRecord {
    type_code: PrimitiveTypeCode,
}
pub struct ArrayPropertyRecord {
    type_code: ArrayTypeCode,
}

pub struct NestedList {}

enum PrimitiveTypeCode {
    TwoByteSignedInteger,
    OneBitBoolean,
    FourByteSignedInteger,
    FourByteSingePrecisionIEEE754Number,
    EightByteDoublePrecisionIEEE754Number,
    EightByteSignedInteger,
}

enum ArrayTypeCode {
    FourByteSinglePrecisionIEEE754NumberArray,
    EightByteDoublePrecisionIEEE754NumberArray,
    EightByteSignedIntegerArray,
    FourByteSignedIntegerArrray,
    OneByteBooleanArray,
}
