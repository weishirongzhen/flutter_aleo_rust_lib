

fn main (){

    // 设置对应的库
    // if cfg!(target_os = "ios") {
        //  cargo lipo --targets aarch64-apple-ios-sim
        println!("cargo:rustc-link-search=native=/Users/gtmickey/project/flutter_aleo_rust_lib/ios_curl_lib/artifacts-iphonesimulator/lib");

        //  cargo lipo --targets aarch64-apple-ios
        // println!("cargo:rustc-link-search=native=/Users/gtmickey/project/flutter_aleo_rust_lib/ios_curl_lib/artifacts-iphoneos/lib");


    // }


}