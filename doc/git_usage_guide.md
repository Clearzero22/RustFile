# Git 使用指南

在软件开发中使用 Git 进行版本控制是一个标准实践。它能帮助你跟踪代码变更、协作开发、管理不同的功能分支以及回滚错误的修改。下面是一个典型的 Git 工作流程，适用于个人或团队开发。

## 1. 基本设置

确保你已经安装了 Git 并进行了基本配置：

```bash
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

## 2. 克隆或初始化仓库

- **如果你是第一次创建项目**，在项目根目录下初始化 Git 仓库：
  ```bash
  git init
  ```

- **如果你要参与一个已有的项目**，克隆远程仓库到本地：
  ```bash
  git clone <repository-url>
  ```

## 3. 日常开发流程

Git 工作区通常包含三个部分：

1.  **工作区 (Working Directory)**：你当前编辑的文件。
2.  **暂存区 (Staging Area / Index)**：你标记为下次提交的文件快照。
3.  **本地仓库 (Repository)**：Git 保存的项目历史版本。

### 开发步骤：

1.  **检查状态**：查看哪些文件被修改了，哪些已暂存。
    ```bash
    git status
    ```

2.  **拉取最新代码**：在开始编码前，确保你的本地分支是基于远程最新的代码。
    ```bash
    git pull origin <branch-name>  # 通常是 main 或 master
    ```

3.  **创建功能分支**（推荐）：为新功能或 bug 修复创建一个独立的分支，这样可以保持主分支的稳定性。
    ```bash
    git checkout -b feature/new-awesome-feature
    # 或者使用新语法 (Git 2.23+)
    git switch -c feature/new-awesome-feature
    ```

4.  **编写代码**：在你的工作区中编辑文件。

5.  **查看变更**：看看你具体修改了什么内容。
    ```bash
    git diff
    ```

6.  **暂存变更**：将你想要提交的文件添加到暂存区。
    ```bash
    git add <file-name>        # 添加单个文件
    git add .                  # 添加当前目录下所有变更
    git add *.rs               # 添加所有 .rs 文件
    ```

7.  **提交变更**：将暂存区的变更保存到本地仓库。
    ```bash
    git commit -m "描述本次提交的变更内容"
    ```
    - 提交信息应简洁明了，说明“做了什么”和“为什么做”。
    - 示例：
      ```bash
      git commit -m "Add user authentication module"
      git commit -m "Fix typo in README.md"
      ```

## 4. 分支管理

- **切换分支**：
  ```bash
  git checkout <branch-name>
  # 或者使用新语法 (Git 2.23+)
  git switch <branch-name>
  ```

- **查看所有分支**：
  ```bash
  git branch -a  # -a 显示本地和远程所有分支
  ```

- **合并分支**：当你完成功能开发和测试后，将功能分支合并到主分支。
  ```bash
  git checkout main  # 先切换到要合并到的目标分支
  git pull origin main  # 确保目标分支是最新
  git merge feature/new-awesome-feature  # 将功能分支合并进来
  ```

- **删除分支**（可选）：合并后可以删除已合并的功能分支。
  ```bash
  git branch -d feature/new-awesome-feature  # 删除本地分支
  git push origin --delete feature/new-awesome-feature  # 删除远程分支
  ```

## 5. 与远程仓库同步

- **推送变更**：将本地提交推送到远程仓库。
  ```bash
  git push origin <branch-name>
  ```
  - 首次推送新分支时，可能需要设置上游分支：
    ```bash
    git push -u origin feature/new-awesome-feature
    ```

- **获取远程更新**：获取远程仓库的最新信息，但不自动合并。
  ```bash
  git fetch origin
  ```

## 6. 协作与冲突解决

- **代码审查 (Code Review)**：在团队开发中，通常通过 Pull Request (PR) 或 Merge Request (MR) 来进行代码审查，然后再合并到主分支。

- **解决冲突**：当多人修改了同一部分代码并尝试合并时，可能会发生冲突。Git 会标记出冲突的部分，你需要手动编辑文件解决冲突，然后重新提交。
  ```bash
  # Git 会提示冲突文件
  # 编辑冲突文件，解决冲突标记（<<<<<<<, =======, >>>>>>>）
  git add <conflicted-file>
  git commit  # 完成合并提交
  ```

## 7. 查看历史

- **查看提交历史**：
  ```bash
  git log --oneline  # 简洁的单行显示
  git log --graph --oneline --all # 图形化显示所有分支
  ```

## 8. 回滚错误修改

- **撤销工作区修改**（未暂存）：
  ```bash
  git checkout -- <file-name>  # 撤销单个文件
  git checkout .               # 撤销所有未暂存的修改
  ```

- **撤销暂存区修改**（已 add，未 commit）：
  ```bash
  git reset HEAD <file-name>  # 取消暂存单个文件
  ```

- **回退提交**（已 commit）：
  ```bash
  git reset --hard HEAD~1  # 回退到上一个提交，并丢弃当前提交的所有变更
  git revert <commit-hash> # 创建一个新提交来撤销指定提交的变更（推荐用于已推送的提交）
  ```

## 总结

这个流程的核心思想是保持主分支（如 `main`）的稳定性，通过功能分支进行开发，定期提交并推送变更，通过合并请求或直接合并来集成代码。合理使用 Git 可以大大提高开发效率和代码质量。