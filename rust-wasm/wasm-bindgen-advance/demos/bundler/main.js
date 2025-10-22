// 不要用默认导入，改用命名空间导入，避免 “没有 default 导出” 的语法报错
import * as mod from './pkg/wasm_bindgen_advance.js';
// 用 ?url 让 Vite 处理 wasm 资源并返回 URL
import wasmUrl from './pkg/wasm_bindgen_advance_bg.wasm?url';

// 兼容不同 glue 导出名：default / init / __wbg_init / initSync
const initFn = mod.default || mod.init || mod.__wbg_init || mod.initSync;
// 有 init 就调用（传对象，避免 deprecated 提示）；有的 glue 在导入时已完成初始化，则此处会跳过
if (typeof initFn === 'function') {
    await initFn({ url: wasmUrl });
}

const { User } = mod;

const u = new User('Carol', 20);
document.getElementById('app').textContent = [
    u.greet(),
    `name(before set): ${u.name}`,
].join('\n');

u.name = 'Dave';
const p = document.createElement('pre');
p.textContent = [
    `name(after set): ${u.name}`,
    u.greet(),
].join('\n');
document.body.appendChild(p);
