# Install-to-Prefix 功能文档

## 功能概述

`install-to-prefix` 功能允许用户将 pixi 环境安装到自定义目录，而不是默认的 `.pixi/envs/<env-name>` 位置。这个功能通过 `pixi install --to-prefix` 命令实现，为用户提供了更灵活的环境部署选项。

## 核心实现逻辑

### 1. PrefixOverrideGuard 结构体

**文件位置**: `src/prefix_override.rs`

核心实现基于 RAII (Resource Acquisition Is Initialization) 模式的 `PrefixOverrideGuard` 结构体：

```rust
pub struct PrefixOverrideGuard {
    env_names: Vec<String>,
}
```

**主要功能**:
- 管理线程本地的 prefix 覆盖
- 自动清理：当 guard 被 drop 时，自动移除覆盖设置
- 支持单个和多个环境的覆盖

**关键方法**:
- `new(env_name: String, custom_prefix: PathBuf)`: 创建单环境覆盖
- `new_multiple(overrides: HashMap<String, PathBuf>)`: 创建多环境覆盖
- `get_prefix_override(env_name: &str) -> Option<PathBuf>`: 查询覆盖路径

### 2. 线程本地存储机制

使用 `thread_local!` 宏实现线程安全的 prefix 覆盖存储：

```rust
thread_local! {
    static PREFIX_OVERRIDES: RefCell<HashMap<String, PathBuf>> = RefCell::new(HashMap::new());
}
```

**设计优势**:
- 线程安全：每个线程有独立的覆盖状态
- 无全局状态污染：不影响其他线程的环境解析
- 自动清理：线程结束时自动清理状态

### 3. Environment::dir() 方法集成

**文件位置**: `src/workspace/environment.rs`

修改了 `Environment::dir()` 方法以支持 prefix 覆盖：

```rust
pub fn dir(&self) -> std::path::PathBuf {
    // Check for thread-local prefix override first
    if let Some(override_path) = crate::prefix_override::get_prefix_override(self.name().as_str()) {
        return override_path;
    }
    
    // Fall back to default behavior
    self.workspace.environments_dir().join(self.environment.name.as_str())
}
```

**执行逻辑**:
1. 首先检查是否有线程本地的 prefix 覆盖
2. 如果有覆盖，返回自定义路径
3. 否则返回默认的环境目录路径

### 4. CLI 参数处理

**文件位置**: `src/cli/install.rs`

新增和修改的命令行参数：

```rust
/// Skip installation of specific packages present in the lockfile. Requires --frozen.
/// This can be useful for instance in a Dockerfile to skip local source dependencies when installing dependencies.
#[arg(long, requires = "frozen")]
pub skip: Option<Vec<String>>,

/// Install to a custom prefix directory instead of the default environment location
#[arg(long, value_name = "PREFIX", conflicts_with = "all")]
pub to_prefix: Option<PathBuf>,

/// The platform to install packages for (only used with --to-prefix)
#[arg(long, short = 'p', requires = "to_prefix")]
pub platform: Option<Platform>,
```

**参数约束**:
- `--skip` 需要 `--frozen` 参数（只在冻结模式下跳过特定包）
- `--to-prefix` 与 `--all` 互斥（不能同时安装所有环境到自定义前缀）
- `--platform` 需要 `--to-prefix` 参数（只在自定义前缀安装时指定平台）

## 运行原理

### 1. 安装流程

```rust
pub async fn execute(args: Args) -> miette::Result<()> {
    // 1. 创建自定义前缀目录
    if let Some(prefix_path) = &args.to_prefix {
        tokio::fs::create_dir_all(prefix_path).await?;
    }

    // 2. 确定要安装的环境（自定义前缀时限制为单个环境）
    let envs = if args.to_prefix.is_some() {
        vec![args.environment.and_then(|envs| envs.into_iter().next())
             .unwrap_or_else(|| "default".to_string())]
    } else if let Some(envs) = args.environment {
        envs
    } else if args.all {
        workspace.environments().iter().map(|env| env.name().to_string()).collect()
    } else {
        vec![workspace.default_environment().name().to_string()]
    };

    // 3. 验证平台支持（如果指定了平台）
    if let Some(platform) = args.platform {
        for env in &environments {
            if !env.platforms().contains(&platform) {
                return Err(miette::miette!("Platform not supported"));
            }
        }
    }

    // 4. 创建 PrefixOverrideGuard（支持平台覆盖）
    let _guard = args.to_prefix.as_ref().map(|prefix_path| {
        if let Some(platform) = args.platform {
            PrefixOverrideGuard::new_with_platform(
                environments[0].name().to_string(),
                prefix_path.clone(),
                platform
            )
        } else {
            PrefixOverrideGuard::new(environments[0].name().to_string(), prefix_path.clone())
        }
    });

    // 5. 执行标准安装流程（包含 skip 参数处理）
    get_update_lock_file_and_prefixes(
        &environments,
        UpdateMode::Revalidate,
        UpdateLockFileOptions { ... },
        ReinstallPackages::default(),
        &args.skip.clone().unwrap_or_default(),
    ).await?;
}
```

### 2. 关键设计决策

**单环境限制**: 使用 `--to-prefix` 时只能安装单个环境，这简化了实现并避免了路径冲突。

**平台覆盖**: 支持通过 `--platform` 参数覆盖环境的默认平台选择，实现跨平台安装。

**包跳过机制**: `--skip` 参数允许在冻结模式下跳过特定包的安装，适用于容器化部署场景。

**RAII 模式**: 使用 `_guard` 变量确保在函数结束时自动清理覆盖状态。

**透明集成**: 现有的安装逻辑无需修改，通过 `Environment::dir()` 和 `Environment::best_platform()` 方法透明地使用自定义路径和平台。

### 3. 错误处理

- 目录创建失败时提供清晰的错误信息
- 参数验证确保不兼容的选项组合被拒绝
- 使用 `miette` 提供用户友好的错误报告

## 兼容性和CI更改

### 1. 文档更改

**CLI 文档更新** (`docs/reference/cli/pixi/install.md`):

- 添加了 `--to-prefix` 和 `--platform` 参数的文档
- 添加了 `--skip` 参数的文档和使用场景
- 更新了使用示例和描述

**第三方工具文档** (`docs/integration/extensions/pixi_install_to_prefix.md`):
- 记录了外部 `pixi-install-to-prefix` 工具
- 提供了安装和使用指南

### 2. 测试基础设施

**集成测试**:
- 现有测试继续使用默认行为
- 没有发现专门针对 `--to-prefix` 功能的集成测试
- 测试主要依赖于现有的 prefix 管理逻辑

**测试覆盖范围**:
- `PrefixOverrideGuard` 的基本功能
- 线程本地存储的正确性
- 与现有安装流程的集成

### 3. CI 工作流更改

**构建和测试**:
- CI 配置没有因为这个功能而发生重大变化
- 使用标准的 Rust 测试流程
- 依赖现有的集成测试覆盖

**发布流程**:
- 功能作为标准发布的一部分包含
- 没有特殊的发布要求或步骤

## 使用示例

### 基本用法

```bash
# 安装默认环境到自定义目录
pixi install --to-prefix /path/to/custom/env

# 安装特定环境到自定义目录
pixi install --environment myenv --to-prefix /path/to/custom/env

# 为特定平台安装到自定义目录
pixi install --to-prefix /path/to/custom/env --platform linux-64

# 跳过特定包的安装（需要 --frozen 模式）
pixi install --frozen --skip package1,package2

# 组合使用：自定义前缀 + 跳过特定包
pixi install --frozen --to-prefix /path/to/custom/env --skip local-package
```

### 与第三方工具的关系

虽然 pixi 现在内置了 `--to-prefix` 功能，但仍然存在第三方工具 `pixi-install-to-prefix`，它提供了额外的功能：

- 激活脚本生成
- 更多的自定义选项
- 独立的工具链

## 技术特点

### 优势

1. **线程安全**: 使用线程本地存储避免竞态条件
2. **RAII 模式**: 自动资源管理，防止状态泄漏
3. **透明集成**: 与现有代码无缝集成
4. **错误处理**: 提供清晰的错误信息和验证

### 限制

1. **单环境限制**: 一次只能安装一个环境到自定义前缀
2. **平台依赖**: 需要明确指定目标平台（使用 `--platform`）
3. **路径管理**: 用户需要手动管理自定义前缀目录
4. **跳过包限制**: `--skip` 参数只能在冻结模式（`--frozen`）下使用
5. **参数组合**: 某些参数组合有限制（如 `--to-prefix` 与 `--all` 互斥）

## 总结

`install-to-prefix` 功能通过巧妙的线程本地存储和 RAII 模式实现，为 pixi 用户提供了灵活的环境部署选项。该实现包含了以下主要功能：

1. **自定义前缀安装**: 通过 `--to-prefix` 参数支持将环境安装到指定目录
2. **平台覆盖**: 通过 `--platform` 参数支持跨平台安装
3. **包跳过机制**: 通过 `--skip` 参数支持在容器化场景中跳过特定包

实现保持了与现有代码的兼容性，同时提供了清晰的 API 和错误处理。功能与上游的 `--skip` 参数完美集成，解决了文档冲突问题。这个功能主要是核心逻辑实现，兼容性和 CI 更改相对较少，主要集中在文档更新和测试覆盖上。
