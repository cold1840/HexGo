[English](BRANCH_PROTECTION.md) | **简体中文**

> 英文版是规范基准。如果翻译存在差异，以英文版为准。

# 保护 `main` 分支

仓库文档会要求贡献者和 AI 代理不得直接在 `main` 工作，但文档无法从技术上阻止推送。仓库管理员必须配置 GitHub ruleset 或 branch protection rule。

## 推荐的 GitHub Ruleset

在仓库 **Settings → Rules → Rulesets** 中创建一个启用状态的分支 ruleset，并将目标设置为默认分支 `main`。

启用以下保护：

- 限制删除分支；
- 禁止 force push；
- 合并前必须创建 PR；
- 至少需要 1 个批准；
- 推送新提交后撤销旧的 PR 批准；
- 合并前必须解决 review conversation；
- 必须通过状态检查；
- 合并前分支必须保持最新；
- 将当前名为 `Check` 的 CI job 设为 required status check；
- 除非另有紧急流程文档，否则不要向管理员、集成或范围过大的团队授予绕过权限。

如果仓库界面同时显示 workflow 和 job，请在 CI 至少运行一次后，选择 `CI` workflow 中 `Check` job 产生的检查。

## 合并设置

在 **Settings → General → Pull Requests** 中：

- 启用 squash merge；
- 除非维护者有意支持，否则关闭 merge commit 和 rebase merge；
- 要求英文 Conventional Commit 风格的 PR 标题，因为 squash commit 会由该标题生成；
- 如果符合团队分支保留需要，启用合并后自动删除分支。

推荐标题示例：

```text
feat(board): render playable vertices
fix(rules): preserve state after illegal moves
docs(contributing): clarify AI disclosure
```

## 安全报告

在 **Settings → Security → Private vulnerability reporting** 中启用私密漏洞报告，使贡献者可以遵循 `SECURITY.zh-CN.md`，而不会公开漏洞。

## 验证

保存 ruleset 后，使用临时分支和 PR 确认：

1. 直接向 `main` 推送会被拒绝；
2. 没有一个批准时无法合并 PR；
3. 新提交会撤销已有批准；
4. 未解决的 review conversation 会阻止合并；
5. `Check` 状态失败或等待时会阻止合并；
6. Force push 和删除分支会被拒绝；
7. 管理员无法通过正常合并界面绕过规则。

任何有意保留的例外都必须同时记录在本文档和仓库 ruleset 描述中。

## 紧急变更

紧急修复仍然使用短期 `fix/` 分支和 PR。如果 GitHub 不可用或重大事故导致正常流程无法执行，维护者只有在另一名维护者批准后才能临时调整保护，并且必须立即恢复。应记录例外并进行事后审查。
