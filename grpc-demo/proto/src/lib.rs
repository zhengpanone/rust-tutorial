// proto/src/main
// 这个文件可以是空的，或者导出生成的代码
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

// 重新导出，方便使用

pub mod common {
    tonic::include_proto!("common");
}

pub mod hello {
    tonic::include_proto!("hello");
}
pub mod user {
    tonic::include_proto!("user");
    // include!(concat!(env!("OUT_DIR"), "/user_handler"));
}

pub mod product {
    tonic::include_proto!("product");
}

// ✅ 给 Reflection 使用
pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/descriptor.bin"));
