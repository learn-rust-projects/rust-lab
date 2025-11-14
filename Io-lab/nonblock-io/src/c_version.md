```c
#include "apue.h"
#include <errno.h>
#include <fcntl.h>

char buf[500000];

/* 设置文件描述符标志位（如 O_NONBLOCK） */
void set_fl(int fd, int flags)
{
    int val;

    if ((val = fcntl(fd, F_GETFL, 0)) < 0)
        err_sys("fcntl F_GETFL error");

    val |= flags;  /* 设置标志位 */
    if (fcntl(fd, F_SETFL, val) < 0)
        err_sys("fcntl F_SETFL error");
}

/* 清除文件描述符标志位 */
void clr_fl(int fd, int flags)
{
    int val;

    if ((val = fcntl(fd, F_GETFL, 0)) < 0)
        err_sys("fcntl F_GETFL error");

    val &= ~flags; /* 清除指定标志位 */
    if (fcntl(fd, F_SETFL, val) < 0)
        err_sys("fcntl F_SETFL error");
}

int main(void)
{
    int ntowrite, nwrite;
    char *ptr;

    /* 从标准输入读入数据 */
    ntowrite = read(STDIN_FILENO, buf, sizeof(buf));
    fprintf(stderr, "read %d bytes\n", ntowrite);

    /* 将标准输出设为非阻塞模式 */
    set_fl(STDOUT_FILENO, O_NONBLOCK);

    ptr = buf;
    while (ntowrite > 0) {
        errno = 0;
        nwrite = write(STDOUT_FILENO, ptr, ntowrite);
        fprintf(stderr, "nwrite = %d, errno = %d\n", nwrite, errno);

        if (nwrite > 0) {
            ptr += nwrite;
            ntowrite -= nwrite;
        } else if (nwrite < 0 && errno != EAGAIN) {
            err_sys("write error");
        }
        /* 若 errno == EAGAIN 表示写缓冲区满，重试即可 */
    }

    /* 恢复标准输出为阻塞模式 */
    clr_fl(STDOUT_FILENO, O_NONBLOCK);

    exit(0);
}


```

#### 1. 功能定义

该程序用于**演示非阻塞 I/O（Nonblocking I/O）**的行为：

- 从标准输入读取一大块数据（500000 字节），
    
- 然后以非阻塞模式不断向标准输出写出。
    

#### 2. 核心机制

- **`fcntl`** 系统调用用于修改文件描述符的状态标志。
    
    - `F_GETFL`: 获取当前状态标志。
        
    - `F_SETFL`: 设置新状态标志。
        
- **`O_NONBLOCK`** 使 `write()` 在写缓冲区满时立即返回 `-1` 并设置 `errno = EAGAIN`，而不是阻塞等待。