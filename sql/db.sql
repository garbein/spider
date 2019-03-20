CREATE TABLE `job` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `job_id` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '职位id',
  `company_id` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '公司id',
  `job_name` varchar(255) NOT NULL DEFAULT '' COMMENT '职位',
  `publish_date` varchar(255) NOT NULL DEFAULT '' COMMENT '发布日期',
  `region` varchar(255) NOT NULL DEFAULT '' COMMENT '区域',
  `salary` varchar(255) NOT NULL DEFAULT '' COMMENT '薪水',
  `number` varchar(255) NOT NULL DEFAULT '' COMMENT '人数',
  `years` varchar(255) NOT NULL DEFAULT '' COMMENT '工作年限',
  `education` varchar(255) NOT NULL DEFAULT '' COMMENT '学历',
  `english` varchar(255) NOT NULL DEFAULT '' COMMENT '英语',
  `address` varchar(255) NOT NULL DEFAULT '' COMMENT '地址',
  `info` text NOT NULL COMMENT '工作描述',
  `mtime` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '修改时间',
  `ctime` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '创建时间',
  `deleted` tinyint(4) unsigned NOT NULL DEFAULT '0' COMMENT '删除标记',
  PRIMARY KEY (`id`),
  KEY `idx_job_id` (`job_id`),
  KEY `idx_company_id` (`company_id`),
  KEY `idx_job_name` (`job_name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `company` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `company_id` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '公司id',
  `company_name` varchar(255) NOT NULL DEFAULT '' COMMENT '公司名称',
  `company_type` varchar(255) NOT NULL DEFAULT '' COMMENT '公司类型',
  `people` varchar(255) NOT NULL DEFAULT '' COMMENT '人数',
  `industry` varchar(255) NOT NULL DEFAULT '' COMMENT '行业',
  `address` varchar(255) NOT NULL DEFAULT '' COMMENT '地址',
  `info` text NOT NULL COMMENT '介绍',
  `mtime` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '修改时间',
  `ctime` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '创建时间',
  `deleted` tinyint(4) unsigned NOT NULL DEFAULT '0' COMMENT '删除标记',
  PRIMARY KEY (`id`),
  KEY `idx_company_id` (`company_id`),
  KEY `idx_company_name` (`company_name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `qy_company` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT COMMENT 'ID',
  `company_id` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '外部公司id',
  `qy_id` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '公司id',
  `qy_name` varchar(255) NOT NULL DEFAULT '' COMMENT '公司名称',
  `data_empty` tinyint(4) unsigned NOT NULL DEFAULT '0' COMMENT '数据为空',
  `mtime` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '修改时间',
  `ctime` int(10) unsigned NOT NULL DEFAULT '0' COMMENT '创建时间',
  `deleted` tinyint(4) unsigned NOT NULL DEFAULT '0' COMMENT '删除标记',
  PRIMARY KEY (`id`),
  KEY `idx_company_id` (`company_id`),
  KEY `idx_qy_id` (`qy_id`),
  KEY `idx_qy_name` (`qy_name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;