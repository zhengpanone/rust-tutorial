-- 服务表
CREATE TABLE `services` (
  `id` VARCHAR(50) PRIMARY KEY COMMENT '服务ID',
  `service_code` VARCHAR(50) UNIQUE NOT NULL COMMENT '服务编码，如 USER_SERVICE',
  `service_name` VARCHAR(100) NOT NULL COMMENT '服务名称，如 用户服务',
  `service_type` TINYINT NOT NULL DEFAULT 1 COMMENT '服务类型：1-业务服务，2-支撑服务，3-数据服务',
  `service_desc` VARCHAR(500) COMMENT '服务描述',
  `owner_team` VARCHAR(100) COMMENT '负责团队',
  `base_url` VARCHAR(200) COMMENT '服务基础URL',
  `status` TINYINT NOT NULL DEFAULT 1 COMMENT '状态：1-启用，0-停用',
  `health_endpoint` VARCHAR(200) COMMENT '健康检查端点',
  `is_internal` BOOLEAN DEFAULT FALSE COMMENT '是否内部服务',
  `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  INDEX `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='微服务定义表';