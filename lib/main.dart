import 'package:flutter/material.dart';
import 'package:flutter_aleo_rust_lib/src/rust/api/private_key_api.dart';
import 'package:flutter_aleo_rust_lib/src/rust/api/simple.dart';
import 'package:flutter_aleo_rust_lib/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  late String pk;

  /// seed f2812b1b381e14d4fa7e0f3545ff3d1570df3cbdd4f5ab5f84f3f6da71989642
  /// right jazz asset blush shell violin cactus jar legend hint uphold vivid, seed: 4fc075d5a84a474b766d6222fc39f3200d2e1f1d13b444a50db885643890c171
  ///
  /// 标准参考数据
  /// APrivateKey1zkp96vBfhFFeo6hHrDkdYwxTjJSmL8S7cgdezbrD5c7Tmiw
  /// AViewKey1gFMrMPFBZZufJTwZULkuQejjsQZ9cp3y3ysiV8t55yRt
  /// aleo1m5tkjhn8586xgnwcq3mcmffp9gmuyeq2fdz6lsj647fsckcqzygqxphn62
  @override
  void initState() {
    // pk = privateKeyNew();
    pk = "APrivateKey1zkp96vBfhFFeo6hHrDkdYwxTjJSmL8S7cgdezbrD5c7Tmiw";
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter aleo rust lib')),
        body: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
                'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
            Text(
                'from seed: `${fromSeedUnchecked(seed: "4fc075d5a84a474b766d6222fc39f3200d2e1f1d13b444a50db885643890c171")}`'),
            Text('private key: `$pk`'),
            Text('view key: `${toViewKey(pk: pk)}`'),
            Text('address: `${toAddress(pk: pk)}`'),
          ],
        ),
      ),
    );
  }
}
