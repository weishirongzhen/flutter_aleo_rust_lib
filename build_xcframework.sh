#!/bin/bash

xcodebuild -create-xcframework \
-framework ./temp_ios/ios/rust_lib_flutter_aleo_rust_lib/rust_lib_flutter_aleo_rust_lib.framework \
-framework ./temp_ios/ios_sim/rust_lib_flutter_aleo_rust_lib/rust_lib_flutter_aleo_rust_lib.framework \
-output ./temp_ios/rust_lib_flutter_aleo_rust_lib.xcframework

