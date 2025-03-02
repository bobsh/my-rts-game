// build.rs
fn main() {
    println!("cargo:warning=Build script executing...");
    
    #[cfg(target_os = "windows")]
    {
        println!("cargo:warning=Windows target detected");
        let icon_path = "assets/icons/quillbrainstars/quillbrainstars-64x64.ico";
        
        // Verify file exists
        if std::path::Path::new(icon_path).exists() {
            println!("cargo:warning=Icon file found at {}", icon_path);
            
            let mut res = winres::WindowsResource::new();
            res.set_icon(icon_path);
            res.set("FileDescription", "My RTS Game"); 
            res.set("ProductName", "My RTS Game");
            res.set("FileVersion", "0.1.0");
            res.set("LegalCopyright", "Copyright © 2025");
            res.set_icon_with_id(icon_path, "32512");
            match res.compile() {
                Ok(_) => println!("cargo:warning=Icon compiled successfully"),
                Err(e) => println!("cargo:warning=Icon compilation failed: {}", e),
            }
        } else {
            println!("cargo:warning=Icon file not found at {}", icon_path);
            println!("cargo:warning=Current dir: {:?}", std::env::current_dir().unwrap());
        }
    }
}
