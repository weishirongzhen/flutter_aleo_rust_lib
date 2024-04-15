import 'dart:async';
import 'dart:developer';

import 'package:convert/convert.dart';
import 'package:flutter/material.dart';
import 'package:flutter_aleo_rust_lib/src/rust/api/aleo_api.dart' as aleo;
import 'package:flutter_aleo_rust_lib/src/rust/api/simple.dart';
import 'package:flutter_aleo_rust_lib/src/rust/frb_generated.dart';
import 'package:bip39_mnemonic/bip39_mnemonic.dart';

import 'aleo_hd_key.dart';

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
  int seconds = 0;
  String transferId = "waiting";
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
            FutureBuilder<String>(
                future: privateKeyFromDerivePath(testMnemonic, 0),
                builder: (context, snapshot) {
                  return Text('private key1: `${snapshot.data ?? ""}`');
                }),
            FutureBuilder<String>(
                future: viewKeyFromDerivePath(testMnemonic, 0),
                builder: (context, snapshot) {
                  return Text('view key1: `${snapshot.data ?? ""}');
                }),
            FutureBuilder<String>(
                future: addressFromDerivePath(testMnemonic, 0),
                builder: (context, snapshot) {
                  return Text('address1: `${snapshot.data ?? ""}`');
                }),
            const Divider(),
            // Text(
            //     'private key2: `${privateKeyFromDerivePath(testMnemonic, 1)}`'),
            // Text('view key2: `${viewKeyFromDerivePath(testMnemonic, 1)}'),
            // Text('address2: `${addressFromDerivePath(testMnemonic, 1)}`'),




            Text('public transfer',style: TextStyle(fontSize: 16,fontWeight: FontWeight.bold),),
            ElevatedButton(
                onPressed: () {
                  buildTransfer();
                },
                child: const Text("transfer")),

            if (transferring) CircularProgressIndicator(),
            Text("time use: ${seconds ~/ 1000} s"),
            Text('id: `$transferId`'),
          ],
        ),
      ),
    );
  }

  void buildTransfer() async {
    print("start transferring");
    setState(() {
      transferring = true;
      transferId = "waiting";
    });
    seconds = DateTime.now().millisecondsSinceEpoch;
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
      seconds = DateTime.now().millisecondsSinceEpoch - seconds;
    });
  }

  Future<String> viewKeyFromDerivePath(String mnemonic, int index) async {
    return aleo.toViewKey(
        privateKey: await privateKeyFromDerivePath(mnemonic, index));
  }

  Future<String> addressFromDerivePath(String mnemonic, int index) async {
    return aleo.toAddress(
        privateKey: await privateKeyFromDerivePath(mnemonic, index));
  }

  Future<String> privateKeyFromDerivePath(String mnemonic, int index) async {
    /// aleo hd account derive path m/44'/0'/<account_index>'/0' and default account_index = 0

    final path = "m/44'/0'/$index'/0'";
    final m = Mnemonic.fromSentence(mnemonic, Language.english);
    final seedHex = hex.encode(m.seed);
    final keys = derivePath(path, seedHex);
    return aleo.privateKeyFromSeed(seed: keys.key!);
  }
}
