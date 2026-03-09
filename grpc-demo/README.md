# DDD 项目
## 创建 workspace
```shell
mkdir grpc-service && cd grpc-service
cargo new --lib proto
cargo new --lib server
cargo new --lib client
cargo new --bin cli-client
```


## 根目录 Cargo.toml

```shell
cat > Cargo.toml << 'EOF'
[workspace]
members = ["proto", "server", "client", "cli-client"]
resolver = "2"
EOF
```

## 运行项目
```shell

# 原来的命令
cargo run -p auth-server

# 使用 cargo watch
cargo watch -x 'run -p auth-server'
cargo watch -x 'run -p auth-server' --ignore 'auth-server.log.*' --ignore 'logs'


cargo watch -x 'run --package auth-server'
cargo watch -w user-server -x 'run'
# 带有清理和重建
cargo watch -x 'clean -p auth-server' -x 'run -p auth-server'

# 验证
grpcurl -plaintext 127.0.0.1:50051 list
```

## 项目结构

```text
auth-server/
├── src/
│   ├── main.rs                          # 应用入口
│   ├── lib.rs                          # 库入口
│   │
│   ├── api/                            # API层 (HTTP接口)
│   │   ├── http/                      # HTTP API
│   │   │   ├── v1/                         # API版本1
│   │   │   │   ├── mod.rs
│   │   │   │   ├── users/                  # 用户相关接口
│   │   │   │   ├── auth/                   # 认证相关接口
│   │   │   │   ├── system/                 # 系统管理接口
│   │   │   │   └── health/                 # 健康检查接口
│   │   ├── grpc/                      # gRPC API
│   │   │   ├── generated/            # 生成的代码
│   │   │   ├── services/             # gRPC服务实现
│   │   │   ├── client/               # gRPC客户端
│   │   │   ├── middleware/
│   │   │   ├── converter/
│   │   │   └── server.rs
│   │   │
│   │   ├── middleware/                 # API中间件
│   │   ├── dto/                        # API数据传输对象
│   │   ├── response/                   # API响应格式
│   │   └── error/                      # API错误处理
│   │
│   ├── app/                            # 应用层
│   │   ├── bootstrap/                  # 应用引导
│   │   ├── config/                     # 配置管理
│   │   ├── state/                      # 应用状态
│   │   ├── middleware/                 # 中间件
│   │   └── error/                      # 应用错误
│   │
│   ├── domain/                         # 领域层
│   │   ├── common/                     # 公共领域
│   │   ├── identity/                   # 身份认证子域
│   │   ├── system/                     # 系统管理子域
│   │   └── message/                    # 消息子域
│   │
│   ├── application/                    # 应用服务层
│   │   ├── services/                   # 应用服务
│   │   ├── handlers/                   # 领域事件处理程序
│   │   └── projections/                # 查询投影
│   │
│   ├── infrastructure/                 # 基础设施层
│   │   ├── persistence/                # 持久化
│   │   ├── web/                        # Web框架集成
│   │   ├── grpc/                       # gRPC框架
│   │   ├── cache/                      # 缓存
│   │   ├── message_queue/              # 消息队列
│   │   └── external/                   # 外部服务


```

## 应用服务和领域服务的区别

## 🎯 应用服务 vs 领域服务 对比

| 对比维度 | 应用服务 (Application Service) | 领域服务 (Domain Service) |
|-----------|----------------------------------|-----------------------------|
| **所在层级** | 应用层 (Application Layer) | 领域层 (Domain Layer) |
| **核心职责** | 用例协调、事务控制、整合多个组件 | 核心业务规则、复杂领域逻辑 |
| **依赖对象** | 仓储、领域服务、消息队列、缓存、外部 API | 仓储、聚合根、值对象 |
| **典型方法** | `register_user()`<br>`update_profile()` | `is_email_registered()`<br>`calculate_reputation()` |
| **状态特征** | 通常无状态 | 可无状态，也可有状态 |
| **可测试性** | 需 Mock 仓储、缓存、消息队列、外部服务 | 只需 Mock 仓储，专注业务规则 |
| **输入类型** | DTO（如 CreateUserRequest） | 领域对象（User、Email、UserId） |
| **输出类型** | DTO | 领域对象或标量值 |
| **事务边界** | 负责定义和管理事务 | 不关心事务 |
| **事件处理** | 负责发布领域事件 | 只产生事件，不发布 |
| **关注点** | “做什么”——完成一个用例 | “怎么做”——实现业务规则 |
| **是否可直接暴露给接口层** | 是 | 否（应通过应用服务间接调用） |


## 🎨 设计原则

### 📁 应用服务 (Application Service) 应该：

- **薄 (Thin)**  
  只负责流程协调，不实现核心业务规则。

- **无状态 (Stateless)**  
  不保存业务状态，只编排调用流程。

- **事务边界 (Transaction Boundary)**  
  明确定义事务范围（如一个用例一个事务）。

- **依赖注入 (Dependency Injection)**  
  通过构造函数注入仓储、领域服务、缓存、消息组件等依赖。

- **错误处理 (Error Mapping)**  
  将领域错误转换为应用层错误（例如 `DomainError → AppError`）。

---

### 📁 领域服务 (Domain Service) 应该：

- **纯业务逻辑 (Pure Business Logic)**  
  专注实现核心领域规则。

- **高可测试性 (Highly Testable)**  
  不依赖外部基础设施（或仅依赖仓储接口），易于单元测试。

- **与聚合协作 (Collaborate with Aggregates)**  
  操作领域对象（聚合根、值对象）。

- **无副作用 (No Direct Side Effects)**  
  不直接调用外部系统；通过聚合产生领域事件。

- **单一职责 (Single Responsibility)**  
  每个领域服务只处理一个特定的业务概念。

---

## ✅ 这种分离带来的好处

- **可测试性 (Testability)**  
  领域服务可以独立进行单元测试。

- **可维护性 (Maintainability)**  
  核心业务逻辑集中在领域层，修改成本低。

- **可扩展性 (Scalability / Extensibility)**  
  应用层可以轻松增加缓存、消息队列、外部 API 等基础设施。

- **清晰分层 (Clear Separation of Concerns)**  
  每一层职责明确，依赖方向清晰。


# TODO

https://yuanbao.tencent.com/chat/naQivTmsDa?chatMode=temp
```sql

-- Add migration script here

-- 角色服务权限表
CREATE TABLE `auth_service`.`role_service_permissions` (
  `id` BIGINT PRIMARY KEY AUTO_INCREMENT,
  `role_id` INT NOT NULL COMMENT '角色ID',
  `service_id` INT NOT NULL COMMENT '服务ID',
  `permission_type` TINYINT NOT NULL DEFAULT 1 COMMENT '权限类型：1-只读，2-读写，3-管理',
  `permission_rules` JSON COMMENT '权限规则，JSON格式，可自定义规则',
  `data_scope` VARCHAR(50) COMMENT '数据范围：ALL-全部，DEPT-本部门，SELF-仅自己',
  `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  `created_by` BIGINT COMMENT '创建人',
  `expired_at` TIMESTAMP NULL COMMENT '权限过期时间',
  UNIQUE KEY `uk_role_service` (`role_id`, `service_id`),
  INDEX `idx_role` (`role_id`),
  INDEX `idx_service` (`service_id`),
  FOREIGN KEY (`role_id`) REFERENCES `system_roles`(`role_id`) ON DELETE CASCADE,
  FOREIGN KEY (`service_id`) REFERENCES `micro_services`(`service_id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='角色服务权限表';

-- 角色模块权限表
CREATE TABLE `auth_service`.`role_module_permissions` (
  `id` BIGINT PRIMARY KEY AUTO_INCREMENT,
  `role_id` INT NOT NULL COMMENT '角色ID',
  `module_id` INT NOT NULL COMMENT '模块ID',
  `can_access` BOOLEAN DEFAULT TRUE COMMENT '是否可以访问',
  `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE KEY `uk_role_module` (`role_id`, `module_id`),
  INDEX `idx_role` (`role_id`),
  INDEX `idx_module` (`module_id`),
  FOREIGN KEY (`role_id`) REFERENCES `system_roles`(`role_id`) ON DELETE CASCADE,
  FOREIGN KEY (`module_id`) REFERENCES `service_modules`(`module_id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='角色模块权限表';

-- 角色接口权限表
CREATE TABLE `auth_service`.`role_api_permissions` (
  `id` BIGINT PRIMARY KEY AUTO_INCREMENT,
  `role_id` INT NOT NULL COMMENT '角色ID',
  `api_id` INT NOT NULL COMMENT '接口ID',
  `permission_action` VARCHAR(20) NOT NULL COMMENT '操作权限：query,create,update,delete,export等',
  `permission_condition` JSON COMMENT '权限条件，JSON格式的过滤条件',
  `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE KEY `uk_role_api_action` (`role_id`, `api_id`, `permission_action`),
  INDEX `idx_role` (`role_id`),
  INDEX `idx_api` (`api_id`),
  FOREIGN KEY (`role_id`) REFERENCES `system_roles`(`role_id`) ON DELETE CASCADE,
  FOREIGN KEY (`api_id`) REFERENCES `service_apis`(`api_id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='角色接口权限表';

-- 角色-资源关联表
CREATE TABLE `role_resources` (
  `id` BIGINT PRIMARY KEY AUTO_INCREMENT,
  `role_id` INT NOT NULL COMMENT '角色ID',
  `resource_id` INT NOT NULL COMMENT '资源ID',
  `permission_type` TINYINT NOT NULL DEFAULT 1 COMMENT '权限类型：1-允许，2-拒绝',
  `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE KEY `uk_role_resource` (`role_id`, `resource_id`),
  INDEX `idx_role` (`role_id`),
  INDEX `idx_resource` (`resource_id`),
  FOREIGN KEY (`role_id`) REFERENCES `roles`(`role_id`) ON DELETE CASCADE,
  FOREIGN KEY (`resource_id`) REFERENCES `api_resources`(`resource_id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='角色权限关联表';


-- 用户角色关联表
CREATE TABLE `auth_service`.`user_role_mappings` (
  `id` BIGINT PRIMARY KEY AUTO_INCREMENT,
  `user_id` BIGINT NOT NULL COMMENT '用户ID',
  `role_id` INT NOT NULL COMMENT '角色ID',
  `source_type` TINYINT DEFAULT 1 COMMENT '来源：1-手动分配，2-职位继承，3-自动分配',
  `source_ref_id` BIGINT COMMENT '来源关联ID，如部门ID、职位ID',
  `effective_start` TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '生效开始时间',
  `effective_end` TIMESTAMP NULL COMMENT '生效结束时间',
  `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  `created_by` BIGINT COMMENT '创建人',
  UNIQUE KEY `uk_user_role_time` (`user_id`, `role_id`, `effective_start`),
  INDEX `idx_user` (`user_id`),
  INDEX `idx_role` (`role_id`),
  INDEX `idx_effective` (`effective_start`, `effective_end`),
  FOREIGN KEY (`role_id`) REFERENCES `system_roles`(`role_id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='用户角色关联表';


-- 服务角色表（服务作为调用方的身份）
CREATE TABLE `auth_service`.`service_roles` (
  `service_role_id` INT PRIMARY KEY AUTO_INCREMENT,
  `service_id` INT NOT NULL COMMENT '服务ID',
  `role_code` VARCHAR(50) UNIQUE NOT NULL COMMENT '服务角色编码',
  `role_name` VARCHAR(100) NOT NULL COMMENT '服务角色名称',
  `description` VARCHAR(500) COMMENT '描述',
  `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE KEY `uk_service_role` (`service_id`, `role_code`),
  FOREIGN KEY (`service_id`) REFERENCES `micro_services`(`service_id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='服务角色表';

-- 服务间调用权限表
CREATE TABLE `auth_service`.`service_to_service_permissions` (
  `id` BIGINT PRIMARY KEY AUTO_INCREMENT,
  `caller_service_id` INT NOT NULL COMMENT '调用方服务ID',
  `caller_role_id` INT NOT NULL COMMENT '调用方服务角色ID',
  `target_service_id` INT NOT NULL COMMENT '被调用服务ID',
  `target_api_id` INT NOT NULL COMMENT '可调用的接口ID',
  `permission_level` TINYINT DEFAULT 1 COMMENT '权限级别：1-只读，2-读写',
  `rate_limit` INT DEFAULT 1000 COMMENT '调用频率限制（次/分钟）',
  `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE KEY `uk_call_target_api` (`caller_service_id`, `caller_role_id`, `target_service_id`, `target_api_id`),
  INDEX `idx_caller` (`caller_service_id`, `caller_role_id`),
  INDEX `idx_target` (`target_service_id`),
  FOREIGN KEY (`caller_service_id`) REFERENCES `micro_services`(`service_id`),
  FOREIGN KEY (`caller_role_id`) REFERENCES `service_roles`(`service_role_id`),
  FOREIGN KEY (`target_service_id`) REFERENCES `micro_services`(`service_id`),
  FOREIGN KEY (`target_api_id`) REFERENCES `service_apis`(`api_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='服务间调用权限表';


-- 服务接口表
CREATE TABLE `auth_service`.`service_apis` (
  `api_id` INT PRIMARY KEY AUTO_INCREMENT,
  `module_id` INT NOT NULL COMMENT '所属模块',
  `api_code` VARCHAR(100) UNIQUE NOT NULL COMMENT '接口编码',
  `api_name` VARCHAR(100) NOT NULL COMMENT '接口名称',
  `api_path` VARCHAR(200) NOT NULL COMMENT '接口路径',
  `http_method` VARCHAR(10) NOT NULL COMMENT 'HTTP方法',
  `api_desc` VARCHAR(500) COMMENT '接口描述',
  `required_permission` VARCHAR(200) COMMENT '所需权限标识',
  `is_public` BOOLEAN DEFAULT FALSE COMMENT '是否公开接口',
  `rate_limit` INT DEFAULT 0 COMMENT '限流值，0表示不限流',
  `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  INDEX `idx_module` (`module_id`),
  INDEX `idx_path_method` (`api_path`(100), `http_method`),
  FOREIGN KEY (`module_id`) REFERENCES `service_modules`(`module_id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='服务接口表';

-- 接口资源表
CREATE TABLE `api_resources` (
  `resource_id` INT PRIMARY KEY AUTO_INCREMENT COMMENT '资源ID',
  `service_id` INT NOT NULL COMMENT '所属服务',
  `resource_code` VARCHAR(100) UNIQUE NOT NULL COMMENT '资源编码，格式: 服务编码:接口路径:方法',
  `resource_name` VARCHAR(100) NOT NULL COMMENT '资源名称',
  `resource_path` VARCHAR(200) NOT NULL COMMENT '接口路径，如 /api/v1/users',
  `http_method` VARCHAR(10) NOT NULL COMMENT 'HTTP方法，GET/POST/PUT/DELETE等',
  `resource_type` TINYINT NOT NULL DEFAULT 1 COMMENT '资源类型：1-API，2-菜单，3-按钮',
  `is_public` BOOLEAN DEFAULT FALSE COMMENT '是否公开接口',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：1-启用，0-停用',
  `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  INDEX `idx_service` (`service_id`),
  INDEX `idx_method_path` (`http_method`, `resource_path`(100)),
  UNIQUE KEY `uk_service_path_method` (`service_id`, `resource_path`, `http_method`),
  FOREIGN KEY (`service_id`) REFERENCES `services`(`service_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='接口资源表';

-- 服务密钥表
CREATE TABLE `service_secrets` (
  `secret_id` INT PRIMARY KEY AUTO_INCREMENT,
  `service_id` INT NOT NULL COMMENT '服务ID',
  `app_key` VARCHAR(100) UNIQUE NOT NULL COMMENT '服务应用Key',
  `app_secret` VARCHAR(255) NOT NULL COMMENT '服务密钥',
  `secret_type` VARCHAR(20) NOT NULL COMMENT '密钥类型：JWT, API_KEY, OAUTH2',
  `is_active` BOOLEAN DEFAULT TRUE COMMENT '是否激活',
  `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  `expires_at` TIMESTAMP NULL COMMENT '过期时间',
  UNIQUE KEY `uk_service_secret` (`service_id`, `secret_type`),
  FOREIGN KEY (`service_id`) REFERENCES `services`(`service_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='服务密钥表';

-- 服务间调用权限表
CREATE TABLE `service_permissions` (
  `id` BIGINT PRIMARY KEY AUTO_INCREMENT,
  `caller_service_id` INT NOT NULL COMMENT '调用方服务ID',
  `target_service_id` INT NOT NULL COMMENT '被调用服务ID',
  `resource_id` INT NOT NULL COMMENT '可访问的资源ID',
  `permission_level` TINYINT DEFAULT 1 COMMENT '权限级别：1-只读，2-读写，3-管理',
  `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE KEY `uk_caller_target_resource` (`caller_service_id`, `target_service_id`, `resource_id`),
  INDEX `idx_caller` (`caller_service_id`),
  INDEX `idx_target` (`target_service_id`),
  FOREIGN KEY (`caller_service_id`) REFERENCES `services`(`service_id`),
  FOREIGN KEY (`target_service_id`) REFERENCES `services`(`service_id`),
  FOREIGN KEY (`resource_id`) REFERENCES `api_resources`(`resource_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='服务间调用权限表';

-- 1. 插入服务定义
INSERT INTO `auth_service`.`micro_services` (service_code, service_name, service_type, service_desc) VALUES
('USER_SERVICE', '用户服务', 1, '管理用户认证、资料'),
('PRODUCT_SERVICE', '商品服务', 1, '管理商品、分类、库存'),
('ORDER_SERVICE', '订单服务', 1, '处理订单、物流'),
('PAYMENT_SERVICE', '支付服务', 1, '处理支付、退款');

-- 2. 插入模块定义
INSERT INTO `auth_service`.`service_modules` (service_id, module_code, module_name) VALUES
(1, 'USER_MANAGE', '用户管理模块'),
(1, 'AUTH', '认证授权模块'),
(2, 'PRODUCT_MANAGE', '商品管理模块'),
(2, 'CATEGORY_MANAGE', '分类管理模块'),
(3, 'ORDER_MANAGE', '订单管理模块'),
(4, 'PAYMENT_MANAGE', '支付管理模块');

-- 3. 插入角色定义
INSERT INTO `auth_service`.`system_roles` (role_code, role_name, role_type, role_level, role_desc, is_system) VALUES
('SUPER_ADMIN', '超级管理员', 1, 100, '系统超级管理员，拥有所有服务权限', TRUE),
('USER_ADMIN', '用户管理员', 2, 50, '管理用户服务的角色', FALSE),
('PRODUCT_ADMIN', '商品管理员', 2, 50, '管理商品服务的角色', FALSE),
('ORDER_ADMIN', '订单管理员', 2, 50, '管理订单服务的角色', FALSE),
('FINANCE', '财务人员', 3, 30, '财务人员，可查看订单和支付服务', FALSE),
('CUSTOMER_SERVICE', '客服', 3, 20, '客服人员，可查看用户和订单服务', FALSE),
('COMMON_USER', '普通用户', 1, 10, '普通用户，基础权限', TRUE);

-- 4. 角色-服务关联（核心：不同的角色可访问不同的服务）
INSERT INTO `auth_service`.`role_service_permissions` (role_id, service_id, permission_type, data_scope) VALUES
-- 超级管理员：所有服务
(1, 1, 3, 'ALL'),  -- 用户服务-管理权限
(1, 2, 3, 'ALL'),  -- 商品服务-管理权限
(1, 3, 3, 'ALL'),  -- 订单服务-管理权限
(1, 4, 3, 'ALL'),  -- 支付服务-管理权限

-- 用户管理员：只能管理用户服务
(2, 1, 3, 'ALL'),  -- 用户服务-管理权限

-- 商品管理员：只能管理商品服务
(3, 2, 3, 'ALL'),  -- 商品服务-管理权限

-- 财务人员：可读订单和支付服务
(5, 3, 1, 'ALL'),  -- 订单服务-只读
(5, 4, 1, 'ALL'),  -- 支付服务-只读

-- 客服：可读写用户和订单服务
(6, 1, 2, 'DEPT'),  -- 用户服务-读写，本部门数据
(6, 3, 2, 'DEPT'),  -- 订单服务-读写，本部门数据

-- 普通用户：只能访问用户服务（自己的资料）和商品服务（浏览）
(7, 1, 1, 'SELF'),  -- 用户服务-只读，仅自己数据
(7, 2, 1, 'ALL');   -- 商品服务-只读，全部商品


-- 1. 查询用户有哪些角色
SELECT role_id FROM `auth_service`.`user_role_mappings` 
WHERE user_id = 123 
  AND (effective_end IS NULL OR effective_end > NOW());

-- 2. 查询这些角色可以访问哪些服务
SELECT DISTINCT s.service_code, s.service_name, rsp.permission_type, rsp.data_scope
FROM `auth_service`.`role_service_permissions` rsp
JOIN `auth_service`.`micro_services` s ON rsp.service_id = s.service_id
WHERE rsp.role_id IN (1, 2, 3)  -- 用户的角色ID
  AND (rsp.expired_at IS NULL OR rsp.expired_at > NOW());

-- 3. 查询具体接口权限
SELECT a.api_path, a.http_method, rap.permission_action
FROM `auth_service`.`role_api_permissions` rap
JOIN `auth_service`.`service_apis` a ON rap.api_id = a.api_id
WHERE rap.role_id IN (1, 2, 3)
  AND a.api_path = '/api/v1/users'
  AND a.http_method = 'GET';

```