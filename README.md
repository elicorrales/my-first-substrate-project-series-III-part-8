# my-first-substrate-project-series-III-part-8

### This project is part of a series and includes a video.

See [Here](https://github.com/elicorrales/blockchain-tutorials/blob/main/README.md) for the overall document that
refers to all the series.  
  
# We Discuss The ```client.js``` Code In More Detail

## Intro: In Part 7 We Were Able to Interact With A Deployed Contract
We did that at the end, really quick, without going into any details.  
 
Let's do that now.  
  
Here is most of the code but I removed some parts just to keep things clearer.  
  
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
```
  
  
In order to help us shed some light on the question of the ```metadata```....
  

## Let's Create Another Smart Contract
  
The easiest way to do this, go to your smart-contract sub-project.  
  
My entire project looks like so: 
  
```
$ tree -L 2
.
├── my-first-client
│   ├── client.js
│   ├── node_modules
│   ├── package-lock.json
│   └── package.json
└── my-first-smart-contracts
    ├── helloworld
    └── upload.instantiate.call.sh

4 directories, 4 files
```
  
So, in my case, I am just going to copy all of ```helloworld``` and name the new one, ```helloworld2```.  
 
So now I have:  
  
```
.
├── my-first-client
│   ├── client.js
│   ├── node_modules
│   ├── package-lock.json
│   └── package.json
└── my-first-smart-contracts
    ├── helloworld
    ├── helloworld2  <----------------- new
    └── upload.instantiate.call.sh

5 directories, 4 files
```
  
Now, ```cd``` into ```helloworld2```, and edit ```Cargo.toml```.  
  
Right near the top, under the ```[package]``` section, find the ```name = ``` and make sure it says ```helloworld2```.  Save and exit.
 
Next, edit ```lib.rs```.   Everywhere you see ```Hello```, ```hello```, ```helloworld```, ```HelloWorld```, ```sayhello```, ```saybye``` - add a ```2``` at the end of the word.  
  

## Build Both Contracts

Go into each of those directories and do a ```cargo +nightly contract build```.  
  

## Start ```substrate-contracts-node```  
  
```
substrate-contracts-node --dev --detailed-log-output -lruntime::contracts=trace
```
  
## Upload And Instantiate ```helloworld```, But NOT ```helloworld2```.  
  
```
cargo contract upload --suri //Alice
```
  
(make sure you copied the ```Hash code``` value.)  
   
```
hash_code=<the hash code you copied>  
```
  
```
cargo contract instantiate \
  --gas 500000000000 \
  --constructor new \
  --suri //Alice \
  --code-hash $code_hash
```
  
(make sure you copiked the ```Contract``` value.)  



