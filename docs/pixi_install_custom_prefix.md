# pixi install 自定义前缀功能实现与合并指南

## 概述
此次改动在 `pixi install` 命令中引入 `--to-prefix` 与 `--platform`，支持将环境安装到自定义目录并覆盖求解平台。核心逻辑通过线程本地覆写表与 RAII guard 实现，确保跨模块共享自定义前缀时不会影响其他调用。

## 核心原理

- **线程本地覆写**：`PrefixOverrideGuard` 将环境名称映射到线程本地的路径/平台表中，Drop 时自动清理，保证覆写作用范围仅限当前安装流程（`crates/pixi_core/src/prefix_override.rs:6`、`crates/pixi_core/src/prefix_override.rs:48`）。
- **环境定位与平台解析**：`Environment::dir` 与 `Environment::best_platform` 在访问工作区默认目录或平台前，优先读取覆写表，从而统一 CLI 与核心逻辑的前缀选择（`crates/pixi_core/src/workspace/environment.rs:98`、`crates/pixi_core/src/workspace/environment.rs:130`）。
- **命令参数绑定**：CLI 在解析参数后会创建目标目录、校验环境支持的平台，并在执行期间持有 `PrefixOverrideGuard` 保证安装过程生效；同时更新安装结果输出，提示目标前缀与被排除的包（`crates/pixi_cli/src/install.rs:64`、`crates/pixi_cli/src/install.rs:150`、`crates/pixi_cli/src/install.rs:275`）。
- **模块导出**：`pixi_core` 对外公开新覆写模块并保留 `prefix` 占位，以便未来扩展前缀工具函数（`crates/pixi_core/src/lib.rs:8`、`crates/pixi_core/src/prefix.rs:1`）。

## 关键实现细节

- CLI 参数：
  - `--to-prefix <PREFIX>` 与 `--platform (-p) <PLATFORM>` 的定义与冲突约束（`crates/pixi_cli/src/install.rs:64`）。
  - 当 `--to-prefix` 存在时仅允许处理单一环境，默认回退到 `default` 环境（`crates/pixi_cli/src/install.rs:108`）。
  - 安装前创建目标目录并通过 `miette` 报告失败原因（`crates/pixi_cli/src/install.rs:91`）。
  - 平台校验循环确保自定义平台位于环境受支持集合内（`crates/pixi_cli/src/install.rs:132`）。
- 覆写守卫：
  - 线程本地 HashMap 保存环境->路径/平台映射（`crates/pixi_core/src/prefix_override.rs:6`）。
  - `PrefixOverrideGuard::new[_with_platform]` 插入映射并返回 RAII guard（`crates/pixi_core/src/prefix_override.rs:17`、`crates/pixi_core/src/prefix_override.rs:30`）。
  - Drop 时移除所有受管环境的覆写（`crates/pixi_core/src/prefix_override.rs:48`）。
  - 提供查询函数供核心模块读取覆写值（`crates/pixi_core/src/prefix_override.rs:67`、`crates/pixi_core/src/prefix_override.rs:72`）。
- 环境逻辑：
  - `dir()` 若有覆写直接返回覆写路径，否则走原有 `environments_dir()` 拼接（`crates/pixi_core/src/workspace/environment.rs:98`）。
  - `best_platform()` 若有覆写直接返回，并保持原有回退与告警逻辑不变（`crates/pixi_core/src/workspace/environment.rs:130`）。
- 其他同步：
  - 文档新增参数描述，提醒 `--to-prefix` 用法（`docs/reference/cli/pixi/install.md:29`）。
  - 测试构建器默认参数补齐新字段，防止初始化缺项（`tests/integration_rust/common/mod.rs:559`）。

## 合并冲突指引

1. **CLI 参数区块**：若与其他 PR 同时修改 `Args` 结构体，确保序列化顺序与 Clap 约束一致；在冲突时保留自定义前缀相关字段，并确认 `#[arg(conflicts_with)]`、`#[arg(requires)]` 语义没有丢失。
2. **安装流程逻辑**：`execute` 函数内部可能因新增功能或日志变更产生冲突，处理时按顺序保留：
   - 目录创建与 `PrefixOverrideGuard` 初始化；
   - 平台校验循环；
   - 输出消息中前缀提示与跳过包整合。
3. **核心模块导出**：`crates/pixi_core/src/lib.rs` 新增 `mod prefix`、`pub mod prefix_override`，若有其他模块导出调整，请手动合并，保证 `prefix_override` 被公开。
4. **环境方法**：`Environment::dir` 与 `Environment::best_platform` 均添加覆写检查，合并时注意不要丢失旧逻辑的回退与一次性告警。
5. **自动生成文档**：`docs/reference/cli/pixi/install.md` 为自动生成文件，冲突时以最新 CLI help 输出为准，确保 `--to-prefix` 与 `--platform` 条目存在；必要时重新生成文档。
6. **测试默认参数**：`tests/integration_rust/common/mod.rs` 的 `Args` 初始化需包含新字段，解决冲突时确认 `to_prefix`、`platform` 均设置为 `None`。

## 建议验证

- `pixi run test-fast`：快速确认 CLI 改动未破坏现有流程。
- 针对 `--to-prefix` 与 `--platform` 的新增集成测试或手动验证，确保证线程本地覆写在多次命令执行后被正确清理。

## 相关文件索引

- CLI 主逻辑：`crates/pixi_cli/src/install.rs`
- 覆写实现：`crates/pixi_core/src/prefix_override.rs`
- 环境适配：`crates/pixi_core/src/workspace/environment.rs`
- 模块导出：`crates/pixi_core/src/lib.rs`
- 文档更新：`docs/reference/cli/pixi/install.md`
- 测试参数同步：`tests/integration_rust/common/mod.rs`
