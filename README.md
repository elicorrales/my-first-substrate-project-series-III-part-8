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
  
# Why ```metadata```?  
  
  
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
  
(make sure you copied the ```Contract``` value.)  



## Now Run Client  
  
Make sure that the ```contractAddress``` matches the value you copied from ```Contract``` when you instantiated.  
  
Make sure your ```metadataPath = ``` contains the path to ```helloworld/target/ink/metadata.json```.
  
Run the client.  
  
You should see proper output on both client and substrate node.  
  
## Now Change ```metadataPath```
  
Change the path from containing ```helloworld```, to ```helloworld2```.  
  
Re-run the client.  
  
What do you see?  
  
It complains about ```sayhello``` is not a function.  
  
We do know that it **is** a function of the uploaded/instantiated ```helloworld```.
  
## Now Change The Function Call
  
Find in the code where it calls ```sayhello``` and change it to ```sayhello2```.  
  
Re-run the client.
  
What do you see?
  
This error is interesting.  The previous error was a local stack trace by the client.  
  
This current error is a result of what the contracts-node returned.  
  
Why?  
  
Because in this case, the ```metadata``` matches/contains the ```sayhello2``` function, **BUT** when we tried to call it on the instantiated contract, it errored because it doesn't really exist.  
  
  
## What Did We Learn?  
  
- The ```metadata``` is used for some checking, but in the end, it has to match what is deployed.


# What Is ```gasLimit```?  
  
> The term gas limit refers to the maximum price 
> a cryptocurrency user is willing to pay 
> when sending a transaction, 
> or performing a smart contract function.

> These fees are calculated in gas unit,
> and the gas limit defines the maximum value 
> that the transaction or function can "charge" 
> or take from the user.

> As such, the gas limit works as a security mechanism
> that prevents high fees from being incorrectly charged
> due to a bug or error in a smart contract.  

So I programmed a loop where I started the gasLimit at some level that caused an error, and the loop incremented the gasLimit, and re-tried to call the contract ```sayhello```, until I found the magic number.
```
    //let gasLimit = 74999922687; //causes error - gasLimit too low
    let gasLimit   = 74999922688; // returns an OK result
    let { result } = await contract.query.sayhello(aliceAddress, { gasLimit });
    console.log('gasLimit:', gasLimit, ',  result:', result.toHuman());
```

# Last Thing - The Debug Messages, The Panic Message
  

