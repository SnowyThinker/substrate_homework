import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { KeyringPair} from '@polkadot/keyring/types';
import { metadata} from '@polkadot/types/interfaces/essentials';
import '@polkadot/api-augment';
const { u8aToString } = require("@polkadot/util");


const WEB_SOCKET = 'ws://127.0.0.1:9944';


const connectSubstrate = async() => {
    const wsProvider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({ provider: wsProvider});
    await api.isReady;
    console.log('connection to substrate is OK.');
    return api;
};

//get const value
const getConst = async(api: ApiPromise) => {
    const existentialDeposit = await api.consts.balances.existentialDeposit.toHuman();
    return existentialDeposit;
};

// get free balance variable
const getFreeBalance = async(api: ApiPromise, address: string) => {
    const aliceAccount = await api.query.system.account(address);
    // console.log('aliceAccount: ', aliceAccount);
    return aliceAccount["data"]["free"].toHuman();
}

const printAliceBobBalance = async (api: ApiPromise) => {
    const keyring = new Keyring({ type: 'sr25519'});
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');

    console.log('alice balance is: ', await getFreeBalance(api, alice.address));
    console.log('bob balance is: ', await getFreeBalance(api, bob.address));
}

const transferFromAliceToBob = async (api: ApiPromise, amount: number) => {
    const keyring = new Keyring({ type: 'sr25519'});
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');

    await api.tx.balances.transfer(bob.address, amount)
        .signAndSend(alice, res => {
            console.log(`Tx status: ${res.status}`);
        });
}

// subscribe balance change
const subscribeAliceBalance = async (api: ApiPromise) => {
    const keyring = new Keyring({ type: 'sr25519'});
    const alice = keyring.addFromUri('//Alice');
    await api.query.system.account(alice.address, aliceAcct => {
        console.log('subscribed to Alice account.');
        const aliceFreeSub = aliceAcct.data.free;
        console.log(`Alice Account (sub): ${aliceFreeSub}`);
    });
}

// get metadata
const getMetadata = async(api: ApiPromise) => {
    const metadata = await api.rpc.state.getMetadata();
    console.log('print metadata:');
    console.log(metadata);
    return metadata;
} 

const getStorage = async (api: ApiPromise) => {
    // let kind = 'PERSISTENT'; 
    let key = 'kuaidi100::indexing_parcel_weight';
    let value = await api.rpc.offchain.localStorageGet("PERSISTENT", key);

    const hexValue = value.toHex();
    const u8aValue = new Uint8Array(
        (hexValue.match(/.{1,2}/g) || []).map((byte) => parseInt(byte, 16))
    );

    const stringValue = u8aToString(u8aValue);

    console.log("value in offchain storage: ", stringValue);
}

function sleep(ms: number) {
    return new Promise( resolve => setTimeout(resolve, ms) );
}

const main = async() => {
    const api = await connectSubstrate();
    // console.log('const value existentialDeposit is:', await getConst(api));

    // await printAliceBobBalance(api);
    // await transferFromAliceToBob(api, 10 * 12);
    // await sleep(6000);

    // await printAliceBobBalance(api);
    // await subscribeAliceBalance(api);
    // await sleep(600000);

    // await getMetadata(api);
    await getStorage(api);

    console.log("game over");
};

main()
    .then(() => {
        console.log("succesfully exited");
        process.exit(0);
    })
    .catch(err => {
        console.log('error occur: ', err);
        process.exit(1);
    })