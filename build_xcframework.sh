#!/bin/bash

xcodebuild -create-xcframework \
-framework ./build/ios/Release-iphoneos/rust_lib_flutter_aleo_rust_lib/rust_lib_flutter_aleo_rust_lib.framework \
-framework ./build/ios/Debug-iphonesimulator/rust_lib_flutter_aleo_rust_lib/rust_lib_flutter_aleo_rust_lib.framework \
-output ./rust_lib_flutter_aleo_rust_lib.xcframework

