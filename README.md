## fosuii

更方便地在 `Termux` 环境管理字体文件，目前还在开发中

克隆此仓库

```bash
git clone https://github.com/z9erv/fosuii.git ~/fosuii
```

编译

```bash
cd ~/fosuii && cargo build --release
```

手动添加到PATH或者创建符号链接

```bash
cp ~/fosuii/target/release/fs /data/data/com.termux/files/usr/bin/fs
```

> 运行 `fs` 命令，程序会自动初始化目录和配置文件。  
> 以及目录 `~/.fonts` 使用 `-u --use, -s --swap` 不会强制使用此目录

## Conf

`fosuii` 使用 yaml 作为配置文件格式，位置在 `~/.config/fs/config.yaml`

可以参考下面的示例

```yaml
fonts:
  maple: /data/data/com.termux/files/home/.fonts/MapleMono-Regular.ttf
  iosevka: /data/data/com.termux/files/home/.fonts/IosevkaNerdFontMono-Regular.ttf
```

`fonts:` 是必须的，接着是键 `maple` 和 `iosevka`  
反序列化时会自动转换成 `HashMap<String, PathBuf>`  
这个键值对将提供给程序的 `-u, --use` 命令
