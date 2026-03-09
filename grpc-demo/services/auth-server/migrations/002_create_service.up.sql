-- migrations/002_create_service.up.sql
-- 创建枚举类型
DROP TYPE IF EXISTS service_type_enum;
DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type where typname = 'service_type_enum') THEN
            CREATE TYPE service_type_enum AS ENUM ('business', 'support', 'data', 'gateway', 'monitor');
        END IF;
    END
$$;

DROP TYPE IF EXISTS service_status_enum;
DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type where typname = 'service_status_enum') THEN
            CREATE TYPE service_status_enum AS ENUM ('enabled', 'disabled', 'maintenance');
        END IF;
    END
$$;

DROP TYPE IF EXISTS api_method_enum;
DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type where typname = 'api_method_enum') THEN
            CREATE TYPE api_method_enum AS ENUM ('GET', 'POST', 'PUT', 'DELETE', 'PATCH', 'HEAD', 'OPTIONS');
        END IF;
    END
$$;

DROP TYPE IF EXISTS api_status_enum;
DO
$$
    BEGIN
        IF NOT EXISTS (SELECT 1 FROM pg_type where typname = 'api_status_enum') THEN
            CREATE TYPE api_status_enum AS ENUM ('published', 'draft', 'deprecated');
        END IF;
    END
$$;


-- 服务表
DROP TABLE IF EXISTS sys_service;
CREATE TABLE IF NOT EXISTS sys_service
(
    id              VARCHAR(50) PRIMARY KEY,
    service_code    VARCHAR(50)         NOT NULL,
    service_name    VARCHAR(100)        NOT NULL,
    service_type    service_type_enum   NOT NULL DEFAULT 'business',
    service_desc    TEXT,
    owner_team      VARCHAR(100),
    base_url        VARCHAR(200),
    status          service_status_enum NOT NULL DEFAULT 'enabled',
    health_endpoint VARCHAR(200),
    is_internal     BOOLEAN                      DEFAULT FALSE,
    modules_count   INT                          DEFAULT 0,
    apis_count      INT                          DEFAULT 0,
    created_at      TIMESTAMP WITH TIME ZONE     DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP WITH TIME ZONE     DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (service_code)
);

COMMENT ON TABLE sys_service IS '微服务定义表';
COMMENT ON COLUMN sys_service.id IS '服务ID';
COMMENT ON COLUMN sys_service.service_code IS '服务编码，唯一标识,如 USER_SERVICE';
COMMENT ON COLUMN sys_service.service_name IS '服务名称,如 用户服务';
COMMENT ON COLUMN sys_service.service_type IS '服务类型：business-业务服务, support-支撑服务, data-数据服务, gateway-网关服务, monitor-监控服务';
COMMENT ON COLUMN sys_service.service_desc IS '服务描述';
COMMENT ON COLUMN sys_service.owner_team IS '负责团队';
COMMENT ON COLUMN sys_service.base_url IS '服务基础URL';
COMMENT ON COLUMN sys_service.status IS '服务状态：enabled-启用, disabled-停用, maintenance-维护中';
COMMENT ON COLUMN sys_service.health_endpoint IS '健康检查端点';
COMMENT ON COLUMN sys_service.is_internal IS '是否内部服务';
COMMENT ON COLUMN sys_service.modules_count IS '模块数量';
COMMENT ON COLUMN sys_service.apis_count IS 'API数量';

CREATE INDEX idx_service_code ON sys_service (service_code);
CREATE INDEX idx_service_type ON sys_service (service_type);
CREATE INDEX idx_service_status ON sys_service (status);
CREATE INDEX idx_is_internal ON sys_service (is_internal);

-- 服务模块表
DROP TABLE IF EXISTS sys_service_module;
CREATE TABLE IF NOT EXISTS sys_service_module
(
    id          VARCHAR(50) PRIMARY KEY,
    service_id  VARCHAR(50)  NOT NULL,
    module_code VARCHAR(50)  NOT NULL,
    module_name VARCHAR(100) NOT NULL,
    module_path VARCHAR(200),
    sort_order  int          not null    default 1,
    module_desc TEXT,
    apis_count  INT                      DEFAULT 0,
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,

    UNIQUE (service_id, module_code)
);

COMMENT ON TABLE sys_service_module IS '服务模块表';
COMMENT ON COLUMN sys_service_module.service_id IS '所属服务ID';
COMMENT ON COLUMN sys_service_module.module_code IS '模块编码，在服务内唯一';
COMMENT ON COLUMN sys_service_module.module_name IS '模块名称';
COMMENT ON COLUMN sys_service_module.module_path IS '模块基础路径';
COMMENT ON COLUMN sys_service_module.sort_order IS '排序';
COMMENT ON COLUMN sys_service_module.module_desc IS '模块描述';
COMMENT ON COLUMN sys_service_module.apis_count IS 'API数量';

CREATE INDEX idx_service_id ON sys_service_module (service_id);
CREATE INDEX idx_module_code ON sys_service_module (module_code);

-- 服务接口表
DROP TABLE IF EXISTS sys_service_api;
CREATE TABLE IF NOT EXISTS sys_service_api
(
    id                  VARCHAR(50) PRIMARY KEY,
    module_id           VARCHAR(50)         NOT NULL,
    api_code            VARCHAR(100) UNIQUE NOT NULL,
    api_path            VARCHAR(200)        NOT NULL,
    api_name            VARCHAR(100)        NOT NULL,
    http_method         api_method_enum     NOT NULL DEFAULT 'GET',
    required_permission VARCHAR(200),
    is_public           BOOLEAN                      DEFAULT FALSE,
    rate_limit          INT                          DEFAULT 0,
    api_desc            TEXT,
    request_example     TEXT,
    response_example    TEXT,
    status              api_status_enum     NOT NULL DEFAULT 'draft',
    deprecated_at       TIMESTAMP WITH TIME ZONE,
    created_at          TIMESTAMP WITH TIME ZONE     DEFAULT CURRENT_TIMESTAMP,
    updated_at          TIMESTAMP WITH TIME ZONE     DEFAULT CURRENT_TIMESTAMP,

    UNIQUE (module_id, http_method, api_path)
);

COMMENT ON TABLE sys_service_api IS 'API接口表';
COMMENT ON COLUMN sys_service_api.module_id IS '所属模块ID';
COMMENT ON COLUMN sys_service_api.api_code IS '接口编码';
COMMENT ON COLUMN sys_service_api.api_path IS 'API路径';
COMMENT ON COLUMN sys_service_api.api_name IS 'API名称';
COMMENT ON COLUMN sys_service_api.http_method IS 'HTTP方法';
COMMENT ON COLUMN sys_service_api.required_permission IS '所需权限标识';
COMMENT ON COLUMN sys_service_api.is_public IS '是否公开接口';
COMMENT ON COLUMN sys_service_api.rate_limit IS '限流值，0表示不限流';
COMMENT ON COLUMN sys_service_api.api_desc IS 'API描述';
COMMENT ON COLUMN sys_service_api.request_example IS '请求示例';
COMMENT ON COLUMN sys_service_api.response_example IS '响应示例';
COMMENT ON COLUMN sys_service_api.status IS 'API状态：published-已发布, draft-草稿, deprecated-已废弃';
COMMENT ON COLUMN sys_service_api.deprecated_at IS '废弃时间';

CREATE INDEX idx_service_api_module_id ON sys_service_api (module_id);
CREATE INDEX idx_service_api_method ON sys_service_api (http_method);
CREATE INDEX idx_service_api_status ON sys_service_api (status);
CREATE INDEX idx_service_api_deprecated_at ON sys_service_api (deprecated_at);


-- 接口资源表
CREATE TABLE sys_api_resources
(
    id            INT PRIMARY KEY,
    service_id    INT                 NOT NULL,
    resource_code VARCHAR(100) UNIQUE NOT NULL,
    resource_name VARCHAR(100)        NOT NULL,
    resource_path VARCHAR(200)        NOT NULL,
    http_method   VARCHAR(10)         NOT NULL,
    resource_type INT                 NOT NULL DEFAULT 1,
    is_public     BOOLEAN                      DEFAULT FALSE,
    status        INT                 NOT NULL DEFAULT 1,
    created_at    TIMESTAMP                    DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMP                    DEFAULT CURRENT_TIMESTAMP,

    UNIQUE (service_id, resource_path, http_method)

);


COMMENT ON TABLE sys_api_resources IS '接口资源表';
COMMENT ON COLUMN sys_api_resources.id IS '资源ID';
COMMENT ON COLUMN sys_api_resources.service_id IS '所属服务ID';
COMMENT ON COLUMN sys_api_resources.resource_code IS '资源编码，格式: 服务编码:接口路径:方法';
COMMENT ON COLUMN sys_api_resources.resource_name IS '资源名称';
COMMENT ON COLUMN sys_api_resources.resource_path IS '接口路径，如 /api/v1/users';
COMMENT ON COLUMN sys_api_resources.http_method IS 'HTTP方法，GET/POST/PUT/DELETE等';
COMMENT ON COLUMN sys_api_resources.resource_type IS '资源类型：1-API，2-菜单，3-按钮';
COMMENT ON COLUMN sys_api_resources.is_public IS '是否公开接口';
COMMENT ON COLUMN sys_api_resources.status IS '状态：1-启用，0-停用';

CREATE INDEX idx_sys_api_resources_service_id ON sys_api_resources (service_id);
CREATE INDEX idx_sys_api_resources_method_path ON sys_api_resources (http_method, resource_path);

-- 创建函数和触发器来更新计数
CREATE OR REPLACE FUNCTION update_service_module_count()
    RETURNS TRIGGER AS
$$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE sys_service
        SET modules_count = modules_count + 1,
            updated_at    = CURRENT_TIMESTAMP
        WHERE id = NEW.service_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE sys_service
        SET modules_count = modules_count - 1,
            updated_at    = CURRENT_TIMESTAMP
        WHERE id = OLD.service_id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER service_modules_count_trigger
    AFTER INSERT OR DELETE
    ON sys_service_module
    FOR EACH ROW
EXECUTE FUNCTION update_service_module_count();

CREATE OR REPLACE FUNCTION update_module_api_count()
    RETURNS TRIGGER AS
$$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE sys_service_module
        SET apis_count = apis_count + 1,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = NEW.module_id;

        UPDATE sys_service s
        SET apis_count = apis_count + 1,
            updated_at = CURRENT_TIMESTAMP
        FROM sys_service_module m
        WHERE s.id = m.service_id
          AND m.id = NEW.module_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE sys_service_module
        SET apis_count = apis_count - 1,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = OLD.module_id;

        UPDATE sys_service s
        SET apis_count = apis_count - 1,
            updated_at = CURRENT_TIMESTAMP
        FROM sys_service_module m
        WHERE s.id = m.service_id
          AND m.id = OLD.module_id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER module_apis_count_trigger
    AFTER INSERT OR DELETE
    ON sys_service_api
    FOR EACH ROW
EXECUTE FUNCTION update_module_api_count();