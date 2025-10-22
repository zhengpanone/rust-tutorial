import { User } from './pkg/wasm_bindgen_advance.js'

async function run() {

    // 使用构造函数创建实例
    const user = new User('Alice', 28);

    // 访问公共字段
    console.log('Age:', user.age);

    // 调用方法
    console.log(user.greet());

    // 使用getter/setter
    console.log('Name:', user.name);
    user.name = 'Bob';
    console.log(user.greet());

    // 别忘了释放内存
    user.free();
}
run();