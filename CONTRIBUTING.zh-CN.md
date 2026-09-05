[English](CONTRIBUTING.md) | **简体中文**

> 英文版是规范基准。如果翻译存在差异，以英文版为准。

# 为 HexGo 贡献

感谢你帮助建设 HexGo。项目欢迎人类独立完成或借助 AI 完成的贡献，两者都必须遵守相同的正确性、审查和责任标准。

## 开始之前

1. 阅读[规则](docs/RULES.zh-CN.md)、[架构](docs/ARCHITECTURE.zh-CN.md)和[路线图](docs/ROADMAP.zh-CN.md)。
2. 检查现有 Issue 和 PR，避免重复工作。
3. 对于大型功能、规则变更、新依赖或架构调整，请在实现前创建提案。
4. 确认工作区没有无关改动：

```bash
git status --short --branch
```

## 分支流程

禁止直接向 `main` 提交。编辑被 Git 跟踪的文件之前，应从最新的 `main` 创建分支：

```bash
git switch main
git pull --ff-only
git switch -c docs/short-description
```

分支前缀如下，后续名称使用小写 kebab-case：

- `feat/`：新功能
- `fix/`：错误修复
- `docs/`：文档
- `refactor/`：不改变行为的重构
- `test/`：测试
- `chore/`：维护工作

每个分支和 PR 应只聚焦一个主题。

## 开发标准

- 使用 stable Rust、Rust 2024 edition、标准 `rustfmt` 和惯用 Rust 命名。
- 领域规则必须独立于 Bevy 和其他展示层逻辑。
- 源代码、标识符、代码注释、Rust 文档注释、提交信息、日志和技术诊断统一使用英语。
- 为新行为和回归问题添加测试。规则行为应当确定，并且无需打开渲染窗口即可测试。
- 未经事先同意，不得添加依赖、引入 `unsafe` 或改变公共行为。
- 不得手动编辑 `Cargo.lock`。

注释应解释意图、不变量或不明显的设计决定，不要复述代码本身。

## 规则与翻译

- [英文规则](docs/RULES.md)是规范基准，[中文规则](docs/RULES.zh-CN.md)是官方翻译。
- 不得顺带修改游戏规则。规则变更需要独立提案并获得维护者明确批准。
- 同一个 PR 必须同步更新两份规则，并保持章节顺序、示例、公式和术语一致。
- 其他成对的英文与 `.zh-CN.md` 文档也必须同步更新。
- 保留每份双语文档顶部的语言切换链接。

如果两个语言版本不一致，请在 PR 中报告，不要自行重新解释规则。

## 本地验证

创建代码 PR 前运行完整质量检查：

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --all-features
cargo test --all-features
```

仅修改文档时还应运行：

```bash
git diff --check
```

检查所有改动过的相对链接，并确认成对翻译仍然同步。可通过 `pre-commit install` 安装可选的 pre-commit hooks。

## 提交

提交信息使用英语和 Conventional Commit 风格：

```text
feat(board): generate the playable vertex graph
fix(rules): reject repeated board positions
docs(rules): clarify boundary liberties
```

标题使用祈使语气并保持简洁。不要在一个提交中混入无关改动，也不要使用 `--no-verify` 绕过 hooks。

## Pull Request

- 使用适合 squash merge 的 Conventional Commit 风格英文标题。
- 说明问题、解决方式、用户可见行为以及对规则或架构的影响。
- 关联相关 Issue；可见 UI 变更需要提供截图。
- 列出实际运行的验证命令以及未运行的检查。
- 确认相关英文和中文文档已同步更新。
- 解决 review conversation 并获得至少一名人类审查者批准。
- 等待全部必需 CI 检查通过后再合并。

### AI 辅助贡献

允许使用 AI，但 PR 必须披露：

- 使用了哪些工具或代理，以及它们完成了什么工作；
- 人类贡献者审查或修改了哪些内容；
- 运行了哪些测试和检查；
- 尚存的不确定性或风险。

除非维护者要求，否则不必在每个 commit 中添加 AI 标记。提交 PR 的人类贡献者对正确性、安全性、许可证和是否遵守本指南负责。

## 报告问题

- 可复现缺陷使用 Bug 报告表单。
- 新行为或规则讨论使用功能提案表单。
- 安全漏洞遵循 [SECURITY.zh-CN.md](SECURITY.zh-CN.md)，禁止在 Issue 中公开敏感细节。
- 所有项目空间均须遵守 [CODE_OF_CONDUCT.zh-CN.md](CODE_OF_CONDUCT.zh-CN.md)。
