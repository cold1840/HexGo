[English](README.md) | **简体中文**

> 英文版是项目的规范基准。如果翻译存在差异，以英文版为准。

# HexGo

HexGo 是一种在正六边形铺砖顶点上进行的新型围棋类游戏。普通内部交点只有三个邻居，而标准围棋通常有四个，因此棋子的连接、气、提子、死活和领地都会呈现不同的拓扑特征。

项目目前处于初始开发阶段。游戏规则已经确定，可玩程序当前只会启动一个最小的 Bevy 应用。

## 核心规则摘要

- 黑白双方轮流在有边连接的棋盘交点落子，黑方先行。
- 相连的同色棋子组成棋串，与棋串相邻的空点是它的气。
- 没有气的棋串会被提掉。
- 完成对方提子后仍然无气的自杀落子属于非法落子。
- 全局同形禁着禁止落子后重现任何历史棋盘局面。
- Pass 永远合法；连续两次 Pass 结束对局。
- 面积计分统计存活棋子和己方完全包围的空点，白方另加可配置贴目。

请阅读完整的[英文规则](docs/RULES.md)或[简体中文规则](docs/RULES.zh-CN.md)。

## 技术与状态

- 语言：Rust 2024 edition
- 游戏引擎：Bevy 0.19.1
- 工具链：stable Rust、`rustfmt` 和 Clippy
- 许可证：GNU General Public License v3.0

当前开发优先级见[路线图](docs/ROADMAP.zh-CN.md)，预期的子系统边界见[架构文档](docs/ARCHITECTURE.zh-CN.md)。

## 开始使用

通过 `rustup` 安装 [Rust](https://www.rust-lang.org/tools/install)。仓库中的 `rust-toolchain.toml` 会自动选择所需的 stable 工具链及组件。

在 Linux 上，Bevy 可能需要音频、输入、Wayland 和 X11 的原生开发包。经过 Ubuntu CI 验证的软件包列表可参考 `.github/workflows/ci.yml`。

克隆仓库、创建工作分支并运行应用：

```bash
git clone https://github.com/cold1840/HexGo.git
cd HexGo
git switch -c feat/your-change
cargo run
```

禁止直接向 `main` 提交。

## 质量检查

创建 PR 前运行与 CI 相同的检查：

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --all-features
cargo test --all-features
```

项目还通过 [pre-commit](https://pre-commit.com/) 配置了可选的本地 hooks：

```bash
pre-commit install
pre-commit run --all-files
```

## 文档

- [游戏规则](docs/RULES.zh-CN.md) ([English](docs/RULES.md))
- [架构](docs/ARCHITECTURE.zh-CN.md) ([English](docs/ARCHITECTURE.md))
- [路线图](docs/ROADMAP.zh-CN.md) ([English](docs/ROADMAP.md))
- [贡献指南](CONTRIBUTING.zh-CN.md) ([English](CONTRIBUTING.md))
- [行为准则](CODE_OF_CONDUCT.zh-CN.md) ([English](CODE_OF_CONDUCT.md))
- [安全策略](SECURITY.zh-CN.md) ([English](SECURITY.md))
- [分支保护配置](docs/BRANCH_PROTECTION.zh-CN.md) ([English](docs/BRANCH_PROTECTION.md))

AI 编程代理还必须遵守英文 [AGENTS.md](AGENTS.md)。

## 参与贡献

项目欢迎人类独立或借助 AI 完成的贡献。所有改动都必须在非 `main` 分支完成，并通过经过审查的 PR 合并。开始前请阅读[贡献指南](CONTRIBUTING.zh-CN.md)。

## 许可证

HexGo 使用 [GNU General Public License v3.0](LICENSE) 许可。
