/* Minimal ioctl stub for Vita cross-compilation of SQLite */
#ifndef _SYS_IOCTL_H_
#define _SYS_IOCTL_H_

#include <sys/types.h>

#define FIONREAD 1

#ifdef __cplusplus
extern "C" {
#endif

int ioctl(int fd, unsigned long request, ...);

#ifdef __cplusplus
}
#endif

#endif /* _SYS_IOCTL_H_ */
