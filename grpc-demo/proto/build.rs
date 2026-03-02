// proto/build.rs

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    unsafe {
        std::env::set_var("PROTOC", protoc);
    }
    // let proto_dir = PathBuf::from("./proto");

    // 定义要编译的所有 proto 文件
    let protos = &[
        "proto/common/common.proto",
        "proto/hello.proto",
        "proto/user_service.proto",
        "proto/auth_service.proto",
        "proto/product.proto",
    ];

    // 包含路径
    let includes = &["proto"];

    // 创建输出目录
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);

    // 设置描述符文件路径
    let descriptor_path = out_dir.join("grpc_descriptor.bin");

    // 编译 proto文件
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        // ✅ Reflection 必须
        .file_descriptor_set_path(&descriptor_path)
        // .type_attribute(
        //     ".microservice.user.User",
        //     "#[derive(serde::Serialize, serde::Deserialize)]",
        // )
        // .type_attribute(
        //     ".microservice.auth.AuthResponse",
        //     "#[derive(serde::Serialize, serde::Deserialize)]"
        // )
        // .type_attribute(
        //     ".microservice.common.CommonResponse",
        //     "#[derive(serde::Serialize, serde::Deserialize)]"
        // )
        // ❌ 不要 out_dir("src")
        // .out_dir("src") // 输出到src目录
        .compile_protos(protos, includes)?;
    // 重新编译 proto文件
    for proto in protos {
        println!("cargo:rerun-if-changed={}", proto);
    }

    // ✅ 将描述符文件复制到项目根目录（可选）
    let _ = std::fs::copy(&descriptor_path, "grpc_descriptor.bin");
    Ok(())
}
