# IOTA SDK EVM Library - WebAssembly Bindings

WebAssembly (Wasm) bindings for TypeScript/JavaScript to the IOTA SDK EVM library.

## Which Bindings to Choose?

The IOTA SDK library also offers dedicated [Node.js bindings](../nodejs). The differences with this package are outlined
below.

**NOTE: Use the Node.js bindings if you can. The Wasm bindings are more portable and exist to support browser
environments.**

|              |   Wasm bindings   |   Node.js bindings    |
|:-------------|:-----------------:|:---------------------:|
| Environment  | Node.js, browsers |        Node.js        |
| Installation |         -         | Rust, Cargo required* |
| Performance  |         ✔️         |         ✔️✔️            |
| Ledger Nano  |         ❌        |          ✔️            |
| Rocksdb      |         ❌        |          ✔️            |
| Stronghold   |         ❌        |          ✔️            |

* The Node.js bindings only need to be compiled during `npm install` if a pre-compiled binary is not available for your
  platform.

## Requirements

- One of the following Node.js versions: '16.x', '18.x';
- `wasm-bindgen` (`cargo install wasm-bindgen-cli`);

## Getting Started

### Installing Using a Package Manager

To install the library from your package manager of choice, you only need to run the following:

#### npm

```bash
npm i @iota/sdk-evm-wasm
```

#### yarn

```bash
yarn add @iota/sdk-evm-wasm
```

### Web Setup

The library loads the compiled Wasm file with an HTTP GET request, so the `iota_sdk_evm_wasm_bg.wasm` file must be copied to
the root of the distribution folder.

A bundler such as [webpack](https://webpack.js.org/) or [rollup](https://rollupjs.org/) is recommended.

### Rollup

1. Install `rollup-plugin-copy`:

    ```bash
    npm install rollup-plugin-copy --save-dev
    ```

2. Add the plugin to your `rollup.config.js`:

    ```js
    // Include the copy plugin.
    import copy from 'rollup-plugin-copy'
    
    // ...
    
    // Add the copy plugin to the `plugins` array:
    copy({
      targets: [{
        src: 'node_modules/@iota/sdk-evm-wasm/web/wasm/iota_sdk_evm_wasm_bg.wasm',
        dest: 'public',
        rename: 'iota_sdk_evm_wasm_bg.wasm'
      }]
    })
    ```

### Webpack

1. Install `copy-webpack-plugin`:

    ```bash
    npm install copy-webpack-plugin --save-dev
    ```

2. Add the plugin to your `webpack.config.js`:

    ```js
    // Include the copy plugin.
    const CopyWebPlugin = require('copy-webpack-plugin');
    
    // ...
    
    experiments: {
        // futureDefaults: true, // includes asyncWebAssembly, topLevelAwait etc.
        asyncWebAssembly: true
    }
    
    // Add the copy plugin to the `plugins` array:
    plugins: [
        new CopyWebPlugin({
          patterns: [
            {
              from: 'node_modules/@iota/sdk-evm-wasm/web/wasm/iota_sdk_evm_wasm_bg.wasm',
              to: 'iota_sdk_evm_wasm_bg.wasm'
            }
          ]
        }),
        // other plugins...
    ]
    ```

## Api Usage

The following example creates an [`Api`](https://wiki.iota.org/shimmer/iota-sdk-evm/references/nodejs/classes/Api/)
instance connected to
the [Shimmer Testnet](https://api.testnet.shimmer.network), and retrieves the node's information by
calling [`Api.getInfo()`](https://wiki.iota.org/shimmer/iota-sdk-evm/references/nodejs/classes/Api/#getinfo),
and then print the node's information.

### Node.js

```javascript
const {Api, initLogger} = require('@iota/sdk-evm-wasm/node');

async function run() {
    initLogger();

    let api = await Api.create(process.env.WASP_NODE as string);

    try {
        const nodeInfo = await api.getInfo();
        console.log('Node info: ', nodeInfo);
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
```

### Web

```javascript
import init, {Api, initLogger} from "@iota/sdk-evm-wasm/web";

init().then(async () => {
    initLogger();

    let api = await Api.create(process.env.WASP_NODE as string);

    try {
        const nodeInfo = await api.getInfo();
        console.log('Node info: ', nodeInfo);
    } catch (error) {
        console.error('Error: ', error);
    }
}).catch(console.error);

// Default path to load is "iota_sdk_evm_wasm_bg.wasm", 
// but you can override it by passing a path explicitly.
//
// init("./static/iota_sdk_evm_wasm_bg.wasm").then(...)
```

## API Reference

If you are using the Wasm binding, you use the Node.js API reference in the
[IOTA Wiki](https://wiki.iota.org/shimmer/iota-sdk/references/nodejs/api_ref/).
