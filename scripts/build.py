#!/usr/bin/env python3
"""
Rust项目自动化构建脚本
支持构建、测试、格式化等功能
"""

import subprocess
import sys
import os
import argparse
from typing import List

def run_command(cmd: List[str], cwd: str = None) -> bool:
    """
    运行命令并返回是否成功
    
    Args:
        cmd: 要运行的命令列表
        cwd: 工作目录
    
    Returns:
        bool: 命令是否成功执行
    """
    print(f"执行命令: {' '.join(cmd)}")
    try:
        result = subprocess.run(cmd, cwd=cwd, check=True, 
                              stdout=subprocess.PIPE, stderr=subprocess.PIPE, 
                              text=True)
        print(result.stdout)
        if result.stderr:
            print(result.stderr)
        return True
    except subprocess.CalledProcessError as e:
        print(f"命令执行失败: {e}")
        print(f"stdout: {e.stdout}")
        print(f"stderr: {e.stderr}")
        return False
    except FileNotFoundError:
        print(f"命令未找到: {cmd[0]}")
        return False

def check_rust_installed() -> bool:
    """检查是否安装了Rust"""
    return run_command(["rustc", "--version"])

def format_code() -> bool:
    """格式化代码"""
    print("正在格式化代码...")
    return run_command(["cargo", "fmt"])

def check_format() -> bool:
    """检查代码格式"""
    print("正在检查代码格式...")
    return run_command(["cargo", "fmt", "--check"])

def run_clippy() -> bool:
    """运行Clippy检查"""
    print("正在运行Clippy...")
    return run_command(["cargo", "clippy", "--all-targets", "--all-features", "--", "-D", "warnings"])

def build_project(release: bool = False) -> bool:
    """构建项目"""
    print(f"正在{'发布' if release else '开发'}构建项目...")
    cmd = ["cargo", "build"]
    if release:
        cmd.append("--release")
    return run_command(cmd)

def run_tests() -> bool:
    """运行测试"""
    print("正在运行测试...")
    return run_command(["cargo", "test"])

def run_specific_test(test_name: str) -> bool:
    """运行特定测试"""
    print(f"正在运行测试: {test_name}...")
    return run_command(["cargo", "test", test_name])

def clean_project() -> bool:
    """清理构建产物"""
    print("正在清理项目...")
    return run_command(["cargo", "clean"])

def check_project() -> bool:
    """检查项目"""
    print("正在检查项目...")
    return run_command(["cargo", "check"])

def main():
    """主函数"""
    parser = argparse.ArgumentParser(description="Rust项目自动化构建脚本")
    parser.add_argument("--format", action="store_true", help="格式化代码")
    parser.add_argument("--check-format", action="store_true", help="检查代码格式")
    parser.add_argument("--clippy", action="store_true", help="运行Clippy检查")
    parser.add_argument("--build", action="store_true", help="构建项目")
    parser.add_argument("--release", action="store_true", help="发布构建")
    parser.add_argument("--test", action="store_true", help="运行测试")
    parser.add_argument("--test-name", type=str, help="运行特定测试")
    parser.add_argument("--clean", action="store_true", help="清理构建产物")
    parser.add_argument("--check", action="store_true", help="检查项目")
    parser.add_argument("--all", action="store_true", help="执行所有操作")
    
    args = parser.parse_args()
    
    # 检查是否安装了Rust
    if not check_rust_installed():
        print("错误: 未找到Rust工具链，请先安装Rust: https://www.rust-lang.org/tools/install")
        return 1
    
    # 确定要执行的操作
    actions = []
    if args.all:
        actions = ["check", "format", "clippy", "build", "test"]
    else:
        if args.check:
            actions.append("check")
        if args.format:
            actions.append("format")
        if args.check_format:
            actions.append("check_format")
        if args.clippy:
            actions.append("clippy")
        if args.build:
            actions.append("build")
        if args.test:
            actions.append("test")
        if args.test_name:
            actions.append("specific_test")
        if args.clean:
            actions.append("clean")
    
    # 如果没有指定任何操作，显示帮助信息
    if not actions:
        parser.print_help()
        return 0
    
    # 执行操作
    success = True
    for action in actions:
        if action == "check":
            success &= check_project()
        elif action == "format":
            success &= format_code()
        elif action == "check_format":
            success &= check_format()
        elif action == "clippy":
            success &= run_clippy()
        elif action == "build":
            success &= build_project(args.release)
        elif action == "test":
            success &= run_tests()
        elif action == "specific_test":
            success &= run_specific_test(args.test_name)
        elif action == "clean":
            success &= clean_project()
        
        # 如果任何操作失败，提前退出
        if not success:
            print(f"操作 {action} 失败，停止执行后续操作")
            break
    
    if success:
        print("所有操作成功完成!")
        return 0
    else:
        print("某些操作失败!")
        return 1

if __name__ == "__main__":
    sys.exit(main())