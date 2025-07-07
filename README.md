<p align="center">
  <a href="https://load.network">
    <img src="https://gateway.load.rs/bundle/0x83cf4417880af0d2df56ce04ecfc108ea4ee940e8fb81400e31ab81571e28d21/0">
  </a>
</p>


## About

`zentra-sp` is a load0 storage provider for [zentra](https://zentra.dev) -- this API takes a zentra function call txid (curretly supports base sepolia), downloads the tx data and finally store on on Load Network via load0 storage pipeline.

> Zentra is a consensus-agnostic subspace for developers to build decentralized applications using Python.

## Whitelisted zentra sequencer addresses

* `0x00000000000000000000000000000000007a656e`
* `0xbBe844467051fE04CCE050efB42cF824431d1e52`

## Use the API

```bash
curl -X POST https://zentra-sp-ap9i.shuttle.app/process/{zentra_txid}
```

### Example

```bash
curl -X POST https://zentra-sp-ap9i.shuttle.app/process/0x469548e90a48718d2f4aacf8d4d7b4e6c66563b83fdd6bd193927321d7816355
```

Example response: once the API return the `load_hash`, you can access the data via the `https://load0.network/resolve/{load_hash}` endpoint

```bash
{"load_hash":"0x1a945eed257520ce9bd317ce9abf778cffd277774a4739bf97fbabb0ba940183"}
```

## License
This repository is licensed under the [MIT License](./LICENSE).