import 'dart:developer';

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
  String seed = "3664fbd2c2349a7e7b8dbcae438113ee05898c5b209383b1381b1d864253d60b3e52b47234b52fa9524b09f6c5ee46116910b710ae6fce8b9197573a08cd7c7b";
  String aleoTestnet3Url = "https://api.explorer.aleo.org/v1/testnet3";

  /// APrivateKey1zkp96vBfhFFeo6hHrDkdYwxTjJSmL8S7cgdezbrD5c7Tmiw
  /// AViewKey1gFMrMPFBZZufJTwZULkuQejjsQZ9cp3y3ysiV8t55yRt
  /// aleo1m5tkjhn8586xgnwcq3mcmffp9gmuyeq2fdz6lsj647fsckcqzygqxphn62
  @override
  void initState() {
    pk = privateKeyNew();
    log("viewwkey = ${toViewKey(privateKey: "APrivateKey1zkp4rNXyNvjMggBrqd3kJAjjPHfUEdmdo1xApZB8BVu5buJ")}");
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
            Text('private key: `$pk`'),
            Text('view key: `${toViewKey(privateKey: "APrivateKey1zkp8zjQLSTzbswrPzDMEEysPP8aCJ8qUdWYvbtLAjfKufp8")}`'),
            Text('address: `${toAddress(privateKey: pk)}`'),
            Text('derivePath account index 0: `${dp()}`'),
            Text(
                'sign output: `${sign(messageBytes: "1".codeUnits, privateKey: pk)}`'),

            ElevatedButton(onPressed: (){

              buildTransfer();
            }, child: Text("Test")),

          ],
        ),
      ),
    );
  }

  String dp() {
    final key = derivePath("m/44'/0'/0'/0'", seed);
    final pk = fromSeedUnchecked(seed: key.key!);

    return pk;
  }

  void buildTransfer(){

    // transfer(privateKey: '', amountCredits: 0.0, recipient: '', transferType: '', feeCredits: 0.0);
  }
}
