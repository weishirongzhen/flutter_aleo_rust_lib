import 'dart:developer';

import 'package:convert/convert.dart';
import 'package:flutter/material.dart';
import 'package:flutter_aleo_rust_lib/src/rust/api/aleo_api.dart';
import 'package:flutter_aleo_rust_lib/src/rust/api/simple.dart';
import 'package:flutter_aleo_rust_lib/src/rust/frb_generated.dart';

import 'aleo/aleo_hd_key.dart';

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
  late String seed;

  /// fiscal document grain ecology wheat around sport nice guitar topple add north
  /// seed f2812b1b381e14d4fa7e0f3545ff3d1570df3cbdd4f5ab5f84f3f6da71989642beab844da5ad56fd1795a3bb55c6319c3228ceac1273706be5946ecc778e5153
  /// APrivateKey1zkp8zjQLSTzbswrPzDMEEysPP8aCJ8qUdWYvbtLAjfKufp8
  /// addreee aleo1m8gqcxedmqfp2ylh8f96w6n3z7zw0ucahenq0symvxpqg0f8sugqd4we6f
  /// 其他标准参考数据
  /// APrivateKey1zkp96vBfhFFeo6hHrDkdYwxTjJSmL8S7cgdezbrD5c7Tmiw
  /// AViewKey1gFMrMPFBZZufJTwZULkuQejjsQZ9cp3y3ysiV8t55yRt
  /// aleo1m5tkjhn8586xgnwcq3mcmffp9gmuyeq2fdz6lsj647fsckcqzygqxphn62
  @override
  void initState() {
    pk = privateKeyNew();
    seed = "f2812b1b381e14d4fa7e0f3545ff3d1570df3cbdd4f5ab5f84f3f6da71989642beab844da5ad56fd1795a3bb55c6319c3228ceac1273706be5946ecc778e5153";
    // pk = "APrivateKey1zkp96vBfhFFeo6hHrDkdYwxTjJSmL8S7cgdezbrD5c7Tmiw";
    dp();
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
                'from seed: `${fromSeedUnchecked(seed: hex.decode("f2812b1b381e14d4fa7e0f3545ff3d1570df3cbdd4f5ab5f84f3f6da71989642"))}`'),
            Text('private key: `$pk`'),
            Text('view key: `${toViewKey(pk: pk)}`'),
            Text(
                'address 1: `${toAddress(privateKey: fromSeedUnchecked(seed: hex.decode("6f2f42f8777c6d333bd72e67800fa956f812d7ada2457953f9811d078d6c6e62")))}`'),
            Text('address: `${toAddress(privateKey: pk)}`'),
            Text(
                'sign output: `${sign(messageHex: "1".codeUnits, privateKey: pk)}`'),

            Text(
                'private key 1: `${ fromSeedUnchecked(seed: hex.decode("6f2f42f8777c6d333bd72e67800fa956f812d7ada2457953f9811d078d6c6e62"))}`'),

          ],
        ),
      ),
    );
  }

  void dp() {
    final key = derivePath("m/44'/0'/0'/0'",
        seed);
    log("wtf key: ${hex.encode(key.key!)}");
    final pk = fromSeedUnchecked(
        seed:key.key!);
 /// APrivateKey1zkp8zjQLSTzbswrPzDMEEysPP8aCJ8qUdWYvbtLAjfKufp8
    log("pk : $pk");
  }
}
