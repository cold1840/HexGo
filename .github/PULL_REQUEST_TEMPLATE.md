# Summary / 摘要

<!-- Explain what changed and why. / 说明改动内容和原因。 -->

## Related issue / 相关 Issue

<!-- Use "Closes #123" when appropriate. / 适用时填写 "Closes #123"。 -->

## Rule and architecture impact / 规则与架构影响

- [ ] No game-rule change / 不涉及游戏规则变更
- [ ] Rule change was explicitly approved and both rule documents were updated / 规则变更已明确批准，并同步更新两份规则文档
- [ ] Architecture documentation was updated where needed / 已按需更新架构文档

Describe any impact:

<!-- Describe behavior, compatibility, or architectural impact. / 说明行为、兼容性或架构影响。 -->

## Validation / 验证

<!-- List exact commands and manual checks actually run. Do not mark unperformed checks as complete. / 列出实际运行的命令和人工检查，不要勾选未执行的检查。 -->

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo build --all-features`
- [ ] `cargo test --all-features`
- [ ] Relevant manual checks / 相关人工检查

## Documentation and translations / 文档与翻译

- [ ] No documentation update was needed / 无需更新文档
- [ ] English documentation was updated / 已更新英文文档
- [ ] Matching Simplified Chinese documentation was updated / 已同步更新简体中文文档
- [ ] Changed relative links were checked / 已检查改动涉及的相对链接

## AI assistance / AI 辅助

- [ ] No AI assistance was used / 未使用 AI
- [ ] AI assistance was used and disclosed below / 使用了 AI，并已在下方披露

Tools and scope / 工具及工作范围:

Human review and changes / 人工审查与修改:

Remaining uncertainty or risk / 尚存的不确定性或风险:

## Final checklist / 最终检查

- [ ] This pull request targets a non-`main` working branch and does not bypass hooks / 本 PR 使用非 `main` 工作分支且未绕过 hooks
- [ ] The change is focused and contains no unrelated edits / 改动聚焦且不包含无关内容
- [ ] No secrets or private data are included / 不包含密钥或隐私数据
- [ ] I accept responsibility for the submitted change / 我对提交的改动负责
