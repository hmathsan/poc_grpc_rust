fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(&["proto\\helloworld.proto", 
                        "proto\\category.proto", 
                        "proto\\product.proto"], 
        &["proto"])?;

    Ok(())
}