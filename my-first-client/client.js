const { WsProvider, ApiPromise, Keyring } = require('@polkadot/api');
const { ContractPromise } = require('@polkadot/api-contract');
const wsUrl = 'ws://localhost:9944';

const aliceAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

let metadataPath = '../my-first-smart-contracts/helloworld/target/ink/metadata.json';

const metadata = require(metadataPath);

//let contractAddress = '5CfQobR3ePUTKNn84VzxhgwppFtYQBHwtKD39unAvvYqrM6M';
let contractAddress = '5FaZEryob4C8r7L74eQ4csPispyG86UP2JRbpgE4pgZKibue';
(async () => {
    let ws = new WsProvider(wsUrl);
    let wsApi = await ApiPromise.create({ provider: ws });
    const contract = new ContractPromise(wsApi, metadata, contractAddress);

    const gasLimit = -1;

    //const { result, output } = await contract.query.sayhello(aliceAddress, { gasLimit });
    //await contract.query.saybye(aliceAddress, { gasLimit });
    contract.query.saybye(aliceAddress, { gasLimit });

    //console.log('result:', result.toHuman());
    //console.log('out:', output != undefined ? output.toHuman() : null);

    wsApi.disconnect();
})();
