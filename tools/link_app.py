# 链接应用程序到内核的数据段，生成link_app.S
import os
print("\nLink Application...")

# link_app.S的路径
path = '/Users/xieyuquan/os/sins_os/src/link_app.S'
# 应用程序的路径
apps_path = '/Users/xieyuquan/os/user/src/bin'
# 编译好的应用程序路径
apps_bin_path = "/Users/xieyuquan/os/user/target/riscv64gc-unknown-none-elf/release"
# 获得应用的名字
apps_bin = [app.split('.')[0] for app in os.listdir(apps_path)]
# 应用的个数
apps_num = len(apps_bin)

apps_bin.sort()
print(apps_bin)

f = open(path, 'w')

f.writelines(["\n  .align 3\n", "  .section .data\n",
             "  .global _num_app\n", "_num_app:\n"])
f.write("  .quad {}\n".format(apps_num))
f.writelines(["  .quad app_{}_start\n".format(i) for i in range(apps_num)])
f.write("  .quad app_{}_end\n".format(apps_num - 1))

for i in range(apps_num):
    app_start = "app_{}_start".format(i)
    app_end = "app_{}_end".format(i)
    bin = "{}/{}.bin".format(apps_bin_path, apps_bin[i])
    f.writelines(["\n  .section .data\n",
                  "  .global {}\n".format(app_start),
                  "  .global {}\n".format(app_end),
                  "{}:\n".format(app_start),
                  "  .incbin \"{}\"\n".format(bin),
                  "{}:\n".format(app_end)
                  ])

f.close()
print("Link Done...\n")
