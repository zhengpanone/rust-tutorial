use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct User {
    // 私有字段需要getter/setter
    name: String,
    // 公开字段，这样JS才能访问
    pub age: u32,
}

#[wasm_bindgen]
impl User {
    // 构造函数
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, age: u32) -> User {
        User { name, age }
    }
    // 方法
    pub fn greet(&self) -> String {
        format!(
            "Hello, my name is {} and I'm {} year old.",
            self.name, self.age
        )
    }
    // getter
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
    // setter
    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
