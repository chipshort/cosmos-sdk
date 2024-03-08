use std::io::Result;
fn main() -> Result<()> {
    // test1/test.proto
    zeropb_build::compile_fd_bytes(&[
        0x0a, 0x10, 0x74, 0x65, 0x73, 0x74, 0x31, 0x2f, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x70, 0x72, 0x6f,
        0x74, 0x6f, 0x12, 0x05, 0x74, 0x65, 0x73, 0x74, 0x31, 0x22, 0x31, 0x0a, 0x05, 0x47, 0x72, 0x65,
        0x65, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
        0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18,
        0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x29, 0x0a, 0x0d,
        0x47, 0x72, 0x65, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x18, 0x0a,
        0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07,
        0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x42, 0x29, 0x5a, 0x27, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
        0x73, 0x73, 0x64, 0x6b, 0x2e, 0x69, 0x6f, 0x2f, 0x6c, 0x6f, 0x61, 0x64, 0x65, 0x72, 0x2f, 0x77,
        0x61, 0x73, 0x6d, 0x2f, 0x74, 0x65, 0x73, 0x74, 0x64, 0x61, 0x74, 0x61, 0x2f, 0x74, 0x65, 0x73,
        0x74, 0x31, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
    ])?;

    // cosmos/base/v1beta1/coin.proto
    zeropb_build::compile_fd_bytes(&[
        0x0a, 0x1e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2f, 0x76, 0x31,
        0x62, 0x65, 0x74, 0x61, 0x31, 0x2f, 0x63, 0x6f, 0x69, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
        0x12, 0x13, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x76, 0x31,
        0x62, 0x65, 0x74, 0x61, 0x31, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
        0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x19, 0x63, 0x6f, 0x73,
        0x6d, 0x6f, 0x73, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
        0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x11, 0x61, 0x6d, 0x69, 0x6e, 0x6f, 0x2f, 0x61, 0x6d,
        0x69, 0x6e, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x6c, 0x0a, 0x04, 0x43, 0x6f, 0x69,
        0x6e, 0x12, 0x14, 0x0a, 0x05, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
        0x52, 0x05, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x12, 0x48, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e,
        0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x42, 0x30, 0xc8, 0xde, 0x1f, 0x00, 0xda, 0xde, 0x1f,
        0x15, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x73, 0x64, 0x6b, 0x2e, 0x69, 0x6f, 0x2f, 0x6d, 0x61,
        0x74, 0x68, 0x2e, 0x49, 0x6e, 0x74, 0xd2, 0xb4, 0x2d, 0x0a, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
        0x2e, 0x49, 0x6e, 0x74, 0xa8, 0xe7, 0xb0, 0x2a, 0x01, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e,
        0x74, 0x3a, 0x04, 0xe8, 0xa0, 0x1f, 0x01, 0x22, 0x70, 0x0a, 0x07, 0x44, 0x65, 0x63, 0x43, 0x6f,
        0x69, 0x6e, 0x12, 0x14, 0x0a, 0x05, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28,
        0x09, 0x52, 0x05, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x12, 0x49, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75,
        0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x42, 0x31, 0xc8, 0xde, 0x1f, 0x00, 0xda, 0xde,
        0x1f, 0x1b, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x73, 0x64, 0x6b, 0x2e, 0x69, 0x6f, 0x2f, 0x6d,
        0x61, 0x74, 0x68, 0x2e, 0x4c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x44, 0x65, 0x63, 0xd2, 0xb4, 0x2d,
        0x0a, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x44, 0x65, 0x63, 0x52, 0x06, 0x61, 0x6d, 0x6f,
        0x75, 0x6e, 0x74, 0x3a, 0x04, 0xe8, 0xa0, 0x1f, 0x01, 0x42, 0xcc, 0x01, 0x0a, 0x17, 0x63, 0x6f,
        0x6d, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x76, 0x31,
        0x62, 0x65, 0x74, 0x61, 0x31, 0x42, 0x09, 0x43, 0x6f, 0x69, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f,
        0x50, 0x01, 0x5a, 0x30, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x73, 0x64, 0x6b, 0x2e, 0x69, 0x6f,
        0x2f, 0x61, 0x70, 0x69, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x62, 0x61, 0x73, 0x65,
        0x2f, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x3b, 0x62, 0x61, 0x73, 0x65, 0x76, 0x31, 0x62,
        0x65, 0x74, 0x61, 0x31, 0xa2, 0x02, 0x03, 0x43, 0x42, 0x58, 0xaa, 0x02, 0x13, 0x43, 0x6f, 0x73,
        0x6d, 0x6f, 0x73, 0x2e, 0x42, 0x61, 0x73, 0x65, 0x2e, 0x56, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31,
        0xca, 0x02, 0x13, 0x43, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x5c, 0x42, 0x61, 0x73, 0x65, 0x5c, 0x56,
        0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0xe2, 0x02, 0x1f, 0x43, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x5c,
        0x42, 0x61, 0x73, 0x65, 0x5c, 0x56, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x5c, 0x47, 0x50, 0x42,
        0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x15, 0x43, 0x6f, 0x73, 0x6d, 0x6f,
        0x73, 0x3a, 0x3a, 0x42, 0x61, 0x73, 0x65, 0x3a, 0x3a, 0x56, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31,
        0xd8, 0xe1, 0x1e, 0x00, 0x80, 0xe2, 0x1e, 0x00, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
    ])?;

    // cosmos/bank/v1beta1/bank.proto
    zeropb_build::compile_fd_bytes(&[
        0x0a, 0x1e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x62, 0x61, 0x6e, 0x6b, 0x2f, 0x76, 0x31,
        0x62, 0x65, 0x74, 0x61, 0x31, 0x2f, 0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
        0x12, 0x13, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x76, 0x31,
        0x62, 0x65, 0x74, 0x61, 0x31, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
        0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x19, 0x63, 0x6f, 0x73,
        0x6d, 0x6f, 0x73, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
        0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x62,
        0x61, 0x73, 0x65, 0x2f, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2f, 0x63, 0x6f, 0x69, 0x6e,
        0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x6d,
        0x73, 0x67, 0x2f, 0x76, 0x31, 0x2f, 0x6d, 0x73, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
        0x11, 0x61, 0x6d, 0x69, 0x6e, 0x6f, 0x2f, 0x61, 0x6d, 0x69, 0x6e, 0x6f, 0x2e, 0x70, 0x72, 0x6f,
        0x74, 0x6f, 0x22, 0xa2, 0x01, 0x0a, 0x06, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x12, 0x47, 0x0a,
        0x0c, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x01, 0x20,
        0x03, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e,
        0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x53, 0x65, 0x6e, 0x64, 0x45, 0x6e,
        0x61, 0x62, 0x6c, 0x65, 0x64, 0x42, 0x02, 0x18, 0x01, 0x52, 0x0b, 0x73, 0x65, 0x6e, 0x64, 0x45,
        0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x12, 0x30, 0x0a, 0x14, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c,
        0x74, 0x5f, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x02,
        0x20, 0x01, 0x28, 0x08, 0x52, 0x12, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x53, 0x65, 0x6e,
        0x64, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x3a, 0x1d, 0x8a, 0xe7, 0xb0, 0x2a, 0x18, 0x63,
        0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x78, 0x2f, 0x62, 0x61, 0x6e, 0x6b,
        0x2f, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x22, 0x43, 0x0a, 0x0b, 0x53, 0x65, 0x6e, 0x64, 0x45,
        0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x18,
        0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x12, 0x18, 0x0a, 0x07,
        0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x65,
        0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x3a, 0x04, 0xe8, 0xa0, 0x1f, 0x01, 0x22, 0xca, 0x01, 0x0a,
        0x05, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x12, 0x32, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73,
        0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x18, 0xd2, 0xb4, 0x2d, 0x14, 0x63, 0x6f, 0x73,
        0x6d, 0x6f, 0x73, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x53, 0x74, 0x72, 0x69, 0x6e,
        0x67, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x77, 0x0a, 0x05, 0x63, 0x6f,
        0x69, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x63, 0x6f, 0x73, 0x6d,
        0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e,
        0x43, 0x6f, 0x69, 0x6e, 0x42, 0x46, 0xc8, 0xde, 0x1f, 0x00, 0xaa, 0xdf, 0x1f, 0x28, 0x67, 0x69,
        0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f,
        0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73,
        0x2e, 0x43, 0x6f, 0x69, 0x6e, 0x73, 0x9a, 0xe7, 0xb0, 0x2a, 0x0c, 0x6c, 0x65, 0x67, 0x61, 0x63,
        0x79, 0x5f, 0x63, 0x6f, 0x69, 0x6e, 0x73, 0xa8, 0xe7, 0xb0, 0x2a, 0x01, 0x52, 0x05, 0x63, 0x6f,
        0x69, 0x6e, 0x73, 0x3a, 0x14, 0x88, 0xa0, 0x1f, 0x00, 0xe8, 0xa0, 0x1f, 0x00, 0x82, 0xe7, 0xb0,
        0x2a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x22, 0xbf, 0x01, 0x0a, 0x06, 0x4f, 0x75,
        0x74, 0x70, 0x75, 0x74, 0x12, 0x32, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18,
        0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x18, 0xd2, 0xb4, 0x2d, 0x14, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
        0x73, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x52,
        0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x77, 0x0a, 0x05, 0x63, 0x6f, 0x69, 0x6e,
        0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
        0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x43, 0x6f,
        0x69, 0x6e, 0x42, 0x46, 0xc8, 0xde, 0x1f, 0x00, 0xaa, 0xdf, 0x1f, 0x28, 0x67, 0x69, 0x74, 0x68,
        0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f,
        0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x43,
        0x6f, 0x69, 0x6e, 0x73, 0x9a, 0xe7, 0xb0, 0x2a, 0x0c, 0x6c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x5f,
        0x63, 0x6f, 0x69, 0x6e, 0x73, 0xa8, 0xe7, 0xb0, 0x2a, 0x01, 0x52, 0x05, 0x63, 0x6f, 0x69, 0x6e,
        0x73, 0x3a, 0x08, 0x88, 0xa0, 0x1f, 0x00, 0xe8, 0xa0, 0x1f, 0x00, 0x22, 0xac, 0x01, 0x0a, 0x06,
        0x53, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x12, 0x77, 0x0a, 0x05, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x18,
        0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62,
        0x61, 0x73, 0x65, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x43, 0x6f, 0x69, 0x6e,
        0x42, 0x46, 0xc8, 0xde, 0x1f, 0x00, 0xaa, 0xdf, 0x1f, 0x28, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62,
        0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x73, 0x6d,
        0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x43, 0x6f, 0x69,
        0x6e, 0x73, 0x9a, 0xe7, 0xb0, 0x2a, 0x0c, 0x6c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x5f, 0x63, 0x6f,
        0x69, 0x6e, 0x73, 0xa8, 0xe7, 0xb0, 0x2a, 0x01, 0x52, 0x05, 0x74, 0x6f, 0x74, 0x61, 0x6c, 0x3a,
        0x29, 0x18, 0x01, 0x88, 0xa0, 0x1f, 0x00, 0xe8, 0xa0, 0x1f, 0x01, 0xca, 0xb4, 0x2d, 0x1b, 0x63,
        0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74,
        0x61, 0x31, 0x2e, 0x53, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x49, 0x22, 0x57, 0x0a, 0x09, 0x44, 0x65,
        0x6e, 0x6f, 0x6d, 0x55, 0x6e, 0x69, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x64, 0x65, 0x6e, 0x6f, 0x6d,
        0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x12, 0x1a, 0x0a,
        0x08, 0x65, 0x78, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52,
        0x08, 0x65, 0x78, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x6c, 0x69,
        0x61, 0x73, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x09, 0x52, 0x07, 0x61, 0x6c, 0x69, 0x61,
        0x73, 0x65, 0x73, 0x22, 0x8a, 0x02, 0x0a, 0x08, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61,
        0x12, 0x20, 0x0a, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x18,
        0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69,
        0x6f, 0x6e, 0x12, 0x3f, 0x0a, 0x0b, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x5f, 0x75, 0x6e, 0x69, 0x74,
        0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
        0x2e, 0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x44, 0x65,
        0x6e, 0x6f, 0x6d, 0x55, 0x6e, 0x69, 0x74, 0x52, 0x0a, 0x64, 0x65, 0x6e, 0x6f, 0x6d, 0x55, 0x6e,
        0x69, 0x74, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x62, 0x61, 0x73, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
        0x09, 0x52, 0x04, 0x62, 0x61, 0x73, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x64, 0x69, 0x73, 0x70, 0x6c,
        0x61, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x64, 0x69, 0x73, 0x70, 0x6c, 0x61,
        0x79, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52,
        0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x79, 0x6d, 0x62, 0x6f, 0x6c, 0x18,
        0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x79, 0x6d, 0x62, 0x6f, 0x6c, 0x12, 0x19, 0x0a,
        0x03, 0x75, 0x72, 0x69, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x42, 0x07, 0xe2, 0xde, 0x1f, 0x03,
        0x55, 0x52, 0x49, 0x52, 0x03, 0x75, 0x72, 0x69, 0x12, 0x26, 0x0a, 0x08, 0x75, 0x72, 0x69, 0x5f,
        0x68, 0x61, 0x73, 0x68, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x42, 0x0b, 0xe2, 0xde, 0x1f, 0x07,
        0x55, 0x52, 0x49, 0x48, 0x61, 0x73, 0x68, 0x52, 0x07, 0x75, 0x72, 0x69, 0x48, 0x61, 0x73, 0x68,
        0x42, 0xc4, 0x01, 0x0a, 0x17, 0x63, 0x6f, 0x6d, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e,
        0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x42, 0x09, 0x42, 0x61,
        0x6e, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x30, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
        0x73, 0x73, 0x64, 0x6b, 0x2e, 0x69, 0x6f, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x63, 0x6f, 0x73, 0x6d,
        0x6f, 0x73, 0x2f, 0x62, 0x61, 0x6e, 0x6b, 0x2f, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x3b,
        0x62, 0x61, 0x6e, 0x6b, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0xa2, 0x02, 0x03, 0x43, 0x42,
        0x58, 0xaa, 0x02, 0x13, 0x43, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x42, 0x61, 0x6e, 0x6b, 0x2e,
        0x56, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0xca, 0x02, 0x13, 0x43, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
        0x5c, 0x42, 0x61, 0x6e, 0x6b, 0x5c, 0x56, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0xe2, 0x02, 0x1f,
        0x43, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x5c, 0x42, 0x61, 0x6e, 0x6b, 0x5c, 0x56, 0x31, 0x62, 0x65,
        0x74, 0x61, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea,
        0x02, 0x15, 0x43, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x3a, 0x3a, 0x42, 0x61, 0x6e, 0x6b, 0x3a, 0x3a,
        0x56, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
    ])?;

    // cosmos/bank/v1beta1/tx.proto
    zeropb_build::compile_fd_bytes(&[
        0x0a, 0x1c, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x62, 0x61, 0x6e, 0x6b, 0x2f, 0x76, 0x31,
        0x62, 0x65, 0x74, 0x61, 0x31, 0x2f, 0x74, 0x78, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13,
        0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65,
        0x74, 0x61, 0x31, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67,
        0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
        0x73, 0x2f, 0x62, 0x61, 0x73, 0x65, 0x2f, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2f, 0x63,
        0x6f, 0x69, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
        0x73, 0x2f, 0x62, 0x61, 0x6e, 0x6b, 0x2f, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2f, 0x62,
        0x61, 0x6e, 0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x19, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
        0x73, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x70,
        0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x6d, 0x73, 0x67,
        0x2f, 0x76, 0x31, 0x2f, 0x6d, 0x73, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x11, 0x61,
        0x6d, 0x69, 0x6e, 0x6f, 0x2f, 0x61, 0x6d, 0x69, 0x6e, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
        0x22, 0xac, 0x02, 0x0a, 0x07, 0x4d, 0x73, 0x67, 0x53, 0x65, 0x6e, 0x64, 0x12, 0x3b, 0x0a, 0x0c,
        0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01,
        0x28, 0x09, 0x42, 0x18, 0xd2, 0xb4, 0x2d, 0x14, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x41,
        0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x0b, 0x66, 0x72,
        0x6f, 0x6d, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x37, 0x0a, 0x0a, 0x74, 0x6f, 0x5f,
        0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x42, 0x18, 0xd2,
        0xb4, 0x2d, 0x14, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73,
        0x73, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x09, 0x74, 0x6f, 0x41, 0x64, 0x64, 0x72, 0x65,
        0x73, 0x73, 0x12, 0x79, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x03,
        0x28, 0x0b, 0x32, 0x19, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x73, 0x65,
        0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x43, 0x6f, 0x69, 0x6e, 0x42, 0x46, 0xc8,
        0xde, 0x1f, 0x00, 0xaa, 0xdf, 0x1f, 0x28, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f,
        0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d,
        0x73, 0x64, 0x6b, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x43, 0x6f, 0x69, 0x6e, 0x73, 0x9a,
        0xe7, 0xb0, 0x2a, 0x0c, 0x6c, 0x65, 0x67, 0x61, 0x63, 0x79, 0x5f, 0x63, 0x6f, 0x69, 0x6e, 0x73,
        0xa8, 0xe7, 0xb0, 0x2a, 0x01, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x3a, 0x30, 0x88,
        0xa0, 0x1f, 0x00, 0xe8, 0xa0, 0x1f, 0x00, 0x82, 0xe7, 0xb0, 0x2a, 0x0c, 0x66, 0x72, 0x6f, 0x6d,
        0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x8a, 0xe7, 0xb0, 0x2a, 0x12, 0x63, 0x6f, 0x73,
        0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x4d, 0x73, 0x67, 0x53, 0x65, 0x6e, 0x64, 0x22,
        0x11, 0x0a, 0x0f, 0x4d, 0x73, 0x67, 0x53, 0x65, 0x6e, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
        0x73, 0x65, 0x22, 0xbc, 0x01, 0x0a, 0x0c, 0x4d, 0x73, 0x67, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x53,
        0x65, 0x6e, 0x64, 0x12, 0x3d, 0x0a, 0x06, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x73, 0x18, 0x01, 0x20,
        0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e,
        0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x42,
        0x09, 0xc8, 0xde, 0x1f, 0x00, 0xa8, 0xe7, 0xb0, 0x2a, 0x01, 0x52, 0x06, 0x69, 0x6e, 0x70, 0x75,
        0x74, 0x73, 0x12, 0x40, 0x0a, 0x07, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x73, 0x18, 0x02, 0x20,
        0x03, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e,
        0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x4f, 0x75, 0x74, 0x70, 0x75, 0x74,
        0x42, 0x09, 0xc8, 0xde, 0x1f, 0x00, 0xa8, 0xe7, 0xb0, 0x2a, 0x01, 0x52, 0x07, 0x6f, 0x75, 0x74,
        0x70, 0x75, 0x74, 0x73, 0x3a, 0x2b, 0xe8, 0xa0, 0x1f, 0x00, 0x82, 0xe7, 0xb0, 0x2a, 0x06, 0x69,
        0x6e, 0x70, 0x75, 0x74, 0x73, 0x8a, 0xe7, 0xb0, 0x2a, 0x17, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
        0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x4d, 0x73, 0x67, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x53, 0x65, 0x6e,
        0x64, 0x22, 0x16, 0x0a, 0x14, 0x4d, 0x73, 0x67, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x53, 0x65, 0x6e,
        0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0xbf, 0x01, 0x0a, 0x0f, 0x4d, 0x73,
        0x67, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x12, 0x36, 0x0a,
        0x09, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
        0x42, 0x18, 0xd2, 0xb4, 0x2d, 0x14, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x41, 0x64, 0x64,
        0x72, 0x65, 0x73, 0x73, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x09, 0x61, 0x75, 0x74, 0x68,
        0x6f, 0x72, 0x69, 0x74, 0x79, 0x12, 0x3e, 0x0a, 0x06, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x18,
        0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62,
        0x61, 0x6e, 0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x50, 0x61, 0x72, 0x61,
        0x6d, 0x73, 0x42, 0x09, 0xc8, 0xde, 0x1f, 0x00, 0xa8, 0xe7, 0xb0, 0x2a, 0x01, 0x52, 0x06, 0x70,
        0x61, 0x72, 0x61, 0x6d, 0x73, 0x3a, 0x34, 0x82, 0xe7, 0xb0, 0x2a, 0x09, 0x61, 0x75, 0x74, 0x68,
        0x6f, 0x72, 0x69, 0x74, 0x79, 0x8a, 0xe7, 0xb0, 0x2a, 0x21, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
        0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x78, 0x2f, 0x62, 0x61, 0x6e, 0x6b, 0x2f, 0x4d, 0x73, 0x67, 0x55,
        0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x22, 0x19, 0x0a, 0x17, 0x4d,
        0x73, 0x67, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x52, 0x65,
        0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0xe9, 0x01, 0x0a, 0x11, 0x4d, 0x73, 0x67, 0x53, 0x65,
        0x74, 0x53, 0x65, 0x6e, 0x64, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x12, 0x36, 0x0a, 0x09,
        0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42,
        0x18, 0xd2, 0xb4, 0x2d, 0x14, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x41, 0x64, 0x64, 0x72,
        0x65, 0x73, 0x73, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x09, 0x61, 0x75, 0x74, 0x68, 0x6f,
        0x72, 0x69, 0x74, 0x79, 0x12, 0x43, 0x0a, 0x0c, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x65, 0x6e, 0x61,
        0x62, 0x6c, 0x65, 0x64, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x63, 0x6f, 0x73,
        0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31,
        0x2e, 0x53, 0x65, 0x6e, 0x64, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x52, 0x0b, 0x73, 0x65,
        0x6e, 0x64, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x12, 0x26, 0x0a, 0x0f, 0x75, 0x73, 0x65,
        0x5f, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x5f, 0x66, 0x6f, 0x72, 0x18, 0x03, 0x20, 0x03,
        0x28, 0x09, 0x52, 0x0d, 0x75, 0x73, 0x65, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x46, 0x6f,
        0x72, 0x3a, 0x2f, 0x82, 0xe7, 0xb0, 0x2a, 0x09, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74,
        0x79, 0x8a, 0xe7, 0xb0, 0x2a, 0x1c, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b,
        0x2f, 0x4d, 0x73, 0x67, 0x53, 0x65, 0x74, 0x53, 0x65, 0x6e, 0x64, 0x45, 0x6e, 0x61, 0x62, 0x6c,
        0x65, 0x64, 0x22, 0x1b, 0x0a, 0x19, 0x4d, 0x73, 0x67, 0x53, 0x65, 0x74, 0x53, 0x65, 0x6e, 0x64,
        0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22,
        0x94, 0x01, 0x0a, 0x07, 0x4d, 0x73, 0x67, 0x42, 0x75, 0x72, 0x6e, 0x12, 0x3b, 0x0a, 0x0c, 0x66,
        0x72, 0x6f, 0x6d, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28,
        0x09, 0x42, 0x18, 0xd2, 0xb4, 0x2d, 0x14, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x41, 0x64,
        0x64, 0x72, 0x65, 0x73, 0x73, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x0b, 0x66, 0x72, 0x6f,
        0x6d, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x31, 0x0a, 0x06, 0x61, 0x6d, 0x6f, 0x75,
        0x6e, 0x74, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f,
        0x73, 0x2e, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x43,
        0x6f, 0x69, 0x6e, 0x52, 0x06, 0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x3a, 0x19, 0x88, 0xa0, 0x1f,
        0x00, 0xe8, 0xa0, 0x1f, 0x00, 0x82, 0xe7, 0xb0, 0x2a, 0x0c, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x61,
        0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x22, 0x11, 0x0a, 0x0f, 0x4d, 0x73, 0x67, 0x42, 0x75, 0x72,
        0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x32, 0xcd, 0x03, 0x0a, 0x03, 0x4d, 0x73,
        0x67, 0x12, 0x4a, 0x0a, 0x04, 0x53, 0x65, 0x6e, 0x64, 0x12, 0x1c, 0x2e, 0x63, 0x6f, 0x73, 0x6d,
        0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e,
        0x4d, 0x73, 0x67, 0x53, 0x65, 0x6e, 0x64, 0x1a, 0x24, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
        0x2e, 0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x4d, 0x73,
        0x67, 0x53, 0x65, 0x6e, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x59, 0x0a,
        0x09, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x53, 0x65, 0x6e, 0x64, 0x12, 0x21, 0x2e, 0x63, 0x6f, 0x73,
        0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31,
        0x2e, 0x4d, 0x73, 0x67, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x53, 0x65, 0x6e, 0x64, 0x1a, 0x29, 0x2e,
        0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65,
        0x74, 0x61, 0x31, 0x2e, 0x4d, 0x73, 0x67, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x53, 0x65, 0x6e, 0x64,
        0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x4a, 0x0a, 0x04, 0x42, 0x75, 0x72, 0x6e,
        0x12, 0x1c, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x76,
        0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x4d, 0x73, 0x67, 0x42, 0x75, 0x72, 0x6e, 0x1a, 0x24,
        0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x76, 0x31, 0x62,
        0x65, 0x74, 0x61, 0x31, 0x2e, 0x4d, 0x73, 0x67, 0x42, 0x75, 0x72, 0x6e, 0x52, 0x65, 0x73, 0x70,
        0x6f, 0x6e, 0x73, 0x65, 0x12, 0x62, 0x0a, 0x0c, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61,
        0x72, 0x61, 0x6d, 0x73, 0x12, 0x24, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61,
        0x6e, 0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x4d, 0x73, 0x67, 0x55, 0x70,
        0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x1a, 0x2c, 0x2e, 0x63, 0x6f, 0x73,
        0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31,
        0x2e, 0x4d, 0x73, 0x67, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73,
        0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x68, 0x0a, 0x0e, 0x53, 0x65, 0x74, 0x53,
        0x65, 0x6e, 0x64, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x12, 0x26, 0x2e, 0x63, 0x6f, 0x73,
        0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31,
        0x2e, 0x4d, 0x73, 0x67, 0x53, 0x65, 0x74, 0x53, 0x65, 0x6e, 0x64, 0x45, 0x6e, 0x61, 0x62, 0x6c,
        0x65, 0x64, 0x1a, 0x2e, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e, 0x6b,
        0x2e, 0x76, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x2e, 0x4d, 0x73, 0x67, 0x53, 0x65, 0x74, 0x53,
        0x65, 0x6e, 0x64, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
        0x73, 0x65, 0x1a, 0x05, 0x80, 0xe7, 0xb0, 0x2a, 0x01, 0x42, 0xc2, 0x01, 0x0a, 0x17, 0x63, 0x6f,
        0x6d, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x62, 0x61, 0x6e, 0x6b, 0x2e, 0x76, 0x31,
        0x62, 0x65, 0x74, 0x61, 0x31, 0x42, 0x07, 0x54, 0x78, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01,
        0x5a, 0x30, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x73, 0x64, 0x6b, 0x2e, 0x69, 0x6f, 0x2f, 0x61,
        0x70, 0x69, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x62, 0x61, 0x6e, 0x6b, 0x2f, 0x76,
        0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x3b, 0x62, 0x61, 0x6e, 0x6b, 0x76, 0x31, 0x62, 0x65, 0x74,
        0x61, 0x31, 0xa2, 0x02, 0x03, 0x43, 0x42, 0x58, 0xaa, 0x02, 0x13, 0x43, 0x6f, 0x73, 0x6d, 0x6f,
        0x73, 0x2e, 0x42, 0x61, 0x6e, 0x6b, 0x2e, 0x56, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0xca, 0x02,
        0x13, 0x43, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x5c, 0x42, 0x61, 0x6e, 0x6b, 0x5c, 0x56, 0x31, 0x62,
        0x65, 0x74, 0x61, 0x31, 0xe2, 0x02, 0x1f, 0x43, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x5c, 0x42, 0x61,
        0x6e, 0x6b, 0x5c, 0x56, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65,
        0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x15, 0x43, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x3a,
        0x3a, 0x42, 0x61, 0x6e, 0x6b, 0x3a, 0x3a, 0x56, 0x31, 0x62, 0x65, 0x74, 0x61, 0x31, 0x62, 0x06,
        0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
    ])?;

    Ok(())
}