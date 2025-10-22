```shell
# 在 Node 环境运行
cd wasm-bindgen-advance
wasm-pack build --target nodejs --out-dir demos/node/pkg --out-name wasm_bindgen_advance
cd demos/node
npm run start

# 供打包器使用
cd wasm-bindgen-advance
wasm-pack build --target bundler --out-dir demos/bundler/pkg --out-name wasm_bindgen_advance
cd demos/bundler
npm i
npm run dev

# 纯浏览器不打包：
cd wasm-bindgen-advance
wasm-pack build --target web --out-dir demos/web/pkg --out-name wasm_bindgen_advance
# Node
npx serve -s .
# 或者
npx http-server .
# Python
python3 -m http.server
```