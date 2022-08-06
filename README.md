# my-first-substrate-project-series-III-part-8

### This project is part of a series and includes a video.

See [Here](https://github.com/elicorrales/blockchain-tutorials/blob/main/README.md) for the overall document that
refers to all the series.  
  
# We Discuss The ```client.js`` Code In More Detail

### In Part 7 We Were Able to Interact With A Deployed Contract
We did that at the end, really quick, without going into any details.  
 
Let's do that now.  
  
Here is most of the code but I removed some part just to keep things clearer.  
  
Let's focus for a bit on ```contract.query.sayhello```, and also go back to my earlier question of, **what is the use or purpose of metadata**?

```
const aliceAddress = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
const gasLimit = -1;

const metadataPath = '../my-first-smart-contracts/helloworld/target/ink/metadata.json';
const metadata = require(metadataPath);

const contractAddress = '5H2EXJWscxyMLjmxKP2KhJmQ6JsUr67MQfgEoUUkrizXVgbz';

(async () => {
    const ws = new WsProvider(wsUrl);
    const wsApi = await ApiPromise.create({ provider: ws });
    const contract = new ContractPromise(wsApi, metadata, contractAddress);

    console.log('query:\n', contract.query);
    console.log('tx:\n', contract.tx);

    const { result, output } = await contract.query.sayhello(aliceAddress, { gasLimit });
    console.log('result:', result.toHuman());

    wsApi.disconnect();
})();

