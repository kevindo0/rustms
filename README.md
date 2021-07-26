# rustms
#### 1、rust版本
```bash
rustup toolchain list           # 默认是 Stable 版本。
rustup override set nightly     # 从 Stable 切换到 Nightly
rustup override set Stable      # 切回 Stable

rustc --pretty expanded -Z unstable-options main.rs # 代码展开
```