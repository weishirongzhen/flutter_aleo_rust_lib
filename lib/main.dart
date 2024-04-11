import 'dart:developer';

import 'package:convert/convert.dart';
import 'package:flutter/material.dart';
import 'package:flutter_aleo_rust_lib/src/rust/api/aleo_api.dart' as aleo;
import 'package:flutter_aleo_rust_lib/src/rust/api/simple.dart';
import 'package:flutter_aleo_rust_lib/src/rust/frb_generated.dart';
import 'package:bip39_mnemonic/bip39_mnemonic.dart';

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
  String aleoTestnet3Url = "https://api.explorer.aleo.org/v1/testnet3";
  bool transferring = false;
  String transferId = "";
  final testMnemonic =
      "lesson maid remove boring swift floor produce crouch kangaroo action kick pole";

  @override
  void initState() {
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
            const Divider(),
            Text("words: $testMnemonic"),
            const Divider(),
            Text(
                'private key1: `${privateKeyFromDerivePath(testMnemonic, 0)}`'),
            Text('view key1: `${viewKeyFromDerivePath(testMnemonic, 0)}'),
            Text('address1: `${addressFromDerivePath(testMnemonic, 0)}`'),
            const Divider(),
            Text(
                'private key2: `${privateKeyFromDerivePath(testMnemonic, 1)}`'),
            Text('view key2: `${viewKeyFromDerivePath(testMnemonic, 1)}'),
            Text('address2: `${addressFromDerivePath(testMnemonic, 1)}`'),
            ElevatedButton(
                onPressed: () {
                  buildTransfer();
                },
                child: const Text("transfer")),

            if(transferring)
            CircularProgressIndicator(),
            Text('id: `$transferId`'),
          ],
        ),
      ),
    );
  }

  void buildTransfer() async {
    setState(() {
      transferring = true;
    });

    final res = await aleo.transfer(
      recipient:
          "aleo19jjmsrusvuduyxgufd7ax24p2sp73eedx0agky7tzfa0su66wcgqlmqz4x",
      transferType: "public",
      amount: 200000000,
      fee: 0.0,
      privateFee: false,
      privateKey: "APrivateKey1zkp8zjQLSTzbswrPzDMEEysPP8aCJ8qUdWYvbtLAjfKufp8",
    );

    print("transfer id = $res");
    setState(() {
      transferring = false;
      transferId = res;
    });
  }

  String viewKeyFromDerivePath(String mnemonic, int index) {
    return aleo.toViewKey(
        privateKey: privateKeyFromDerivePath(mnemonic, index));
  }

  String addressFromDerivePath(String mnemonic, int index) {
    return aleo.toAddress(
        privateKey: privateKeyFromDerivePath(mnemonic, index));
  }

  String privateKeyFromDerivePath(String mnemonic, int index) {
    /// aleo hd account derive path m/44'/0'/<account_index>'/0' and default account_index = 0

    final path = "m/44'/0'/$index'/0'";
    final m = Mnemonic.fromSentence(mnemonic, Language.english);
    final seedHex = hex.encode(m.seed);
    final keys = derivePath(path, seedHex);
    return aleo.privateKeyFromSeed(seed: keys.key!);
  }
}
