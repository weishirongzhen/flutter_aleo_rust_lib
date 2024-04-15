### Run on iOS will need curl lib, but iphone sdk does not provide it. so need to build it correctly

download tar.gz from https://curl.se, do not clone with github, configure file may not included
## prepare env:
for arm64 ios-simulator

```shell
export ARCH=arm64
export SDK=iphonesimulator
export DEPLOYMENT_TARGET=12.0
sudo xcode-select --switch /Applications/Xcode.app
export CFLAGS="-arch $ARCH -isysroot $(xcrun -sdk $SDK --show-sdk-path) -m$SDK-version-min=$DEPLOYMENT_TARGET"

./configure --host=$ARCH-apple-darwin --prefix $(pwd)/artifacts-iphonesimulator  --with-secure-transport
make -j8
make install
```


for arm64 ios

```shell
export ARCH=arm64
export SDK=iphoneos
export DEPLOYMENT_TARGET=12.0
sudo xcode-select --switch /Applications/Xcode.app
export CFLAGS="-arch $ARCH -isysroot $(xcrun -sdk $SDK --show-sdk-path) -m$SDK-version-min=$DEPLOYMENT_TARGET"

./configure --host=$ARCH-apple-darwin --prefix $(pwd)/artifacts-iphoneos  --with-secure-transport
make -j8
make install
```

- after flutter run, open xcode -> select you rust lib framework -> build phrase -> Link Binary With libraries -> add dylib
- build settings -> Link Search Paths -> add dylib path


# remember to remove "x86_64" on rust_builder/cargokit/build_tool/lib/src/environment.dart or else will build failed, because there is no x86 lib

```dart
  static List<String> get darwinArchs {
    final r = _getEnv("CARGOKIT_DARWIN_ARCHS").split(' ');
    r.remove("x86_64");
    return r;
  }
```



