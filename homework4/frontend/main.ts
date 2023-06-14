import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { KeyringPair} from '@polkadot/keyring/types';
import { metadata} from '@polkadot/types/interfaces/essentials';
import '@polkadot/api-augment';
// import { resolve } from "path";


const WEB_SOCKET = 'ws://127.0.0.1:9944';
// const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
// const sleep = (waitSeconds) => {
//     return new Promise<void>(resolve => {
//         setTimeout(() => {
// 			resolve()
// 		}, waitSeconds * 1000)
//     })
// }

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

function sleep(ms: number) {
    return new Promise( resolve => setTimeout(resolve, ms) );
}

const main = async() => {
    const api = await connectSubstrate();
    console.log('const value existentialDeposit is:', await getConst(api));

    await printAliceBobBalance(api);
    await transferFromAliceToBob(api, 10 * 12);
    await sleep(6000);

    await printAliceBobBalance(api);
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