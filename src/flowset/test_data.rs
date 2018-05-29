pub const TEMPLATE_DATA: [u8; 92] = [
    0x00,
    0x00,
    0x00,
    0x5c,
    0x04,
    0x00,
    0x00,
    0x15,
    0x00,
    0x15,
    0x00,
    0x04,
    0x00,
    0x16,
    0x00,
    0x04,
    0x00,
    0x01,
    0x00,
    0x04,
    0x00,
    0x02,
    0x00,
    0x04,
    0x00,
    0x3c,
    0x00,
    0x01,
    0x00,
    0x0a,
    0x00,
    0x02,
    0x00,
    0x0e,
    0x00,
    0x02,
    0x00,
    0x3d,
    0x00,
    0x01,
    0x00,
    0x03,
    0x00,
    0x04,
    0x00,
    0x08,
    0x00,
    0x04,
    0x00,
    0x0c,
    0x00,
    0x04,
    0x00,
    0x07,
    0x00,
    0x02,
    0x00,
    0x0b,
    0x00,
    0x02,
    0x00,
    0x05,
    0x00,
    0x01,
    0x00,
    0x06,
    0x00,
    0x01,
    0x00,
    0x04,
    0x00,
    0x01,
    0x00,
    0x38,
    0x00,
    0x06,
    0x00,
    0x50,
    0x00,
    0x06,
    0x00,
    0x3a,
    0x00,
    0x02,
    0x00,
    0xc9,
    0x00,
    0x04,
    0x00,
    0x30,
    0x00,
    0x01,
];

pub const OPTION_DATA: [u8; 26] = [
    0x00,
    0x01,
    0x00,
    0x1a,
    0x10,
    0x00,
    0x00,
    0x04,
    0x00,
    0x0c,
    0x00,
    0x01,
    0x00,
    0x04,
    0x00,
    0x30,
    0x00,
    0x01,
    0x00,
    0x31,
    0x00,
    0x01,
    0x00,
    0x32,
    0x00,
    0x04,
];

pub const DATAFLOW_DATA: [u8; 14] = [
    0x10,
    0x00,
    0x00,
    0x0e,
    0x00,
    0x00,
    0x00,
    0x00,
    0x01,
    0x02,
    0x00,
    0x00,
    0x00,
    0x01,
];

pub const TEMPLATE_AND_DATA: ([u8; 92], [u8; 484]) = (
    [
        0x00,
        0x00,
        0x00,
        0x5c,
        0x04,
        0x00,
        0x00,
        0x15,
        0x00,
        0x15,
        0x00,
        0x04,
        0x00,
        0x16,
        0x00,
        0x04,
        0x00,
        0x01,
        0x00,
        0x04,
        0x00,
        0x02,
        0x00,
        0x04,
        0x00,
        0x3c,
        0x00,
        0x01,
        0x00,
        0x0a,
        0x00,
        0x02,
        0x00,
        0x0e,
        0x00,
        0x02,
        0x00,
        0x3d,
        0x00,
        0x01,
        0x00,
        0x03,
        0x00,
        0x04,
        0x00,
        0x08,
        0x00,
        0x04,
        0x00,
        0x0c,
        0x00,
        0x04,
        0x00,
        0x07,
        0x00,
        0x02,
        0x00,
        0x0b,
        0x00,
        0x02,
        0x00,
        0x05,
        0x00,
        0x01,
        0x00,
        0x06,
        0x00,
        0x01,
        0x00,
        0x04,
        0x00,
        0x01,
        0x00,
        0x38,
        0x00,
        0x06,
        0x00,
        0x50,
        0x00,
        0x06,
        0x00,
        0x3a,
        0x00,
        0x02,
        0x00,
        0xc9,
        0x00,
        0x04,
        0x00,
        0x30,
        0x00,
        0x01,
    ],
    [
        0x04,
        0x00,
        0x01,
        0xe4,
        0x00,
        0x52,
        0x1b,
        0x3d,
        0x00,
        0x52,
        0x1b,
        0x3d,
        0x00,
        0x00,
        0x00,
        0x28,
        0x00,
        0x00,
        0x00,
        0x01,
        0x04,
        0x00,
        0x03,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0xc0,
        0xa8,
        0x06,
        0x65,
        0xc0,
        0xa8,
        0x06,
        0x66,
        0x4d,
        0x82,
        0x84,
        0x60,
        0x00,
        0x14,
        0x06,
        0x08,
        0x00,
        0x27,
        0x3a,
        0xdd,
        0x56,
        0x90,
        0x1b,
        0x0e,
        0x95,
        0x74,
        0x93,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x03,
        0x01,
        0x00,
        0x52,
        0x1b,
        0x3d,
        0x00,
        0x52,
        0x1b,
        0x3d,
        0x00,
        0x00,
        0x00,
        0x28,
        0x00,
        0x00,
        0x00,
        0x01,
        0x04,
        0x00,
        0x03,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0xc0,
        0xa8,
        0x06,
        0x65,
        0xc0,
        0xa8,
        0x06,
        0x66,
        0x09,
        0xdd,
        0xca,
        0x38,
        0x00,
        0x14,
        0x06,
        0x08,
        0x00,
        0x27,
        0x3a,
        0xdd,
        0x56,
        0x90,
        0x1b,
        0x0e,
        0x95,
        0x74,
        0x93,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x03,
        0x01,
        0x00,
        0x52,
        0x1b,
        0x3d,
        0x00,
        0x52,
        0x1b,
        0x3d,
        0x00,
        0x00,
        0x00,
        0x28,
        0x00,
        0x00,
        0x00,
        0x01,
        0x04,
        0x00,
        0x03,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0xc0,
        0xa8,
        0x06,
        0x65,
        0xc0,
        0xa8,
        0x06,
        0x66,
        0x04,
        0x5f,
        0x80,
        0x8a,
        0x00,
        0x14,
        0x06,
        0x08,
        0x00,
        0x27,
        0x3a,
        0xdd,
        0x56,
        0x90,
        0x1b,
        0x0e,
        0x95,
        0x74,
        0x93,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x03,
        0x01,
        0x00,
        0x52,
        0x1b,
        0x3d,
        0x00,
        0x52,
        0x1b,
        0x3d,
        0x00,
        0x00,
        0x00,
        0x28,
        0x00,
        0x00,
        0x00,
        0x01,
        0x04,
        0x00,
        0x03,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0xc0,
        0xa8,
        0x06,
        0x65,
        0xc0,
        0xa8,
        0x06,
        0x66,
        0x3e,
        0xf1,
        0x9f,
        0x1c,
        0x00,
        0x14,
        0x06,
        0x08,
        0x00,
        0x27,
        0x3a,
        0xdd,
        0x56,
        0x90,
        0x1b,
        0x0e,
        0x95,
        0x74,
        0x93,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x03,
        0x01,
        0x00,
        0x52,
        0x1b,
        0x3e,
        0x00,
        0x52,
        0x1b,
        0x3e,
        0x00,
        0x00,
        0x00,
        0x28,
        0x00,
        0x00,
        0x00,
        0x01,
        0x04,
        0x00,
        0x03,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0xc0,
        0xa8,
        0x06,
        0x65,
        0xc0,
        0xa8,
        0x06,
        0x66,
        0x20,
        0xfb,
        0xe7,
        0xe2,
        0x00,
        0x14,
        0x06,
        0x08,
        0x00,
        0x27,
        0x3a,
        0xdd,
        0x56,
        0x90,
        0x1b,
        0x0e,
        0x95,
        0x74,
        0x93,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x03,
        0x01,
        0x00,
        0x52,
        0x1b,
        0x3e,
        0x00,
        0x52,
        0x1b,
        0x3e,
        0x00,
        0x00,
        0x00,
        0x28,
        0x00,
        0x00,
        0x00,
        0x01,
        0x04,
        0x00,
        0x03,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0xc0,
        0xa8,
        0x06,
        0x65,
        0xc0,
        0xa8,
        0x06,
        0x66,
        0x0a,
        0x96,
        0xb8,
        0xa6,
        0x00,
        0x14,
        0x06,
        0x08,
        0x00,
        0x27,
        0x3a,
        0xdd,
        0x56,
        0x90,
        0x1b,
        0x0e,
        0x95,
        0x74,
        0x93,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x03,
        0x01,
        0x00,
        0x52,
        0x1b,
        0x3e,
        0x00,
        0x52,
        0x1b,
        0x3e,
        0x00,
        0x00,
        0x00,
        0x28,
        0x00,
        0x00,
        0x00,
        0x01,
        0x04,
        0x00,
        0x03,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0xc0,
        0xa8,
        0x06,
        0x65,
        0xc0,
        0xa8,
        0x06,
        0x66,
        0x27,
        0xe7,
        0x90,
        0xb2,
        0x00,
        0x14,
        0x06,
        0x08,
        0x00,
        0x27,
        0x3a,
        0xdd,
        0x56,
        0x90,
        0x1b,
        0x0e,
        0x95,
        0x74,
        0x93,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x03,
        0x01,
        0x00,
        0x52,
        0x1b,
        0x3e,
        0x00,
        0x52,
        0x1b,
        0x3e,
        0x00,
        0x00,
        0x00,
        0x28,
        0x00,
        0x00,
        0x00,
        0x01,
        0x04,
        0x00,
        0x03,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0xc0,
        0xa8,
        0x06,
        0x65,
        0xc0,
        0xa8,
        0x06,
        0x66,
        0x09,
        0xbc,
        0x97,
        0x80,
        0x00,
        0x14,
        0x06,
        0x08,
        0x00,
        0x27,
        0x3a,
        0xdd,
        0x56,
        0x90,
        0x1b,
        0x0e,
        0x95,
        0x74,
        0x93,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x03,
        0x01,
    ],
);