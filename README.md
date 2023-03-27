# Enway CRM Smart Contract

<b>This is a project aimed at replicating a fully functional CRM (Customer relationship management) onto a program
(smart contract) deployed on 
the Solana blockchain.</b>

<i>Development: </i>
<b>Solana Blockchain(solana-test-validator, solana_program,borsh_serializer), CLion IDE, WebStorm IDE, React(with 
Phantom wallet)</b>

## Build instructions
Install <a href="https://docs.solana.com/cli/install-solana-cli-tools">Solana tool suite</a>
<br>

```shell
solana-keygen new
```
<i>Start a local instance of the Solana Network.</i>
```shell
solana-test-validator
```
Compile a <b>.so</b> file by running the command in <b>Solana-Smart-Contract</b> folder (in Git Bash as admin)
```shell
cargo-build-bpf
```
deploy the build file onto the local blockchain network
```shell
solana program deploy target/deploy/Solana_Rust_2023.so 
```
Run the JS script
```shell
node javascript/index.js
```
