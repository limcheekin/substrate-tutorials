// A demonstration of interacting with custom RPCs using Polkadot js API

const { ApiPromise, WsProvider } = require('@polkadot/api');
const { readFileSync } = require('fs');
const { Keyring } = require('@polkadot/keyring');
const { decodeAddress } = require('@polkadot/util-crypto');
const { u8aToHex } = require('@polkadot/util');


// Construct parameters for API instance
const wsProvider = new WsProvider('ws://localhost:9944');
const types = {};
const rpc = {
  sumStorage: {
    getSum: {
      description: "Gets the sum of the two storage values in sum-storage pallet via a runtime api.",
      params: [],
      type: "u32",
    }
  }
}

// REF: https://stackoverflow.com/questions/66998019/get-public-key-from-ss58-address
function getPublicKeyInHex(address) {
  return u8aToHex(getPublicKey(address));
}

function getPublicKey(address) {
  return decodeAddress(address);
}

async function main() {
  // Construct the actual api
  const api = await ApiPromise.create({
    provider: wsProvider,
    types,
    rpc,
  });

  // Query raw storage values, the oldschool way
  const v1 = (await api.query.sumStorage.thing1()).toNumber();
  const v2 = (await api.query.sumStorage.thing2()).toNumber();
  console.log(`The individual storage values are ${v1}, and ${v2}.`);
  console.log(`The sum calculated in javascript is ${v1 + v2}\n`);

  // Query the custom RPC that uses the runtimeAPI
  let directSum = (await api.rpc.sumStorage.getSum()).toNumber();
  console.log(`The sum queried directly from the RPC is ${directSum}`);

  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.createFromUri('//Alice');
  const bob = keyring.createFromUri('//Bob');
  console.log("alice.address", alice.address);
  console.log("alice.publicKey", u8aToHex(alice.publicKey));
  console.log("alice.hexPublicKey", getPublicKeyInHex(alice.address));

  let channelId = "V1StGXR8_Z5jdHi6B-myT";
  let accountCommonKeys = new Map([
    [alice.address, "alice common key"],
    [bob.address, "bob common key"]
  ]);

  await api.tx.subMessage
    .newChannel(channelId, accountCommonKeys)
    .signAndSend(alice, ({ status, dispatchError }) => {
      status.isFinalized
        ? console.log(`😉 Finalized. Block hash: ${status.asFinalized.toString()}`)
        : console.log(`Current transaction status: ${status.type}`)

      // status would still be set, but in the case of error we can shortcut
      // to just check it (so an error would indicate InBlock or Finalized)
      if (dispatchError) {
        if (dispatchError.isModule) {
          // for module errors, we have the section indexed, lookup
          const decoded = api.registry.findMetaError(dispatchError.asModule);
          const { section, name } = decoded;
          console.error(`${section} - ${name}`);
        } else {
          // Other, CannotLookup, BadOrigin, no extra info
          console.error(dispatchError.toString());
        }
      }
    })
    .catch((err) => {
      console.log(`😞 Transaction Failed: ${err.toString()}`)
    });

    const commonKey = await api.query.subMessage.commonKeyByChannelIdAccountId(channelId, alice.address);
    console.log('commonKey', commonKey);

}

main().catch(console.error).finally(() => process.exit());
