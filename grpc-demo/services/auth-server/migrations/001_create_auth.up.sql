-- Add migration script here
DROP TYPE IF EXISTS user_status_enum;
DO
$$
    BEGIN
        IF
            NOT EXISTS (SELECT 1 FROM pg_type where typname = 'user_status_enum') THEN
            CREATE TYPE user_status_enum as ENUM ('activate', 'deactivate', 'suspended', 'locked', 'pending', 'deleted');
        END IF;

    end
$$;


-- Create sys_user table
DROP TABLE IF EXISTS sys_user;

CREATE TABLE IF NOT EXISTS sys_user
(
    -- 主键
    id                   UUID PRIMARY KEY          DEFAULT gen_random_uuid(),

    -- 基本信息
    username             VARCHAR(50)      NOT NULL,
    email                VARCHAR(100)     NOT NULL UNIQUE,
    phone                VARCHAR(20),
    password_hash        VARCHAR(255)     NOT NULL,
    display_name         VARCHAR(100)     NOT NULL,
    avatar_url           VARCHAR(500),

    -- 验证状态
    email_verified       BOOLEAN          NOT NULL DEFAULT FALSE,
    phone_verified       BOOLEAN          NOT NULL DEFAULT FALSE,

    -- 用户状态
    status               user_status_enum NOT NULL DEFAULT 'pending',

    -- 登录相关
    last_login_at        TIMESTAMPTZ,
    login_count          INTEGER          NOT NULL DEFAULT 0,
    failed_login_count   INTEGER          NOT NULL DEFAULT 0,
    last_failed_login_at TIMESTAMPTZ,
    is_first_login       BOOLEAN          NOT NULL DEFAULT TRUE,
    last_activity_at     TIMESTAMPTZ,

    -- 账户安全
    locked_until         TIMESTAMPTZ,
    locked_at            TIMESTAMPTZ,
    lock_reason          VARCHAR(255),
    password_changed_at  TIMESTAMPTZ,
    password_expires_at  TIMESTAMPTZ,

    -- 角色和权限
    roles                JSONB            NOT NULL DEFAULT '[]'::jsonb,
    permissions          JSONB            NOT NULL DEFAULT '[]'::jsonb,

    -- 个人偏好
    timezone             VARCHAR(50),
    language             VARCHAR(10),

    -- 元数据
    metadata             JSONB,

    -- 时间戳
    created_at           TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    deleted_at           TIMESTAMPTZ
);

-- 添加注释
COMMENT ON TABLE sys_user IS '用户表';
COMMENT ON COLUMN sys_user.id IS '用户ID';
COMMENT ON COLUMN sys_user.username IS '用户名';
COMMENT ON COLUMN sys_user.email IS '邮箱';
COMMENT ON COLUMN sys_user.phone IS '手机号';
COMMENT ON COLUMN sys_user.password_hash IS '密码哈希';
COMMENT ON COLUMN sys_user.display_name IS '显示名称';
COMMENT ON COLUMN sys_user.avatar_url IS '头像URL';
COMMENT ON COLUMN sys_user.email_verified IS '邮箱是否已验证';
COMMENT ON COLUMN sys_user.phone_verified IS '手机号是否已验证';
COMMENT ON COLUMN sys_user.status IS '用户状态';
COMMENT ON COLUMN sys_user.last_login_at IS '最后登录时间';
COMMENT ON COLUMN sys_user.login_count IS '登录次数';
COMMENT ON COLUMN sys_user.failed_login_count IS '失败登录次数';
COMMENT ON COLUMN sys_user.last_failed_login_at IS '最后失败登录时间';
COMMENT ON COLUMN sys_user.locked_at IS '账户锁定时间';
COMMENT ON COLUMN sys_user.locked_until IS '锁定到期时间';
COMMENT ON COLUMN sys_user.lock_reason IS '账户锁定原因';
COMMENT ON COLUMN sys_user.password_changed_at IS '密码最后修改时间';
COMMENT ON COLUMN sys_user.password_expires_at IS '密码过期时间';
COMMENT ON COLUMN sys_user.is_first_login IS '是否首次登录';
COMMENT ON COLUMN sys_user.last_activity_at IS '上次活动时间';
COMMENT ON COLUMN sys_user.timezone IS '时区';
COMMENT ON COLUMN sys_user.language IS '语言';
COMMENT ON COLUMN sys_user.roles IS '角色列表';
COMMENT ON COLUMN sys_user.permissions IS '权限列表';
COMMENT ON COLUMN sys_user.metadata IS '元数据';
COMMENT ON COLUMN sys_user.created_at IS '创建时间';
COMMENT ON COLUMN sys_user.updated_at IS '更新时间';
COMMENT ON COLUMN sys_user.deleted_at IS '软删除时间';

-- 创建索引
CREATE INDEX idx_sys_user_email ON sys_user (email);
CREATE INDEX idx_sys_user_username ON sys_user (username);
CREATE INDEX idx_sys_user_status ON sys_user (status);
CREATE INDEX idx_sys_user_deleted_at ON sys_user (deleted_at);



-- 创建 updated_at 触发器（如果不存在）
CREATE OR REPLACE FUNCTION trg_set_timestamp()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at := NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';


DROP TRIGGER IF EXISTS set_sys_user_updated_at ON sys_user;

-- Create trigger for sys_user table
CREATE TRIGGER set_sys_user_updated_at
    BEFORE UPDATE
    ON sys_user
    FOR EACH ROW
EXECUTE FUNCTION trg_set_timestamp();



do
$$
    BEGIN
        IF
            NOT EXISTS (SELECT 1 FROM pg_type where typname = 'role_status_enum') THEN
            CREATE TYPE role_status_enum as ENUM ('active', 'inactive','banned');
        END IF;

    end
$$;

-- 角色表
DROP TABLE IF EXISTS sys_role;
CREATE TABLE IF NOT EXISTS sys_role
(
    id           VARCHAR(36) PRIMARY KEY   DEFAULT gen_random_uuid(),
    role_code    VARCHAR(255)     NOT NULL UNIQUE,
    role_name    VARCHAR(255)     NOT NULL,
    role_status  role_status_enum NOT NULL DEFAULT 'active',
    role_type    INT              NOT NULL DEFAULT 1,
    order_num    int              not null default 1,
    remark       VARCHAR(255),
    role_desc    VARCHAR(255),
    is_default   boolean          not null default false,
    is_protected boolean          not null default false,
    created_at   TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    created_id   VARCHAR(255)     not null DEFAULT '1',
    created_by   VARCHAR(255)     NOT NULL DEFAULT 'system',
    updated_at   TIMESTAMPTZ      NOT NULL DEFAULT NOW(),
    updated_id   VARCHAR(255)     not null DEFAULT '1',
    updated_by   VARCHAR(255)     NOT NULL DEFAULT 'system',
    is_deleted   boolean          not null default false,
    deleted_at   TIMESTAMP
);

comment on TABLE sys_role is '角色表';
comment on COLUMN sys_role.id is '角色ID';
comment on COLUMN sys_role.role_name is '角色名称';
comment on COLUMN sys_role.role_code is '角色编码，如 ADMIN, USER, SUPPORT';
comment on COLUMN sys_role.role_type is '角色类型：1-系统角色，2-业务角色，3-自定义角色';
comment on COLUMN sys_role.role_desc is '角色描述';
comment on COLUMN sys_role.role_status is '角色状态';
comment on COLUMN sys_role.is_default is '是否默认角色';
comment on COLUMN sys_role.is_protected is '是否保护角色';
comment on COLUMN sys_role.is_deleted is '是否删除';

-- 权限表
DROP TABLE IF EXISTS sys_permission;
CREATE TABLE IF NOT EXISTS sys_permission
(
    id              VARCHAR(36) PRIMARY KEY DEFAULT gen_random_uuid(),
    permission_code VARCHAR(255) NOT NULL UNIQUE,
    permission_name VARCHAR(255) NOT NULL,
    permission_type INT          NOT NULL   DEFAULT 1,
    order_num       int          not null   default 1,
    remark          VARCHAR(255),
    permission_desc VARCHAR(255),
    created_at      TIMESTAMPTZ  NOT NULL   DEFAULT NOW(),
    created_id      VARCHAR(255) not null   DEFAULT '1',
    created_by      VARCHAR(255) NOT NULL   DEFAULT 'system',
    updated_at      TIMESTAMPTZ  NOT NULL   DEFAULT NOW(),
    updated_id      VARCHAR(255) not null   DEFAULT '1',
    updated_by      VARCHAR(255) NOT NULL   DEFAULT 'system',
    is_deleted      boolean      not null   default false,
    deleted_at      TIMESTAMP
);

comment on TABLE sys_permission is '权限表';
comment on COLUMN sys_permission.id is '权限ID';
comment on COLUMN sys_permission.permission_name is '权限名称';
comment on COLUMN sys_permission.permission_code is '权限编码，如 ADMIN, USER, SUPPORT';
comment on COLUMN sys_permission.permission_type is '权限类型：1-菜单权限，2-操作权限，3-数据权限， 4-API权限';
comment on COLUMN sys_permission.permission_desc is '权限描述';
comment on COLUMN sys_permission.remark is '权限备注';
comment on COLUMN sys_permission.is_deleted is '是否删除';

-- 用户-角色关联表
DROP TABLE IF EXISTS sys_user_role;
CREATE TABLE IF NOT EXISTS sys_user_role
(
    id             VARCHAR(36) PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id        VARCHAR(36)  NOT NULL,
    role_id        VARCHAR(36)  NOT NULL,
    source         INT                     DEFAULT 1 NOT NULL,
    effective_from TIMESTAMPTZ  NULL,
    effective_to   TIMESTAMPTZ  NULL,
    created_at     TIMESTAMPTZ  NOT NULL   DEFAULT NOW(),
    created_id     VARCHAR(255) not null   DEFAULT '1',
    created_by     VARCHAR(255) NOT NULL   DEFAULT 'system',
    updated_at     TIMESTAMPTZ  NOT NULL   DEFAULT NOW(),
    updated_id     VARCHAR(255) not null   DEFAULT '1',
    updated_by     VARCHAR(255) NOT NULL   DEFAULT 'system',
    is_deleted     boolean      not null   default false,
    deleted_at     TIMESTAMPTZ,
    UNIQUE (user_id, role_id)
);

comment on TABLE sys_user_role is '用户角色表';
comment on COLUMN sys_user_role.id is '用户角色ID';
comment on COLUMN sys_user_role.user_id is '用户ID';
comment on COLUMN sys_user_role.role_id is '角色ID';
comment on COLUMN sys_user_role.source is '来源：1-手动分配，2-自动分配，3-继承';
comment on COLUMN sys_user_role.effective_from is '生效时间';
comment on COLUMN sys_user_role.effective_to is '失效时间';
comment on COLUMN sys_user_role.created_at is '创建时间';
comment on COLUMN sys_user_role.created_id is '创建人ID';
comment on COLUMN sys_user_role.created_by is '创建人';

-- 创建索引
CREATE INDEX idx_user_id ON sys_user_role (user_id);
CREATE INDEX idx_role_id ON sys_user_role (role_id);


-- Create refresh_tokens table
DROP TABLE IF EXISTS refresh_tokens;
CREATE TABLE IF NOT EXISTS refresh_tokens
(
    jti        VARCHAR(36) PRIMARY KEY,
    user_id    VARCHAR(36) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked    BOOLEAN     NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_refresh_user ON refresh_tokens (user_id);

-- Create email_verifications table
DROP TABLE IF EXISTS email_verifications;
CREATE TABLE IF NOT EXISTS email_verifications
(
    user_id    VARCHAR(36) PRIMARY KEY,
    token      VARCHAR(36) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Create password_resets table
DROP TABLE IF EXISTS password_resets;
CREATE TABLE IF NOT EXISTS password_resets
(
    user_id    VARCHAR(36) PRIMARY KEY,
    token      VARCHAR(36) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);