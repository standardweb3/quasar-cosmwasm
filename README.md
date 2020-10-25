# Quasar Cosmwasm 🌌

<p align="center">
  <img src="/doc/quasar-cosmwasm.png" width="300">
</p>

[![GitHub license](https://img.shields.io/github/license/quasar-protocol/quasar-cosmwasm)](LICENSE) 
[![Twitter](https://img.shields.io/twitter/follow/SubstrateSwap?label=Follow&style=social)](https://twitter.com/SubstrateSwap)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)]()


Quasar Cosmwasm is the implementation of quasar in cosmwasm supported blockchains

## Trying it out

### For developers

Developers can run the implementation and test with client sdk from its Cosmos SDK based blockchains.

Here are the instructions that one should follow on setting up the environment to hack this project.

#### Terra

First, install [houston](https://github.com/terra-project/houston).

```
npm install -g @terra-money/houston
```

In case of Testing on Terra you can connect to datahub from Figment or run localterra on the device then deploy contracts in local environment.

#### Localterra

Clone localterra repo and run docker-compose in the project root directory.

```
git clone https://github.com/terra-project/LocalTerra.git
cd LocalTerra
docker-compose up
```

### For users

The current interface implementation is on its way to production.
Check updates in this [repo](https://github.com/quasar-protocol/quasar-interface)

## Documentation

For more research and details, check out the documentation link below.

<a href="http://doc.quasar.money/"><img src="/doc/gitbook.png" width="300"></a>

## License

The license of this project follows [Apache2.0](./LICENSE)
